use crate::shell::types::{CommandResult, ShellConfig};
use anyhow::{Context, Result};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use std::io::{Read, Write};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

/// A PTY (Pseudo-Terminal) is a pair of virtual devices that provide a terminal interface.
///
/// PTYs consist of two parts:
/// - **Parent (historically called Master)**: The controlling side that applications use to communicate with the terminal
/// - **Child (historically called Slave)**: The side where the shell process runs, thinking it's connected to a real terminal
///
/// PTYs enable:
/// - **Terminal Emulation**: Applications can create virtual terminals for shell processes
/// - **I/O Redirection**: Capture and control all input/output to/from shell commands  
/// - **Signal Handling**: Proper delivery of terminal signals (SIGWINCH, SIGINT, etc.)
/// - **Terminal Features**: Support for colors, cursor positioning, and other terminal capabilities
pub struct PtySession {
    session_id: String,
    shell_path: String,
    cols: u16,
    rows: u16,
    pty_parent: Box<dyn MasterPty + Send>,
    pty_writer: Option<Box<dyn Write + Send>>,
    child: Box<dyn Child + Send + Sync>,
}

impl std::fmt::Debug for PtySession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PtySession")
            .field("session_id", &self.session_id)
            .field("shell_path", &self.shell_path)
            .field("cols", &self.cols)
            .field("rows", &self.rows)
            .field("pty_parent", &"<pty_parent>")
            .field("pty_writer", &"<pty_writer>")
            .field("child", &"<child>")
            .finish()
    }
}

impl PtySession {
    pub async fn new(config: ShellConfig) -> Result<Self> {
        let session_id = format!("tp-{}", &Uuid::new_v4().to_string()[..8]);

        let pty_system = native_pty_system();
        let pty_pair = pty_system
            .openpty(PtySize {
                rows: config.rows,
                cols: config.cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .context("Failed to create PTY pair")?;

        let mut cmd = CommandBuilder::new(&config.shell_path);
        cmd.env("TERM", "xterm-256color");

        let child = pty_pair
            .slave
            .spawn_command(cmd)
            .context("Failed to spawn shell in PTY")?;

        let writer = pty_pair
            .master
            .take_writer()
            .context("Failed to get PTY writer")?;

        Ok(Self {
            session_id,
            shell_path: config.shell_path,
            cols: config.cols,
            rows: config.rows,
            pty_parent: pty_pair.master,
            pty_writer: Some(writer),
            child,
        })
    }

    pub fn send_input(&mut self, input: &str) -> Result<()> {
        if let Some(writer) = &mut self.pty_writer {
            writer
                .write_all(input.as_bytes())
                .context("Failed to write input to PTY parent")?;
            writer.flush().context("Failed to flush PTY writer")?;
            Ok(())
        } else {
            Err(anyhow::anyhow!("PTY not initialized"))
        }
    }

    /// Get currently available output from PTY buffer
    pub fn get_available_output(&mut self) -> Result<String> {
        let mut buffer = [0u8; 4096];
        let mut reader = self
            .pty_parent
            .try_clone_reader()
            .context("Failed to get PTY reader")?;
        match reader
            .read(&mut buffer)
            .context("Failed to read from PTY parent")
        {
            Ok(bytes_read) => {
                let output = String::from_utf8_lossy(&buffer[..bytes_read]);
                Ok(output.to_string())
            }
            Err(_) => Ok("No output available".to_string()),
        }
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub fn is_alive(&mut self) -> bool {
        self.child.try_wait().is_ok()
    }

    pub fn resize(&mut self, rows: u16, cols: u16) -> Result<()> {
        let size = PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        };

        self.pty_parent.resize(size).context("Failed to resize PTY")
    }

    /// Take the PTY writer for external use
    pub fn take_pty_writer(&mut self) -> Option<Box<dyn Write + Send>> {
        self.pty_writer.take()
    }

    /// Get a cloned PTY reader
    pub fn clone_pty_reader(&mut self) -> Result<Box<dyn std::io::Read + Send>> {
        self.pty_parent
            .try_clone_reader()
            .context("Failed to clone PTY reader")
    }
}

impl Drop for PtySession {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

/// Shared PTY session wrapper that enables safe concurrent access across multiple async tasks.
///
/// **Why we need Arc<Mutex<PtySession>>:**
///
/// The application's architecture requires multiple components to access the same PTY session concurrently:
/// - **Queue Processor**: Injects commands from queue files into the PTY
/// - **Interactive Handler**: Processes user input and forwards it to the PTY  
/// - **Session Manager**: Manages PTY operations like resizing and status checking
/// - **Output Handler**: Reads PTY output and forwards it to the user's terminal
///
/// **Concurrency Requirements:**
/// - `Arc<T>` provides **shared ownership** - multiple tasks can hold references to the same PTY
/// - `Mutex<T>` provides **thread safety** - prevents data races when accessing mutable PTY state
/// - **Async-safe**: Uses `tokio::sync::Mutex` for proper async/await support
///
/// **Without this wrapper:** We'd have issues like:
/// - Cannot share PtySession across async task boundaries (no `Clone`)
/// - Data races when multiple tasks try to read/write PTY simultaneously  
/// - Borrow checker conflicts when multiple components need mutable access
///
/// **With this wrapper:** We get:
/// - Safe concurrent access to PTY operations (send_input, get_available_output, resize)
/// - Coordinated access prevents conflicts between user input and queue injection
/// - Clean separation between interactive and automated PTY usage
pub type SharedPtySession = Arc<Mutex<PtySession>>;

/// Create shared PTY session
pub async fn create_pty_session(config: ShellConfig) -> Result<SharedPtySession> {
    let session = PtySession::new(config).await?;
    Ok(Arc::new(Mutex::new(session)))
}

/// The PtySessionManager serves as a higher-level wrapper around the core PtySession,
/// providing async-friendly interfaces and additional functionality:
///
/// **Core Responsibilities:**
/// - **Async Interface**: Wraps synchronous PtySession methods with async/await support
/// - **Session Management**: Maintains session metadata (ID, dimensions) for external reference
/// - **Input Processing**: Provides raw input via `send_input` (callers can add newlines as needed)
/// - **Output Handling**: Offers convenient method to read PTY output as strings
/// - **Queue Command Processing**: Processes commands from external queue with proper timing and output capture
/// - **Session Lifecycle**: Handles session resizing
///
/// **Architecture:**
/// - Wraps a `SharedPtySession` (Arc<Mutex<PtySession>>) for thread-safe access
/// - Maintains cached session metadata to avoid locking the inner session for basic queries
/// - Provides async methods that properly handle mutex locking and PTY operations
///
/// **Usage Pattern:**
/// ```rust,no_run
/// let config = ShellConfig::default();
/// let manager = PtySessionManager::new(config).await?;
///
/// // Send commands
/// manager.send_input(&format!("{}\n", "ls -la")).await?;
///
/// // Read output
/// let output = manager.get_available_output().await?;
///
/// // Process queue commands with timing and output capture
/// let result = manager.process_queue_command("echo hello").await?;
/// ```
#[derive(Debug)]
pub struct PtySessionManager {
    inner_session: SharedPtySession,
    session_id: String,
}

impl PtySessionManager {
    pub async fn new(config: ShellConfig) -> Result<Self> {
        let inner_session = create_pty_session(config.clone()).await?;
        let session_id = {
            let guard = inner_session.lock().await;
            guard.session_id().to_string()
        };

        Ok(Self {
            inner_session,
            session_id,
        })
    }

    pub async fn send_input(&self, input: &str) -> Result<()> {
        let mut session_guard = self.inner_session.lock().await;
        session_guard.send_input(input)
    }

    pub async fn get_available_output(&self) -> Result<String> {
        let mut session_guard = self.inner_session.lock().await;
        session_guard.get_available_output()
    }

    pub async fn process_queue_command(&self, command: &str) -> Result<CommandResult> {
        self.send_input(&format!("{}\n", command))
            .await
            .context("Failed to send queue command to terminal")?;

        // Wait a bit for the command to process
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let output = self
            .get_available_output()
            .await
            .unwrap_or_else(|_| "Command executed".to_string());

        Ok(CommandResult {
            output,
            success: true,
        })
    }

    pub fn session_id(&self) -> &str {
        &self.session_id
    }

    pub async fn resize(&self, cols: u16, rows: u16) -> Result<()> {
        let mut session_guard = self.inner_session.lock().await;
        session_guard.resize(rows, cols)
    }
}

pub type SharedPtySessionManager = Arc<Mutex<PtySessionManager>>;

pub async fn create_pty_session_manager(config: ShellConfig) -> Result<SharedPtySessionManager> {
    let session = PtySessionManager::new(config).await?;
    Ok(Arc::new(Mutex::new(session)))
}

pub async fn pty_manager_write_line(session: &SharedPtySessionManager, text: &str) -> Result<()> {
    let session_guard = session.lock().await;
    session_guard.send_input(&format!("{}\n", text)).await
}

pub async fn pty_manager_execute_and_wait(
    session: &SharedPtySessionManager,
    command: &str,
    _timeout_ms: u64,
) -> Result<CommandResult> {
    let session_guard = session.lock().await;
    session_guard.process_queue_command(command).await
}
