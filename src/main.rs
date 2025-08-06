use anyhow::Result;
use clap::{Arg, Command};
use typey_pipe::shell::ShellConfig;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = Command::new("typeypipe")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Transparent shell messaging system")
        .arg(
            Arg::new("shell")
                .short('s')
                .long("shell")
                .value_name("SHELL")
                .help("Shell to use (default: /bin/bash)")
                .default_value("/bin/bash")
        )
        .arg(
            Arg::new("queue-dir")
                .short('q')
                .long("queue-dir")
                .value_name("NAME")
                .help("Queue directory name under .tp/ directory (default: process ID)")
        )
        .arg(
            Arg::new("input-timeout")
                .short('t')
                .long("input-timeout")
                .value_name("SECONDS")
                .help("Seconds to wait after user input before resuming queue processing")
                .default_value("30")
        )
        .arg(
            Arg::new("quiet")
                .short('u')
                .long("quiet")
                .help("Suppress startup messages")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Parse configuration
    let config = ShellConfig {
        shell_path: matches.get_one::<String>("shell").unwrap().clone(),
        cols: 120,
        rows: 30,
    };
    
    let input_timeout_secs: u64 = matches.get_one::<String>("input-timeout")
        .unwrap()
        .parse()
        .unwrap_or(30);

    // Create .tp directory structure
    let tp_base_dir = std::env::current_dir()?.join(".tp");
    tokio::fs::create_dir_all(&tp_base_dir).await?;
    
    // Determine queue directory name and create paths
    let queue_name = matches.get_one::<String>("queue-dir")
        .map(|s| s.as_str())
        .unwrap_or_else(|| {
            // Use process ID as default to ensure uniqueness
            Box::leak(std::process::id().to_string().into_boxed_str())
        });
    
    let queue_dir = tp_base_dir.join(queue_name);
    let log_file = tp_base_dir.join(format!("{}.log", queue_name));
    
    // Startup messages (unless quiet mode)
    if !matches.get_flag("quiet") {
        println!("🚀 Typey Pipe - Shell messaging system");
        println!("📁 Message queue: {}", queue_dir.display());
        println!();
    }

    // Clear existing log file if it exists
    if log_file.exists() {
        tokio::fs::remove_file(&log_file).await.ok(); // Ignore errors if file doesn't exist
    }
    
    // Create the log file at startup
    tokio::fs::File::create(&log_file).await?;
    
    // Clear and recreate queue directory
    if queue_dir.exists() {
        tokio::fs::remove_dir_all(&queue_dir).await.ok(); // Ignore errors if directory doesn't exist
    }
    tokio::fs::create_dir_all(&queue_dir).await?;
    
    // Create the shared PTY session
    let session = typey_pipe::shell::create_pty_session(config.clone()).await?;
    
    // Start interactive shell with integrated queue processing
    typey_pipe::shell::setup_interactive_pty(session, Some(queue_dir), Some(log_file), input_timeout_secs).await?;
    
    Ok(())
}