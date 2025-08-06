pub mod shell;

// Re-export main shell functionality for library use
pub use shell::{ShellConfig, CommandResult, PtyQueueProcessor, create_pty_session, setup_interactive_pty};

// Convenience functions for common use cases
pub mod prelude {
    pub use crate::shell::{ShellConfig, CommandResult, PtyQueueProcessor, create_pty_session, setup_interactive_pty};
}