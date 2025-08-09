// Stub keybinds module for Phase 6 simplification

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Keybinds {
    // Minimal stub
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {}
    }
}