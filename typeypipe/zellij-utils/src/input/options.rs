//! Handles cli and configuration options

use clap::{ArgEnum, Args};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize, Serialize, ArgEnum)]
pub enum OnForceClose {
    #[serde(alias = "quit")]
    Quit,
    #[serde(alias = "detach")]
    Detach,
}

impl Default for OnForceClose {
    fn default() -> Self {
        Self::Detach
    }
}

impl FromStr for OnForceClose {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "quit" => Ok(Self::Quit),
            "detach" => Ok(Self::Detach),
            e => Err(e.to_string().into()),
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Deserialize, Serialize, Args)]
/// Simplified options for basic shell wrapper functionality
pub struct Options {
    /// Whether to show the status bar at the bottom of the terminal
    /// default is true
    #[clap(long, value_parser)]
    #[serde(default)]
    pub status_bar: Option<bool>,

    /// Status bar refresh interval in seconds
    /// default is 1
    #[clap(long, value_parser)]
    #[serde(default)]
    pub status_bar_refresh_interval: Option<u64>,

    /// Set behaviour on force close (quit or detach)
    #[clap(long, arg_enum, hide_possible_values = true, value_parser)]
    pub on_force_close: Option<OnForceClose>,

    #[clap(long, value_parser)]
    pub scroll_buffer_size: Option<usize>,

    // Stub fields for removed functionality - kept to avoid compilation errors
    #[serde(default)]
    pub theme_dir: Option<PathBuf>,
    
    #[serde(default)]
    pub layout_dir: Option<PathBuf>,
    
    #[serde(default)]
    pub default_layout: Option<PathBuf>,
    
    #[serde(default)]
    pub web_server_ip: Option<std::net::IpAddr>,
    
    #[serde(default)]
    pub web_server_port: Option<u16>,
    
    #[serde(default)]
    pub web_server_cert: Option<PathBuf>,
    
    #[serde(default)]
    pub web_server_key: Option<PathBuf>,
    
    #[serde(default)]
    pub enforce_https_for_localhost: Option<bool>,
    
    // Additional stub fields for removed functionality
    #[serde(default)]
    pub default_mode: Option<crate::data::InputMode>,
    
    #[serde(default)]
    pub simplified_ui: Option<bool>,
    
    #[serde(default)]
    pub pane_frames: Option<bool>,
    
    #[serde(default)]
    pub auto_layout: Option<bool>,
    
    #[serde(default)]
    pub session_serialization: Option<bool>,
    
    #[serde(default)]
    pub serialize_pane_viewport: Option<bool>,
    
    #[serde(default)]
    pub scrollback_lines_to_serialize: Option<usize>,
    
    #[serde(default)]
    pub mirror_session: Option<bool>,
    
    #[serde(default)]
    pub default_shell: Option<PathBuf>,
    
    #[serde(default)]
    pub scrollback_editor: Option<PathBuf>,
    
    #[serde(default)]
    pub copy_command: Option<String>,
    
    #[serde(default)]
    pub copy_clipboard: Option<crate::data::Clipboard>,
    
    #[serde(default)]
    pub copy_on_select: Option<bool>,
    
    #[serde(default)]
    pub styled_underlines: Option<bool>,
    
    // Additional missing fields
    #[serde(default)]
    pub support_kitty_keyboard_protocol: Option<bool>,
    
    #[serde(default)]
    pub stacked_resize: Option<bool>,
    
    #[serde(default)]
    pub web_sharing: Option<crate::data::WebSharing>,
    
    #[serde(default)]
    pub advanced_mouse_actions: Option<bool>,
    
    #[serde(default)]
    pub default_cwd: Option<PathBuf>,
    
    #[serde(default)]
    pub show_release_notes: Option<bool>,
    
    #[serde(default)]
    pub show_startup_tips: Option<bool>,
    
    // Additional missing fields needed by server
    #[serde(default)]
    pub serialization_interval: Option<u64>,
    
    #[serde(default)]
    pub disable_session_metadata: Option<bool>,
    
    #[serde(default)]
    pub post_command_discovery_hook: Option<String>,
}

impl Options {
    pub fn from_yaml(from_yaml: Option<Options>) -> Options {
        if let Some(opts) = from_yaml {
            opts
        } else {
            Options::default()
        }
    }
    
    /// Merges two [`Options`] structs, a `Some` in `other`
    /// will supersede a `Some` in `self`
    pub fn merge(&self, other: Options) -> Options {
        Options {
            status_bar: other.status_bar.or(self.status_bar),
            status_bar_refresh_interval: other.status_bar_refresh_interval.or(self.status_bar_refresh_interval),
            on_force_close: other.on_force_close.or(self.on_force_close),
            scroll_buffer_size: other.scroll_buffer_size.or(self.scroll_buffer_size),
            theme_dir: other.theme_dir.or(self.theme_dir.clone()),
            layout_dir: other.layout_dir.or(self.layout_dir.clone()),
            default_layout: other.default_layout.or(self.default_layout.clone()),
            web_server_ip: other.web_server_ip.or(self.web_server_ip),
            web_server_port: other.web_server_port.or(self.web_server_port),
            web_server_cert: other.web_server_cert.or(self.web_server_cert.clone()),
            web_server_key: other.web_server_key.or(self.web_server_key.clone()),
            enforce_https_for_localhost: other.enforce_https_for_localhost.or(self.enforce_https_for_localhost),
            default_mode: other.default_mode.or(self.default_mode),
            simplified_ui: other.simplified_ui.or(self.simplified_ui),
            pane_frames: other.pane_frames.or(self.pane_frames),
            auto_layout: other.auto_layout.or(self.auto_layout),
            session_serialization: other.session_serialization.or(self.session_serialization),
            serialize_pane_viewport: other.serialize_pane_viewport.or(self.serialize_pane_viewport),
            scrollback_lines_to_serialize: other.scrollback_lines_to_serialize.or(self.scrollback_lines_to_serialize),
            mirror_session: other.mirror_session.or(self.mirror_session),
            default_shell: other.default_shell.or(self.default_shell.clone()),
            scrollback_editor: other.scrollback_editor.or(self.scrollback_editor.clone()),
            copy_command: other.copy_command.or(self.copy_command.clone()),
            copy_clipboard: other.copy_clipboard.or(self.copy_clipboard.clone()),
            copy_on_select: other.copy_on_select.or(self.copy_on_select),
            styled_underlines: other.styled_underlines.or(self.styled_underlines),
            support_kitty_keyboard_protocol: other.support_kitty_keyboard_protocol.or(self.support_kitty_keyboard_protocol),
            stacked_resize: other.stacked_resize.or(self.stacked_resize),
            web_sharing: other.web_sharing.or(self.web_sharing.clone()),
            advanced_mouse_actions: other.advanced_mouse_actions.or(self.advanced_mouse_actions),
            default_cwd: other.default_cwd.or(self.default_cwd.clone()),
            show_release_notes: other.show_release_notes.or(self.show_release_notes),
            show_startup_tips: other.show_startup_tips.or(self.show_startup_tips),
            serialization_interval: other.serialization_interval.or(self.serialization_interval),
            disable_session_metadata: other.disable_session_metadata.or(self.disable_session_metadata),
            post_command_discovery_hook: other.post_command_discovery_hook.or(self.post_command_discovery_hook.clone()),
        }
    }

    /// Merges two [`Options`] structs,
    /// - `Some` in `other` will supersede a `Some` in `self`
    /// - `Some(bool)` in `other` will toggle a `Some(bool)` in `self`
    pub fn merge_from_cli(&self, other: Options) -> Options {
        let merge_bool = |opt_other: Option<bool>, opt_self: Option<bool>| {
            if opt_other.is_some() ^ opt_self.is_some() {
                opt_other.or(opt_self)
            } else if opt_other.is_some() && opt_self.is_some() {
                Some(opt_other.unwrap() ^ opt_self.unwrap())
            } else {
                None
            }
        };

        Options {
            status_bar: merge_bool(other.status_bar, self.status_bar),
            status_bar_refresh_interval: other.status_bar_refresh_interval.or(self.status_bar_refresh_interval),
            on_force_close: other.on_force_close.or(self.on_force_close),
            scroll_buffer_size: other.scroll_buffer_size.or(self.scroll_buffer_size),
            theme_dir: other.theme_dir.or(self.theme_dir.clone()),
            layout_dir: other.layout_dir.or(self.layout_dir.clone()),
            default_layout: other.default_layout.or(self.default_layout.clone()),
            web_server_ip: other.web_server_ip.or(self.web_server_ip),
            web_server_port: other.web_server_port.or(self.web_server_port),
            web_server_cert: other.web_server_cert.or(self.web_server_cert.clone()),
            web_server_key: other.web_server_key.or(self.web_server_key.clone()),
            enforce_https_for_localhost: other.enforce_https_for_localhost.or(self.enforce_https_for_localhost),
            default_mode: other.default_mode.or(self.default_mode),
            simplified_ui: merge_bool(other.simplified_ui, self.simplified_ui),
            pane_frames: merge_bool(other.pane_frames, self.pane_frames),
            auto_layout: merge_bool(other.auto_layout, self.auto_layout),
            session_serialization: merge_bool(other.session_serialization, self.session_serialization),
            serialize_pane_viewport: merge_bool(other.serialize_pane_viewport, self.serialize_pane_viewport),
            scrollback_lines_to_serialize: other.scrollback_lines_to_serialize.or(self.scrollback_lines_to_serialize),
            mirror_session: merge_bool(other.mirror_session, self.mirror_session),
            default_shell: other.default_shell.or(self.default_shell.clone()),
            scrollback_editor: other.scrollback_editor.or(self.scrollback_editor.clone()),
            copy_command: other.copy_command.or(self.copy_command.clone()),
            copy_clipboard: other.copy_clipboard.or(self.copy_clipboard.clone()),
            copy_on_select: merge_bool(other.copy_on_select, self.copy_on_select),
            styled_underlines: merge_bool(other.styled_underlines, self.styled_underlines),
            support_kitty_keyboard_protocol: merge_bool(other.support_kitty_keyboard_protocol, self.support_kitty_keyboard_protocol),
            stacked_resize: merge_bool(other.stacked_resize, self.stacked_resize),
            web_sharing: other.web_sharing.or(self.web_sharing.clone()),
            advanced_mouse_actions: merge_bool(other.advanced_mouse_actions, self.advanced_mouse_actions),
            default_cwd: other.default_cwd.or(self.default_cwd.clone()),
            show_release_notes: merge_bool(other.show_release_notes, self.show_release_notes),
            show_startup_tips: merge_bool(other.show_startup_tips, self.show_startup_tips),
            serialization_interval: other.serialization_interval.or(self.serialization_interval),
            disable_session_metadata: merge_bool(other.disable_session_metadata, self.disable_session_metadata),
            post_command_discovery_hook: other.post_command_discovery_hook.or(self.post_command_discovery_hook.clone()),
        }
    }

    pub fn from_cli(&self, _other: Option<()>) -> Options {
        self.to_owned()
    }
}

#[derive(Clone, Default, Debug, PartialEq, Args, Serialize, Deserialize)]
/// Options that can be set through cli flags
pub struct CliOptions {
    #[clap(flatten)]
    pub options: Options,
}

impl From<CliOptions> for Options {
    fn from(cli_options: CliOptions) -> Self {
        cli_options.options
    }
}