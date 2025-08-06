use serde::{Deserialize, Serialize};

/// Configuration for shell creation
#[derive(Debug, Clone)]
pub struct ShellConfig {
    pub shell_path: String,
    pub cols: u16,
    pub rows: u16,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            shell_path: std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string()),
            cols: 80,
            rows: 24,
        }
    }
}

/// Command execution result
#[derive(Debug, Serialize, Deserialize)]
pub struct CommandResult {
    pub output: String,
    pub success: bool,
}