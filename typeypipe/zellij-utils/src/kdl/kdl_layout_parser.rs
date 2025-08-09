// Minimal stub for KDL layout parser - functionality removed for Typey Pipe
use crate::data::PluginUserConfiguration;
use kdl::*;
use std::collections::BTreeMap;

pub struct KdlLayoutParser;

impl KdlLayoutParser {
    pub fn parse_plugin_user_configuration(_kdl_node: &KdlNode) -> Result<PluginUserConfiguration, Box<dyn std::error::Error>> {
        Ok(PluginUserConfiguration::new(BTreeMap::new()))
    }
    
    pub fn is_a_reserved_plugin_property(_name: &str) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_plugin_user_configuration() {
        let kdl_node = KdlNode::new("test");
        let result = KdlLayoutParser::parse_plugin_user_configuration(&kdl_node);
        assert!(result.is_ok());
    }
}