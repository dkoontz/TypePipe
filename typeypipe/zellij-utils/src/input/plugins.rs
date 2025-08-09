// Stub plugins module for Phase 6 simplification

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PluginAliases {
    pub aliases: BTreeMap<String, String>,
}

impl Default for PluginAliases {
    fn default() -> Self {
        Self {
            aliases: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum RunPluginOrAlias {
    Alias(String),
    RunPlugin(RunPlugin),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct RunPlugin {
    pub location: String,
    pub configuration: Option<BTreeMap<String, String>>,
    pub initial_cwd: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PluginUserConfiguration {
    pub configuration: BTreeMap<String, String>,
}