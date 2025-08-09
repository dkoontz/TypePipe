pub mod actions;
pub mod command;
pub mod config;
pub mod keybinds;
pub mod layout;
pub mod mouse;
pub mod options;
pub mod permission;
pub mod plugins;

use crate::data::ModeInfo;
use crate::ipc::ClientAttributes;

// Stub function for get_mode_info
pub fn get_mode_info(
    _default_mode: Option<crate::data::InputMode>,
    _client_attributes: &ClientAttributes,
    _capabilities: crate::data::PluginCapabilities,
) -> ModeInfo {
    ModeInfo::default()
}

// Stub function for parse_keys
pub fn parse_keys(_keys: &str) -> Result<Vec<crate::data::BareKey>, Box<dyn std::error::Error>> {
    Ok(vec![])
}


