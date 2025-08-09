// Minimal stub for session serialization - functionality removed for Typey Pipe
use kdl::{KdlDocument, KdlEntry, KdlNode, KdlValue};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use crate::pane_size::{Constraint, PaneGeom};

#[derive(Default, Debug, Clone)]
pub struct GlobalLayoutManifest {
    pub global_cwd: Option<PathBuf>,
    pub default_shell: Option<PathBuf>,
    pub tabs: Vec<(String, TabLayoutManifest)>,
}

#[derive(Default, Debug, Clone)]
pub struct TabLayoutManifest {
    pub tiled_panes: Vec<PaneLayoutManifest>,
    pub floating_panes: Vec<PaneLayoutManifest>,
    pub is_focused: bool,
    pub hide_floating_panes: bool,
}

#[derive(Default, Debug, Clone)]
pub struct PaneLayoutManifest {
    pub geom: PaneGeom,
    pub run: Option<String>,
    pub cwd: Option<PathBuf>,
    pub is_focused: bool,
    pub pane_name: Option<String>,
    pub pane_initial_contents: Option<String>,
    pub is_borderless: bool,
    pub exclude_from_sync: bool,
}

// Stub function - returns empty layout
pub fn serialize_session_layout(
    _global_cwd: Option<PathBuf>,
    _default_shell: Option<PathBuf>,
    _tabs: Vec<(String, TabLayoutManifest)>,
    _swap_tiled_layouts: Vec<(String, String)>,
    _swap_floating_layouts: Vec<(String, String)>,
    _focused_tab_index: usize,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok("// Session serialization disabled in Typey Pipe\n".to_string())
}

// Stub functions for missing session serialization functions
pub fn extract_command_and_args(_run: &str) -> (PathBuf, Vec<String>) {
    (PathBuf::from("sh"), vec![])
}

pub fn extract_edit_and_line_number(_run: &str) -> Option<(PathBuf, Option<usize>)> {
    None
}

pub fn extract_plugin_and_config(_run: &str) -> Option<(String, BTreeMap<String, String>)> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_empty_session() {
        let result = serialize_session_layout(None, None, vec![], vec![], vec![], 0);
        assert!(result.is_ok());
    }
}