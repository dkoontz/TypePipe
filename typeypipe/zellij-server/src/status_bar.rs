use std::env;
use std::path::PathBuf;
use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct StatusBar {
    terminal_width: usize,
    current_time: String,
    shell_info: String,
    current_directory: String,
    system_load: Option<String>,
    enabled: bool,
}

impl StatusBar {
    pub fn new(terminal_width: usize) -> Self {
        StatusBar {
            terminal_width,
            current_time: String::new(),
            shell_info: String::new(),
            current_directory: String::new(),
            system_load: None,
            enabled: true,
        }
    }

    pub fn update(&mut self, terminal_width: usize) {
        self.terminal_width = terminal_width;
        self.update_time();
        self.update_shell_info();
        self.update_current_directory();
        self.update_system_load();
    }

    pub fn render(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        let mut left_parts = vec![self.current_time.clone(), self.shell_info.clone()];
        if let Some(ref load) = self.system_load {
            left_parts.push(load.clone());
        }
        let left_section = left_parts.join(" │ ");
        let right_section = format!("📁 {}", self.current_directory);
        
        let available_width = self.terminal_width.saturating_sub(2); // Account for padding
        let left_len = left_section.len();
        let right_len = right_section.len();
        
        if left_len + right_len + 3 > available_width {
            // Truncate directory if too long
            let max_dir_len = available_width.saturating_sub(left_len + 7); // 7 for " │ 📁 ..."
            let truncated_dir = if self.current_directory.len() > max_dir_len {
                format!("...{}", &self.current_directory[self.current_directory.len().saturating_sub(max_dir_len.saturating_sub(3))..])
            } else {
                self.current_directory.clone()
            };
            let final_right = format!("📁 {}", truncated_dir);
            
            let padding = available_width.saturating_sub(left_len + final_right.len());
            format!("{}{}{}", left_section, " ".repeat(padding), final_right)
        } else {
            // Center align with padding
            let padding = available_width.saturating_sub(left_len + right_section.len());
            format!("{}{}{}", left_section, " ".repeat(padding), right_section)
        }
    }

    pub fn render_with_style(&self) -> String {
        if !self.enabled {
            return String::new();
        }

        let content = self.render();
        let padding_needed = self.terminal_width.saturating_sub(content.len());
        let padded_content = format!("{}{}", content, " ".repeat(padding_needed));
        
        // Apply background color and styling with better colors
        // Background: dark gray (236), Text: light gray (250), Separators: cyan (14)
        let styled_content = padded_content
            .replace("│", "\x1b[38;5;14m│\x1b[38;5;250m")
            .replace("📁", "\x1b[38;5;11m📁\x1b[38;5;250m");
        
        format!("\x1b[48;5;236m\x1b[38;5;250m{}\x1b[0m", styled_content)
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn update_time(&mut self) {
        let now: DateTime<Local> = Local::now();
        self.current_time = format!("🕐 {}", now.format("%H:%M"));
    }

    fn update_shell_info(&mut self) {
        let shell_name = env::var("SHELL")
            .unwrap_or_else(|_| "unknown".to_string())
            .split('/')
            .last()
            .unwrap_or("shell")
            .to_string();
        self.shell_info = format!("🐚 {}", shell_name);
    }

    fn update_current_directory(&mut self) {
        self.current_directory = env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("~"))
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("~")
            .to_string();
    }

    fn update_system_load(&mut self) {
        // Try to read system load average on Unix systems
        #[cfg(unix)]
        {
            if let Ok(load_avg) = std::fs::read_to_string("/proc/loadavg") {
                if let Some(first_load) = load_avg.split_whitespace().next() {
                    if let Ok(load_val) = first_load.parse::<f32>() {
                        self.system_load = Some(format!("⚡ {:.1}", load_val));
                        return;
                    }
                }
            }
        }
        
        // Fallback - no system load info available
        self.system_load = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_bar_creation() {
        let status_bar = StatusBar::new(80);
        assert_eq!(status_bar.terminal_width, 80);
        assert!(status_bar.is_enabled());
    }

    #[test]
    fn test_status_bar_render() {
        let mut status_bar = StatusBar::new(80);
        status_bar.update(80);
        let rendered = status_bar.render();
        assert!(!rendered.is_empty());
    }

    #[test]
    fn test_status_bar_disabled() {
        let mut status_bar = StatusBar::new(80);
        status_bar.set_enabled(false);
        let rendered = status_bar.render();
        assert!(rendered.is_empty());
    }
}