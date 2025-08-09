// Stub layout module for Phase 6 simplification

pub use crate::data::{Layout, SplitSize};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::ops::Not;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

impl Not for SplitDirection {
    type Output = Self;
    
    fn not(self) -> Self::Output {
        match self {
            SplitDirection::Horizontal => SplitDirection::Vertical,
            SplitDirection::Vertical => SplitDirection::Horizontal,
        }
    }
}

impl From<crate::data::Direction> for SplitDirection {
    fn from(direction: crate::data::Direction) -> Self {
        match direction {
            crate::data::Direction::Left | crate::data::Direction::Right => SplitDirection::Horizontal,
            crate::data::Direction::Up | crate::data::Direction::Down => SplitDirection::Vertical,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Run {
    Command(RunCommand),
    Plugin(RunPlugin),
    EditFile(std::path::PathBuf, Option<usize>, Option<std::path::PathBuf>),
    Cwd(std::path::PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunCommand {
    pub command: PathBuf,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub hold_on_close: bool,
    pub hold_on_start: bool,
}

impl std::fmt::Display for RunCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.command.display())
    }
}

impl From<crate::input::command::RunCommand> for RunCommand {
    fn from(cmd: crate::input::command::RunCommand) -> Self {
        RunCommand {
            command: cmd.command,
            args: cmd.args,
            cwd: cmd.cwd,
            hold_on_close: cmd.hold_on_close,
            hold_on_start: cmd.hold_on_start,
        }
    }
}

impl Default for Run {
    fn default() -> Self {
        Self::Command(RunCommand {
            command: PathBuf::from("sh"),
            args: vec![],
            cwd: None,
            hold_on_close: false,
            hold_on_start: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct RunPlugin {
    pub location: String,
    pub configuration: std::collections::BTreeMap<String, String>,
}

impl RunPlugin {
    pub fn location_string(&self) -> String {
        self.location.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum RunPluginOrAlias {
    RunPlugin(RunPlugin),
    Alias(String),
}

impl RunPluginOrAlias {
    pub fn is_equivalent_to_run(&self, _other: &Option<Run>) -> bool {
        false // Stub implementation
    }
    
    pub fn from_url(
        _url: &str,
        _config: &Option<crate::input::config::Config>,
        _cwd: Option<std::path::PathBuf>,
        _pane_title: Option<String>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Stub implementation
        Ok(RunPluginOrAlias::RunPlugin(RunPlugin {
            location: _url.to_string(),
            configuration: std::collections::BTreeMap::new(),
        }))
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FloatingPaneLayout {
    pub name: Option<String>,
    pub height: Option<SplitSize>,
    pub width: Option<SplitSize>,
    pub x: Option<SplitSize>,
    pub y: Option<SplitSize>,
    pub run: Option<Run>,
    pub focus: Option<bool>,
    pub already_running: bool,
    pub pinned: Option<bool>,
    pub logical_position: Option<usize>,
    pub pane_initial_contents: Option<String>,
}

impl Default for FloatingPaneLayout {
    fn default() -> Self {
        Self {
            name: None,
            height: None,
            width: None,
            x: None,
            y: None,
            run: None,
            focus: None,
            already_running: false,
            pinned: None,
            logical_position: None,
            pane_initial_contents: None,
        }
    }
}

impl FloatingPaneLayout {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct TiledPaneLayout {
    pub children_split_direction: SplitDirection,
    pub name: Option<String>,
    pub children: Vec<TiledPaneLayout>,
    pub split_size: Option<SplitSize>,
    pub run: Option<Run>,
    pub borderless: bool,
    pub focus: Option<bool>,
    pub external_children_index: Option<usize>,
    pub children_are_stacked: bool,
    pub is_expanded_in_stack: bool,
    pub exclude_from_sync: Option<bool>,
    pub run_instructions_to_ignore: Vec<Option<Run>>,
    pub hide_floating_panes: bool,
    pub pane_initial_contents: Option<String>,
}

impl Default for TiledPaneLayout {
    fn default() -> Self {
        Self {
            children_split_direction: SplitDirection::Horizontal,
            name: None,
            children: vec![],
            split_size: None,
            run: None,
            borderless: false,
            focus: None,
            external_children_index: None,
            children_are_stacked: false,
            is_expanded_in_stack: false,
            exclude_from_sync: None,
            run_instructions_to_ignore: vec![],
            hide_floating_panes: false,
            pane_initial_contents: None,
        }
    }
}

impl TiledPaneLayout {
    pub fn position_panes_in_space(
        &self,
        _space: &crate::pane_size::PaneGeom,
        _tiled_panes_count: Option<usize>,
        _should_add_pane: bool,
        _focus_layout_if_not_focused: bool,
    ) -> Result<Vec<(TiledPaneLayout, crate::pane_size::PaneGeom)>, Box<dyn std::error::Error + Send + Sync + 'static>> {
        // Stub implementation
        Ok(vec![])
    }
    
    pub fn pane_count(&self) -> usize {
        // Stub implementation
        1
    }
    
    pub fn extract_run_instructions(&self) -> Vec<Option<Run>> {
        // Stub implementation
        vec![]
    }
}

// Additional stub types
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SwapTiledLayout(
    pub std::collections::BTreeMap<LayoutConstraint, TiledPaneLayout>,
    pub Option<String>,
);

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SwapFloatingLayout(
    pub std::collections::BTreeMap<LayoutConstraint, Vec<FloatingPaneLayout>>,
    pub Option<String>,
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum LayoutConstraint {
    ExactPanes(usize),
    MaxPanes(usize),
    MinPanes(usize),
    NoConstraint,
}

// SplitSize is already imported above