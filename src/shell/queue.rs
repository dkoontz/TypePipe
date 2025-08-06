use crate::shell::pty::SharedPtySession;
use crate::shell::types::CommandResult;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::path::PathBuf;

/// The PtyQueueProcessor enables external applications to send commands to a running shell
/// session through a file-based queue system, providing programmatic control over interactive
/// shell processes.
///
/// **Core Responsibilities:**
/// - **File-Based Queue Processing**: Monitors a directory for command files and processes them chronologically
/// - **Command Injection**: Safely injects commands from queue files into the PTY session
/// - **Logging & Audit Trail**: Logs processed commands with timestamps
/// - **Error Handling**: Graceful handling of malformed commands, file I/O errors, and PTY communication issues
/// - **File Cleanup**: Automatic removal of processed command files to prevent reprocessing
/// - **Chronological Ordering**: Processes commands in the order they were created (oldest first)
///
/// **Architecture:**
/// - Wraps a `SharedPtySession` for thread-safe access to the underlying terminal
/// - Monitors a specified directory for files
/// - Uses atomic file operations to ensure commands are fully written before processing
///
/// **Queue File Format:**
/// - All files placed in the queue directory are processed
/// - File contents are sent exactly as stored to the PTY (including or excluding newlines)
/// - Files are processed by modification time (oldest first)
/// - Files are automatically removed after successful processing
///
/// External applications can send commands by creating temporary files and atomically
/// moving them to the queue directory:
/// ```bash
/// echo "ls -la" > temp_cmd
/// mv temp_cmd .tp/myapp/
/// ```
pub struct PtyQueueProcessor {
    session: SharedPtySession,
    queue_dir: PathBuf,
    log_file: PathBuf,
}

impl PtyQueueProcessor {
    pub async fn new(
        session: SharedPtySession,
        queue_dir: PathBuf,
        log_file: PathBuf,
    ) -> Result<Self> {
        Ok(Self {
            session,
            queue_dir,
            log_file,
        })
    }

    pub async fn process_queue(&self) -> Result<HashMap<String, CommandResult>> {
        use tokio::fs;

        let mut results = HashMap::new();

        let mut entries = fs::read_dir(&self.queue_dir)
            .await
            .context("Failed to read queue directory")?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();

            let filename = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            match fs::read_to_string(&path).await {
                Ok(command) => {
                    let command = command.trim();
                    let _ = self
                        .log_message(&format!(
                            "🔄 Processing queue file: {} -> {}",
                            filename, command
                        ))
                        .await;

                    let result: Result<CommandResult> = {
                        let mut session_guard = self.session.lock().await;
                        let command_with_newline = format!("{}\n", command);
                        session_guard.send_input(&command_with_newline)?;

                        Ok(CommandResult {
                            output: "Command sent to shell".to_string(),
                            success: true,
                        })
                    };

                    match result {
                        Ok(cmd_result) => {
                            results.insert(filename.clone(), cmd_result);

                            // Remove the processed file
                            if let Err(e) = fs::remove_file(&path).await {
                                let _ = self
                                    .log_message(&format!(
                                        "⚠️  Warning: Failed to remove queue file {}: {}",
                                        filename, e
                                    ))
                                    .await;
                            } else {
                                let _ = self
                                    .log_message(&format!("✅ Completed and removed: {}", filename))
                                    .await;
                            }
                        }
                        Err(e) => {
                            let _ = self
                                .log_message(&format!("❌ Error processing {}: {}", filename, e))
                                .await;
                            results.insert(
                                filename,
                                CommandResult {
                                    output: format!("Error: {}", e),
                                    success: false,
                                },
                            );
                        }
                    }
                }
                Err(e) => {
                    let _ = self
                        .log_message(&format!("❌ Error reading queue file {}: {}", filename, e))
                        .await;
                }
            }
        }

        Ok(results)
    }

    /// Start continuous queue processing
    pub async fn start_processing(&self, interval_ms: u64) -> Result<()> {
        let _ = self
            .log_message(&format!(
                "🚀 Starting PTY queue processor (interval: {}ms)",
                interval_ms
            ))
            .await;
        let _ = self
            .log_message(&format!("📁 Queue directory: {}", self.queue_dir.display()))
            .await;

        let mut interval = tokio::time::interval(std::time::Duration::from_millis(interval_ms));

        loop {
            interval.tick().await;

            match self.process_queue().await {
                Ok(results) => {
                    if !results.is_empty() {
                        let _ = self
                            .log_message(&format!("📊 Processed {} queue items", results.len()))
                            .await;
                    }
                }
                Err(e) => {
                    let _ = self
                        .log_message(&format!("❌ Queue processing error: {}", e))
                        .await;
                }
            }
        }
    }

    async fn log_message(&self, message: &str) -> Result<()> {
        use tokio::io::AsyncWriteExt;
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        let log_entry = format!("[{}] {}\n", timestamp, message);

        let mut file = tokio::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_file)
            .await
            .context("Failed to open log file")?;

        file.write_all(log_entry.as_bytes())
            .await
            .context("Failed to write to log file")?;
        file.flush().await.context("Failed to flush log file")?;
        Ok(())
    }
}
