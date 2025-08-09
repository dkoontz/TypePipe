// Stub actions module for Phase 6 simplification

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum Action {
    NoOp,
    CliPipe {
        pipe_id: String,
        name: Option<String>,
        payload: Option<String>,
        args: Option<std::collections::BTreeMap<String, String>>,
        configuration: Option<std::collections::BTreeMap<String, String>>,
        launch_new: Option<bool>,
        skip_cache: Option<bool>,
        floating: Option<bool>,
        in_place: Option<bool>,
        cwd: Option<std::path::PathBuf>,
    },
}

impl Default for Action {
    fn default() -> Self {
        Action::NoOp
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum SearchDirection {
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum SearchOption {
    CaseSensitive,
    WholeWord,
    Wrap,
}