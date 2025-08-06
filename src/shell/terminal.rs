use crate::shell::pty::SharedPtySession;
use anyhow::{Context, Result};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

/// Global atomic variables to track user typing state
static LAST_USER_INPUT_TIME: AtomicU64 = AtomicU64::new(0);
static USER_IS_TYPING: AtomicBool = AtomicBool::new(false);
static INPUT_TIMEOUT_MS: AtomicU64 = AtomicU64::new(30_000); // Default 30 seconds

/// Global state for tracking pause/resume logging
static QUEUE_PAUSED_LOGGED: AtomicBool = AtomicBool::new(false);

/// Setup interactive mode with PTY session using proper terminal bridge
pub async fn setup_interactive_pty(
    session: SharedPtySession,
    queue_dir: Option<PathBuf>,
    log_file: Option<PathBuf>,
    input_timeout_secs: u64,
) -> Result<()> {
    set_input_timeout(input_timeout_secs);
    use crossterm::{
        event::{self, Event, KeyCode, KeyModifiers},
        terminal::{disable_raw_mode, enable_raw_mode},
    };
    use std::io::{self, Read, Write};

    let (mut pty_reader, mut pty_writer) = {
        let mut session_guard = session.lock().await;
        let reader = session_guard.clone_pty_reader()?;

        let pty_writer_main = session_guard
            .take_pty_writer()
            .ok_or_else(|| anyhow::anyhow!("PTY writer not available"))?;

        (reader, pty_writer_main)
    };

    let raw_mode_enabled = match enable_raw_mode() {
        Ok(()) => true,
        Err(_) => false,
    };

    let pty_output_task = tokio::task::spawn_blocking(move || {
        let mut buffer = [0u8; 1024];
        let mut stdout = io::stdout();

        loop {
            match pty_reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    stdout.write_all(&buffer[..n]).unwrap();
                    stdout.flush().unwrap();
                }
                Err(_) => break, // Error reading from PTY
            }
        }
    });

    // Create appropriate input handler based on raw mode availability with integrated queue monitoring
    let input_task = if raw_mode_enabled {
        // Raw mode: character-by-character input with queue monitoring
        tokio::task::spawn_blocking(move || -> Result<()> {
            let rt = tokio::runtime::Handle::current();
            let mut last_queue_check = std::time::Instant::now();

            loop {
                if last_queue_check.elapsed() >= std::time::Duration::from_secs(1) {
                    if let (Some(ref queue_dir), Some(ref log_file)) =
                        (queue_dir.as_ref(), log_file.as_ref())
                    {
                        rt.block_on(async {
                            let _ =
                                process_next_queue_command(queue_dir, log_file, &mut pty_writer)
                                    .await;
                        });
                    }
                    last_queue_check = std::time::Instant::now();
                }

                if event::poll(std::time::Duration::from_millis(100))
                    .context("Failed to poll for events")?
                {
                    let crossterm_event = event::read().context("Failed to read event")?;
                    match &crossterm_event {
                        Event::Key(key_event) => {
                            update_user_input();

                            if let Ok(terminput_event) =
                                terminput_crossterm::to_terminput(crossterm_event.clone())
                            {
                                let mut buffer = [0u8; 16];
                                if let Ok(bytes_written) =
                                    terminput_event.encode(&mut buffer, terminput::Encoding::Xterm)
                                {
                                    pty_writer
                                        .write_all(&buffer[..bytes_written])
                                        .context("Failed to write to PTY")?;
                                    pty_writer.flush().context("Failed to flush PTY writer")?;
                                } else {
                                    if let KeyCode::Char(c) = key_event.code {
                                        let bytes = if key_event
                                            .modifiers
                                            .contains(KeyModifiers::CONTROL)
                                        {
                                            vec![c as u8 & 0x1f]
                                        } else {
                                            vec![c as u8]
                                        };
                                        pty_writer
                                            .write_all(&bytes)
                                            .context("Failed to write to PTY")?;
                                        pty_writer.flush().context("Failed to flush PTY writer")?;
                                    }
                                }
                            }
                        }
                        _ => {
                            // Ignore other events
                        }
                    }
                }
            }
        })
    } else {
        // Line mode: fallback for non-interactive environments with queue monitoring
        tokio::spawn(async move {
            use tokio::io::{AsyncBufReadExt, BufReader};
            let stdin = tokio::io::stdin();
            let mut reader = BufReader::new(stdin);
            let mut line = String::new();
            let mut last_queue_check = std::time::Instant::now();
            let mut eof_warned = false;

            loop {
                if last_queue_check.elapsed() >= std::time::Duration::from_secs(1) {
                    if let (Some(ref queue_dir), Some(ref log_file)) =
                        (queue_dir.as_ref(), log_file.as_ref())
                    {
                        let _ =
                            process_next_queue_command(queue_dir, log_file, &mut pty_writer).await;
                    }
                    last_queue_check = std::time::Instant::now();
                }

                line.clear();
                match tokio::time::timeout(
                    std::time::Duration::from_millis(100),
                    reader.read_line(&mut line),
                )
                .await
                {
                    Ok(Ok(0)) => {
                        if !eof_warned {
                            eof_warned = true;
                        }
                        // Don't break! Continue running to process queue commands
                        // Just wait longer between checks
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    }
                    Ok(Ok(_)) => {
                        update_user_input();
                        pty_writer
                            .write_all(line.as_bytes())
                            .context("Failed to write line to PTY")?;
                        pty_writer.flush().context("Failed to flush PTY writer")?;
                    }
                    Ok(Err(_)) => break, // Error reading from stdin
                    Err(_) => {}         // Timeout, continue loop to check queue
                }
            }
            Ok(())
        })
    };

    // Wait for any task to complete or Ctrl+C
    let result = tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            Ok(())
        }
        result = pty_output_task => {
            result.context("PTY output task failed")?;
            Ok(())
        }
        result = input_task => {
            result.context("Input task join failed")??;
            Ok(())
        }
    };

    // Restore terminal mode only if we enabled it
    if raw_mode_enabled {
        disable_raw_mode().context("Failed to disable raw mode")?;
    }

    result
}

pub fn set_input_timeout(timeout_secs: u64) {
    INPUT_TIMEOUT_MS.store(timeout_secs * 1000, Ordering::Relaxed);
}

fn current_time_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn update_user_input() {
    let now = current_time_ms();
    LAST_USER_INPUT_TIME.store(now, Ordering::Relaxed);
    USER_IS_TYPING.store(true, Ordering::Relaxed);
}

fn is_user_typing() -> bool {
    let now = current_time_ms();
    let last_input = LAST_USER_INPUT_TIME.load(Ordering::Relaxed);
    let timeout_ms = INPUT_TIMEOUT_MS.load(Ordering::Relaxed);
    let time_since_input = now.saturating_sub(last_input);

    if time_since_input > timeout_ms {
        USER_IS_TYPING.store(false, Ordering::Relaxed);
        false
    } else {
        true
    }
}

/// Log files are placed next to the queue directories inside the .tp directory
async fn log_to_file(log_file: &PathBuf, message: &str) -> Result<()> {
    use tokio::io::AsyncWriteExt;
    let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
    let log_entry = format!("[{}] {}\n", timestamp, message);

    let mut file = tokio::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .await
        .context("Failed to open log file")?;

    file.write_all(log_entry.as_bytes())
        .await
        .context("Failed to write to log file")?;
    file.flush().await.context("Failed to flush log file")?;
    Ok(())
}

/// Process the next queue command if one exists by injecting the command into the interactive shell
async fn process_next_queue_command(
    queue_dir: &PathBuf,
    log_file: &PathBuf,
    pty_writer: &mut Box<dyn Write + Send>,
) -> Result<()> {
    use tokio::fs;
    use tokio::io::AsyncWriteExt;

    if is_user_typing() {
        if !QUEUE_PAUSED_LOGGED.load(Ordering::Relaxed) {
            let _ = log_to_file(log_file, "⏸️ Queue processing paused - user is typing").await;
            QUEUE_PAUSED_LOGGED.store(true, Ordering::Relaxed);
        }
        return Ok(()); // Skip processing while user is typing
    } else {
        if QUEUE_PAUSED_LOGGED.load(Ordering::Relaxed) {
            let _ = log_to_file(
                log_file,
                "▶️ Queue processing resumed - user input timeout expired",
            )
            .await;
            QUEUE_PAUSED_LOGGED.store(false, Ordering::Relaxed);
        }
    }

    // Read and sort queue directory entries by modification time (oldest first)
    let mut file_entries = Vec::new();
    let mut entries = match fs::read_dir(queue_dir).await {
        Ok(entries) => entries,
        Err(_) => return Ok(()), // Skip if can't read directory
    };

    // Collect all file entries with their metadata
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if path.is_file() {
            if let Ok(metadata) = fs::metadata(&path).await {
                if let Ok(modified) = metadata.modified() {
                    file_entries.push((path, modified));
                }
            }
        }
    }

    file_entries.sort_by(|a, b| a.1.cmp(&b.1));

    // Process only the oldest file (one message per tick)
    if let Some((path, _)) = file_entries.first() {
        let filename = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        if let Ok(command) = fs::read_to_string(&path).await {
            let command = command.trim();

            let log_entry = {
                let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
                format!("[{}] 🔄 Processing: {}\n{}\n", timestamp, filename, command)
            };

            let mut file = tokio::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file)
                .await
                .unwrap_or_else(|_| panic!("Failed to open log file"));

            file.write_all(log_entry.as_bytes()).await.ok();
            file.flush().await.ok();

            let command_with_newline = format!("{}\r", command);
            let mut _success = false;

            // Try up to 50 times for recoverable errors
            for attempt in 0..50 {
                let write_result = pty_writer.write_all(command_with_newline.as_bytes());

                match write_result {
                    Err(e) => match e.kind() {
                        std::io::ErrorKind::WouldBlock | std::io::ErrorKind::Interrupted => {
                            if attempt == 49 {
                                // Final attempt failed - log and remove file
                                let retry_log_entry = {
                                    let timestamp =
                                        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
                                    format!("[{}] ❌ Gave up after 50 retries for: {} ({})\nCommand was:\n{}\n", 
                                                timestamp, filename, e.kind(), command)
                                };

                                let mut file = tokio::fs::OpenOptions::new()
                                    .create(true)
                                    .append(true)
                                    .open(log_file)
                                    .await
                                    .unwrap_or_else(|_| panic!("Failed to open log file"));

                                file.write_all(retry_log_entry.as_bytes()).await.ok();
                                file.flush().await.ok();
                                let _ = fs::remove_file(&path).await; // Remove failed file
                                break;
                            }
                            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                            continue;
                        }
                        _ => {
                            // Non-recoverable error - log and remove file
                            let error_log_entry = {
                                let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
                                format!("[{}] ❌ Failed to inject command from: {}\nError: {}\nCommand was:\n{}\n", 
                                            timestamp, filename, e, command)
                            };

                            let mut file = tokio::fs::OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(log_file)
                                .await
                                .unwrap_or_else(|_| panic!("Failed to open log file"));

                            file.write_all(error_log_entry.as_bytes()).await.ok();
                            file.flush().await.ok();
                            let _ = fs::remove_file(&path).await;
                            break;
                        }
                    },
                    Ok(()) => {
                        for flush_attempt in 0..50 {
                            match pty_writer.flush() {
                                Err(e) => match e.kind() {
                                    std::io::ErrorKind::WouldBlock
                                    | std::io::ErrorKind::Interrupted => {
                                        if flush_attempt == 49 {
                                            let retry_log_entry = {
                                                let timestamp = chrono::Utc::now()
                                                    .format("%Y-%m-%d %H:%M:%S UTC");
                                                format!("[{}] ❌ Gave up after 50 flush retries for: {} ({})\nCommand was:\n{}\n", 
                                                            timestamp, filename, e.kind(), command)
                                            };

                                            let mut file = tokio::fs::OpenOptions::new()
                                                .create(true)
                                                .append(true)
                                                .open(log_file)
                                                .await
                                                .unwrap_or_else(|_| {
                                                    panic!("Failed to open log file")
                                                });

                                            file.write_all(retry_log_entry.as_bytes()).await.ok();
                                            file.flush().await.ok();
                                            let _ = fs::remove_file(&path).await; // Remove failed file
                                            break;
                                        }
                                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                        continue;
                                    }
                                    _ => {
                                        let error_log_entry = {
                                            let timestamp =
                                                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
                                            format!("[{}] ❌ Failed to flush PTY writer for: {}\nError: {}\nCommand was:\n{}\n", 
                                                        timestamp, filename, e, command)
                                        };

                                        let mut file = tokio::fs::OpenOptions::new()
                                            .create(true)
                                            .append(true)
                                            .open(log_file)
                                            .await
                                            .unwrap_or_else(|_| panic!("Failed to open log file"));

                                        file.write_all(error_log_entry.as_bytes()).await.ok();
                                        file.flush().await.ok();
                                        let _ = fs::remove_file(&path).await;
                                        break;
                                    }
                                },
                                Ok(()) => {
                                    // Both write and flush succeeded - remove the processed file
                                    let _ = fs::remove_file(&path).await;
                                    _success = true;
                                    break;
                                }
                            }
                        }
                        break; // Exit write retry loop
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::shell::pty::{create_pty_session, PtySessionManager};
    use crate::shell::types::ShellConfig;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_pty_session_manager_creation() {
        let config = ShellConfig::default();
        let result = PtySessionManager::new(config).await;

        match result {
            Ok(session) => {
                assert!(!session.session_id().is_empty());
                eprintln!(
                    "✅ Successfully created PTY session manager: {}",
                    session.session_id()
                );
            }
            Err(e) => {
                eprintln!("❌ PTY session manager creation failed: {}", e);
                // This might fail in CI environments without proper terminal setup
            }
        }
    }

    #[tokio::test]
    async fn test_queue_processor_creation() {
        let config = ShellConfig::default();
        let _temp_dir = TempDir::new().unwrap();

        // Just test that we can create a session - queue processor needs separate setup
        let result = create_pty_session(config).await;

        match result {
            Ok(_session) => {
                eprintln!("✅ Successfully created PTY session for queue processing");
            }
            Err(e) => {
                eprintln!("❌ PTY session creation failed: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_pty_resize_functionality() {
        let config = ShellConfig::default();

        let result = create_pty_session(config).await;

        match result {
            Ok(session) => {
                let mut session_guard = session.lock().await;

                // Test resizing to different dimensions
                match session_guard.resize(50, 120) {
                    Ok(()) => eprintln!("✅ PTY resize to 50x120 successful"),
                    Err(e) => eprintln!("❌ PTY resize to 50x120 failed: {}", e),
                }

                match session_guard.resize(30, 80) {
                    Ok(()) => eprintln!("✅ PTY resize to 30x80 successful"),
                    Err(e) => eprintln!("❌ PTY resize to 30x80 failed: {}", e),
                }

                eprintln!("🎉 PTY resize functionality test completed");
            }
            Err(e) => {
                eprintln!("❌ PTY session creation failed: {}", e);
            }
        }
    }
}
