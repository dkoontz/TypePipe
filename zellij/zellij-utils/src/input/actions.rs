//! Definition of the actions that can be bound to keys.

use super::command::{OpenFilePayload, RunCommandAction};
use super::layout::{
    FloatingPaneLayout, RunPluginOrAlias,
    SwapFloatingLayout, SwapTiledLayout, TiledPaneLayout,
};

use crate::data::{Direction, KeyWithModifier, PaneId, Resize};
use crate::data::{FloatingPaneCoordinates, InputMode};

use crate::input::config::Config;
use crate::input::mouse::MouseEvent;
use crate::input::options::OnForceClose;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;


use std::path::PathBuf;
use std::str::FromStr;

use crate::position::Position;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ResizeDirection {
    Left,
    Right,
    Up,
    Down,
    Increase,
    Decrease,
}
impl FromStr for ResizeDirection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Left" | "left" => Ok(ResizeDirection::Left),
            "Right" | "right" => Ok(ResizeDirection::Right),
            "Up" | "up" => Ok(ResizeDirection::Up),
            "Down" | "down" => Ok(ResizeDirection::Down),
            "Increase" | "increase" | "+" => Ok(ResizeDirection::Increase),
            "Decrease" | "decrease" | "-" => Ok(ResizeDirection::Decrease),
            _ => Err(format!(
                "Failed to parse ResizeDirection. Unknown ResizeDirection: {}",
                s
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SearchDirection {
    Down,
    Up,
}

impl FromStr for SearchDirection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Down" | "down" => Ok(SearchDirection::Down),
            "Up" | "up" => Ok(SearchDirection::Up),
            _ => Err(format!(
                "Failed to parse SearchDirection. Unknown SearchDirection: {}",
                s
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SearchOption {
    CaseSensitivity,
    WholeWord,
    Wrap,
}

impl FromStr for SearchOption {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CaseSensitivity" | "casesensitivity" | "Casesensitivity" => {
                Ok(SearchOption::CaseSensitivity)
            },
            "WholeWord" | "wholeword" | "Wholeword" => Ok(SearchOption::WholeWord),
            "Wrap" | "wrap" => Ok(SearchOption::Wrap),
            _ => Err(format!(
                "Failed to parse SearchOption. Unknown SearchOption: {}",
                s
            )),
        }
    }
}

// As these actions are bound to the default config, please
// do take care when refactoring - or renaming.
// They might need to be adjusted in the default config
// as well `../../assets/config/default.yaml`
/// Actions that can be bound to keys.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Action {
    /// Quit Zellij.
    Quit,
    /// Write to the terminal.
    Write(Option<KeyWithModifier>, Vec<u8>, bool), // bool -> is_kitty_keyboard_protocol
    /// Write Characters to the terminal.
    WriteChars(String),
    /// Switch to the specified input mode.
    SwitchToMode(InputMode),
    /// Switch all connected clients to the specified input mode.
    SwitchModeForAllClients(InputMode),
    /// Shrink/enlarge focused pane at specified border
    Resize(Resize, Option<Direction>),
    /// Switch focus to next pane in specified direction.
    FocusNextPane,
    FocusPreviousPane,
    /// Move the focus pane in specified direction.
    SwitchFocus,
    MoveFocus(Direction),
    /// Tries to move the focus pane in specified direction.
    /// If there is no pane in the direction, move to previous/next Tab.
    MoveFocusOrTab(Direction),
    MovePane(Option<Direction>),
    MovePaneBackwards,
    /// Clear all buffers of a current screen
    ClearScreen,
    /// Dumps the screen to a file
    DumpScreen(String, bool),
    /// Dumps
    DumpLayout,
    /// Scroll up in focus pane.
    EditScrollback,
    ScrollUp,
    /// Scroll up at point
    ScrollUpAt(Position),
    /// Scroll down in focus pane.
    ScrollDown,
    /// Scroll down at point
    ScrollDownAt(Position),
    /// Scroll down to bottom in focus pane.
    ScrollToBottom,
    /// Scroll up to top in focus pane.
    ScrollToTop,
    /// Scroll up one page in focus pane.
    PageScrollUp,
    /// Scroll down one page in focus pane.
    PageScrollDown,
    /// Scroll up half page in focus pane.
    HalfPageScrollUp,
    /// Scroll down half page in focus pane.
    HalfPageScrollDown,
    /// Toggle between fullscreen focus pane and normal layout.
    ToggleFocusFullscreen,
    /// Toggle frames around panes in the UI
    TogglePaneFrames,
    /// Toggle between sending text commands to all panes on the current tab and normal mode.
    ToggleActiveSyncTab,
    /// Open a new pane in the specified direction (relative to focus).
    /// If no direction is specified, will try to use the biggest available space.
    NewPane(Option<Direction>, Option<String>, bool), // String is an optional pane name
    /// Open the file in a new pane using the default editor, bool -> start suppressed
    EditFile(
        OpenFilePayload,
        Option<Direction>,
        bool,
        bool,
        bool,
        Option<FloatingPaneCoordinates>,
    ), // bool is floating true/false, second bool is in_place
    // third bool is start_suppressed
    /// Open a new floating pane
    NewFloatingPane(
        Option<RunCommandAction>,
        Option<String>,
        Option<FloatingPaneCoordinates>,
    ), // String is an optional pane name
    /// Open a new tiled (embedded, non-floating) pane
    NewTiledPane(Option<Direction>, Option<RunCommandAction>, Option<String>), // String is an
    /// Open a new pane in place of the focused one, suppressing it instead
    NewInPlacePane(Option<RunCommandAction>, Option<String>), // String is an
    // optional pane
    NewStackedPane(Option<RunCommandAction>, Option<String>), // String is an
    // optional pane
    // name
    /// Embed focused pane in tab if floating or float focused pane if embedded
    TogglePaneEmbedOrFloating,
    /// Toggle the visibility of all floating panes (if any) in the current Tab
    ToggleFloatingPanes,
    /// Close the focus pane.
    CloseFocus,
    PaneNameInput(Vec<u8>),
    UndoRenamePane,
    /// Create a new tab, optionally with a specified tab layout.
    NewTab(
        Option<TiledPaneLayout>,
        Vec<FloatingPaneLayout>,
        Option<Vec<SwapTiledLayout>>,
        Option<Vec<SwapFloatingLayout>>,
        Option<String>,
        bool,            // should_change_focus_to_new_tab
        Option<PathBuf>, // cwd
    ), // the String is the tab name
    /// Do nothing.
    NoOp,
    /// Go to the next tab.
    GoToNextTab,
    /// Go to the previous tab.
    GoToPreviousTab,
    /// Close the current tab.
    CloseTab,
    GoToTab(u32),
    GoToTabName(String, bool),
    ToggleTab,
    TabNameInput(Vec<u8>),
    UndoRenameTab,
    MoveTab(Direction),
    /// Run specified command in new pane.
    Run(RunCommandAction),
    /// Detach session and exit
    Detach,
    LaunchOrFocusPlugin(RunPluginOrAlias, bool, bool, bool, bool), // bools => should float,
    // move_to_focused_tab, should_open_in_place, skip_cache
    LaunchPlugin(RunPluginOrAlias, bool, bool, bool, Option<PathBuf>), // bools => should float,
    // should_open_in_place, skip_cache, Option<PathBuf> is cwd
    MouseEvent(MouseEvent),
    Copy,
    /// Confirm a prompt
    Confirm,
    /// Deny a prompt
    Deny,
    /// Confirm an action that invokes a prompt automatically
    SkipConfirm(Box<Action>),
    /// Search for String
    SearchInput(Vec<u8>),
    /// Search for something
    Search(SearchDirection),
    /// Toggle case sensitivity of search
    SearchToggleOption(SearchOption),
    ToggleMouseMode,
    PreviousSwapLayout,
    NextSwapLayout,
    /// Query all tab names
    QueryTabNames,
    /// Open a new tiled (embedded, non-floating) plugin pane
    NewTiledPluginPane(RunPluginOrAlias, Option<String>, bool, Option<PathBuf>), // String is an optional name, bool is
    // skip_cache, Option<PathBuf> is cwd
    NewFloatingPluginPane(
        RunPluginOrAlias,
        Option<String>,
        bool,
        Option<PathBuf>,
        Option<FloatingPaneCoordinates>,
    ), // String is an optional name, bool is
    // skip_cache, Option<PathBuf> is cwd
    NewInPlacePluginPane(RunPluginOrAlias, Option<String>, bool), // String is an optional name, bool is
    // skip_cache
    StartOrReloadPlugin(RunPluginOrAlias),
    CloseTerminalPane(u32),
    ClosePluginPane(u32),
    FocusTerminalPaneWithId(u32, bool), // bool is should_float_if_hidden
    FocusPluginPaneWithId(u32, bool),   // bool is should_float_if_hidden
    RenameTerminalPane(u32, Vec<u8>),
    RenamePluginPane(u32, Vec<u8>),
    RenameTab(u32, Vec<u8>),
    BreakPane,
    BreakPaneRight,
    BreakPaneLeft,
    RenameSession(String),
    CliPipe {
        pipe_id: String,
        name: Option<String>,
        payload: Option<String>,
        args: Option<BTreeMap<String, String>>,
        plugin: Option<String>,
        configuration: Option<BTreeMap<String, String>>,
        launch_new: bool,
        skip_cache: bool,
        floating: Option<bool>,
        in_place: Option<bool>,
        cwd: Option<PathBuf>,
        pane_title: Option<String>,
    },
    KeybindPipe {
        name: Option<String>,
        payload: Option<String>,
        args: Option<BTreeMap<String, String>>,
        plugin: Option<String>,
        plugin_id: Option<u32>, // supercedes plugin if present
        configuration: Option<BTreeMap<String, String>>,
        launch_new: bool,
        skip_cache: bool,
        floating: Option<bool>,
        in_place: Option<bool>,
        cwd: Option<PathBuf>,
        pane_title: Option<String>,
    },
    ListClients,
    TogglePanePinned,
    StackPanes(Vec<PaneId>),
    ChangeFloatingPaneCoordinates(PaneId, FloatingPaneCoordinates),
    TogglePaneInGroup,
    ToggleGroupMarking,
}

impl Action {
    /// Checks that two Action are match except their mutable attributes.
    pub fn shallow_eq(&self, other_action: &Action) -> bool {
        match (self, other_action) {
            (Action::NewTab(..), Action::NewTab(..)) => true,
            (Action::LaunchOrFocusPlugin(..), Action::LaunchOrFocusPlugin(..)) => true,
            (Action::LaunchPlugin(..), Action::LaunchPlugin(..)) => true,
            _ => self == other_action,
        }
    }

    pub fn actions_from_cli(
        _cli_action: (),
        _get_current_dir: Box<dyn Fn() -> PathBuf>,
        _config: Option<Config>,
    ) -> Result<Vec<Action>, String> {
        Err("CliAction functionality removed in shell wrapper mode".to_string())
    }
    pub fn launches_plugin(&self, plugin_url: &str) -> bool {
        match self {
            Action::LaunchPlugin(run_plugin_or_alias, ..) => {
                &run_plugin_or_alias.location_string() == plugin_url
            },
            Action::LaunchOrFocusPlugin(run_plugin_or_alias, ..) => {
                &run_plugin_or_alias.location_string() == plugin_url
            },
            _ => false,
        }
    }
    pub fn is_mouse_action(&self) -> bool {
        if let Action::MouseEvent(_mouse_event) = self {
            return true;
        }
        false
    }
}

impl From<OnForceClose> for Action {
    fn from(ofc: OnForceClose) -> Action {
        match ofc {
            OnForceClose::Quit => Action::Quit,
            OnForceClose::Detach => Action::Detach,
        }
    }
}
