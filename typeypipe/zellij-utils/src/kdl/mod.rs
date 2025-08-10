mod kdl_layout_parser;



use crate::input::config::{ConfigError};
use crate::data::{Action, PluginAliases, WebClientConfig};
use crate::input::options::{Options};








use kdl::{KdlNode};






impl Action {
    pub fn new_from_bytes(
        action_name: &str,
        bytes: Vec<u8>,
        action_node: &KdlNode,
    ) -> Result<Self, ConfigError> {
        match action_name {
            "Write" => Ok(Action::Write(None, bytes, false)),
            "PaneNameInput" => Ok(Action::PaneNameInput(bytes)),
            "TabNameInput" => Ok(Action::TabNameInput(bytes)),
            "SearchInput" => Ok(Action::SearchInput(bytes)),
            _ => Err(ConfigError::new_kdl_error(
                format!("Unsupported action with bytes: {}", action_name),
                action_node.span().offset(),
                action_node.span().len(),
            )),
        }
    }

    pub fn new_from_string(
        action_name: &str,
        string: String,
        action_node: &KdlNode,
    ) -> Result<Self, ConfigError> {
        match action_name {
            "Write" => Ok(Action::Write(Some(string), vec![], false)),
            "GoToTab" => {
                let tab_index = string.parse::<u32>().map_err(|_| {
                    ConfigError::new_kdl_error(
                        format!("Invalid tab index: {}", string),
                        action_node.span().offset(),
                        action_node.span().len(),
                    )
                })?;
                Ok(Action::GoToTab(tab_index))
            }
            _ => Err(ConfigError::new_kdl_error(
                format!("Unsupported action with string: {}", action_name),
                action_node.span().offset(),
                action_node.span().len(),
            )),
        }
    }

    pub fn to_kdl(&self) -> Option<KdlNode> {
        match self {
            Action::NoOp => {
                Some(KdlNode::new("NoOp"))
            }
            Action::Write(text, bytes, _) => {
                let mut node = KdlNode::new("Write");
                if let Some(text) = text {
                    node.push(text.clone());
                } else if !bytes.is_empty() {
                    // Convert bytes to string representation
                    node.push(format!("{:?}", bytes));
                }
                Some(node)
            }
            Action::PaneNameInput(bytes) => {
                let mut node = KdlNode::new("PaneNameInput");
                node.push(format!("{:?}", bytes));
                Some(node)
            }
            Action::TabNameInput(bytes) => {
                let mut node = KdlNode::new("TabNameInput");
                node.push(format!("{:?}", bytes));
                Some(node)
            }
            Action::SearchInput(bytes) => {
                let mut node = KdlNode::new("SearchInput");
                node.push(format!("{:?}", bytes));
                Some(node)
            }
            Action::GoToTab(tab_index) => {
                let mut node = KdlNode::new("GoToTab");
                node.push(*tab_index as i64);
                Some(node)
            }
            Action::WriteChars(chars) => {
                let mut node = KdlNode::new("WriteChars");
                node.push(chars.clone());
                Some(node)
            }
            Action::SwitchToMode(_mode) => {
                Some(KdlNode::new("SwitchToMode"))
            }
            Action::Resize(_resize, _direction) => {
                Some(KdlNode::new("Resize"))
            }
            Action::SwitchFocus => {
                Some(KdlNode::new("SwitchFocus"))
            }
            Action::ToggleTab => {
                Some(KdlNode::new("ToggleTab"))
            }
            Action::MoveFocus(_direction) => {
                Some(KdlNode::new("MoveFocus"))
            }
            Action::MoveFocusOrTab(_direction) => {
                Some(KdlNode::new("MoveFocusOrTab"))
            }
            Action::MovePane(_direction) => {
                Some(KdlNode::new("MovePane"))
            }
            Action::NewPane(_direction, _name) => {
                Some(KdlNode::new("NewPane"))
            }
            Action::NewTab(_cwd, _name) => {
                Some(KdlNode::new("NewTab"))
            }
            Action::CloseTab => {
                Some(KdlNode::new("CloseTab"))
            }
            Action::CloseFocus => {
                Some(KdlNode::new("CloseFocus"))
            }
            Action::Quit => {
                Some(KdlNode::new("Quit"))
            }
            Action::MouseEvent(_event) => {
                Some(KdlNode::new("MouseEvent"))
            }
            Action::CliPipe { pipe_id, .. } => {
                let mut node = KdlNode::new("CliPipe");
                node.push(pipe_id.clone());
                Some(node)
            }
            // Stub implementations for all other Action variants
            _ => {
                // For Phase 6 simplification, we don't need to serialize most actions to KDL
                None
            }
        }
    }
}

// Layout and Config implementations are in their respective files to avoid duplicates

// Simplified stubs for removed functionality
impl PluginAliases {
    pub fn from_kdl(_kdl_plugin_aliases: &KdlNode) -> Result<Self, ConfigError> {
        Ok(PluginAliases::default())
    }
}

impl WebClientConfig {
    pub fn from_kdl(_kdl_web_client_config: &KdlNode) -> Result<Self, ConfigError> {
        Ok(WebClientConfig::default())
    }
}

impl Options {
    pub fn from_kdl(_kdl_options: &KdlNode) -> Result<Self, ConfigError> {
        Ok(Options::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_config() {
        let config = Config::from_kdl("", None).unwrap();
        assert_eq!(config.options, Options::default());
    }

    #[test]
    fn test_action_serialization() {
        let action = Action::NoOp;
        let kdl_node = action.to_kdl().unwrap();
        assert_eq!(kdl_node.name().value(), "NoOp");
    }
}