// Stub for removed plugin functionality
use std::collections::BTreeMap;
use crate::input::layout::RunPlugin;
use crate::input::config::ConfigError;
use url::Url;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PluginAliases {
    pub aliases: BTreeMap<String, RunPlugin>,
}

impl PluginAliases {
    pub fn merge(&mut self, other: PluginAliases) {
        // Plugin functionality removed - merge is a no-op
        self.aliases.extend(other.aliases);
    }

    pub fn from_data(data: BTreeMap<String, RunPlugin>) -> Self {
        PluginAliases { aliases: data }
    }
}

// Stub error type for plugin configuration
#[derive(Debug, Clone)]
pub enum PluginsConfigError {
    InvalidUrlScheme(String),
    InvalidUrl(String),
    Generic(String),
}

impl std::fmt::Display for PluginsConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginsConfigError::InvalidUrlScheme(msg) => write!(f, "Invalid URL scheme: {}", msg),
            PluginsConfigError::InvalidUrl(msg) => write!(f, "Invalid URL: {}", msg),
            PluginsConfigError::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for PluginsConfigError {}

impl From<ConfigError> for PluginsConfigError {
    fn from(err: ConfigError) -> Self {
        PluginsConfigError::Generic(format!("{:?}", err))
    }
}

impl From<url::ParseError> for PluginsConfigError {
    fn from(err: url::ParseError) -> Self {
        PluginsConfigError::InvalidUrl(format!("{:?}", err))
    }
}

// Stub for plugin tags
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Hash)]
pub enum PluginTag {
    // Plugin functionality removed - stub variant
    Stub(String),
}

impl PluginTag {
    pub fn new(name: &str) -> Self {
        PluginTag::Stub(name.to_string())
    }
}

impl std::fmt::Display for PluginTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginTag::Stub(name) => write!(f, "{}", name),
        }
    }
}