//! Script Registry for managing dynamic scripts

use crate::tools::config::{ToolDefinition, ScriptSource};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use tracing::{info, warn};

/// Registry for managing tool definitions (scripts)
pub struct ScriptRegistry {
    tools: Arc<RwLock<HashMap<String, ToolDefinition>>>,
}

impl ScriptRegistry {
    pub fn new() -> Self {
        Self {
            tools: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Register a tool (script) in the registry
    pub async fn register_tool(&self, tool: ToolDefinition) {
        let mut tools = self.tools.write().await;
        let name = tool.name.clone();
        tools.insert(name.clone(), tool);
        info!("Registered tool: {}", name);
    }
    
    /// Unregister a tool from the registry
    pub async fn unregister_tool(&self, name: &str) {
        let mut tools = self.tools.write().await;
        if tools.remove(name).is_some() {
            info!("Unregistered tool: {}", name);
        } else {
            warn!("Attempted to unregister non-existent tool: {}", name);
        }
    }
    
    /// Get a tool by name
    pub async fn get_tool(&self, name: &str) -> Option<ToolDefinition> {
        let tools = self.tools.read().await;
        tools.get(name).cloned()
    }
    
    /// List all registered tool names
    pub async fn list_scripts(&self) -> Vec<String> {
        let tools = self.tools.read().await;
        tools.keys().cloned().collect()
    }
    
    /// Get script source (inline or path) for a tool
    pub async fn get_script_source(&self, name: &str) -> Option<ScriptSource> {
        let tool = self.get_tool(name).await?;
        Some(tool.script)
    }
    
    /// Check if a tool exists
    pub async fn has_tool(&self, name: &str) -> bool {
        let tools = self.tools.read().await;
        tools.contains_key(name)
    }
    
    /// Load tools from ToolConfig
    pub async fn load_from_config(&self, tools: Vec<ToolDefinition>) {
        let mut registry = self.tools.write().await;
        registry.clear();
        
        for tool in tools {
            let name = tool.name.clone();
            registry.insert(name.clone(), tool);
            info!("Loaded tool from config: {}", name);
        }
    }
}

impl Default for ScriptRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::config::{ParameterType, ReturnType};
    
    #[tokio::test]
    async fn test_script_registry_default() {
        let registry = ScriptRegistry::default();
        assert_eq!(registry.list_scripts().await.len(), 0);
    }
    
    #[tokio::test]
    async fn test_script_registry_load_from_config() {
        let registry = ScriptRegistry::new();
        
        let tools = vec![
            ToolDefinition {
                name: "tool1".to_string(),
                description: "Tool 1".to_string(),
                parameters: vec![],
                return_type: ReturnType::String,
                script: ScriptSource::inline("return 'tool1'".to_string()),
            },
            ToolDefinition {
                name: "tool2".to_string(),
                description: "Tool 2".to_string(),
                parameters: vec![],
                return_type: ReturnType::Number,
                script: ScriptSource::inline("return 42".to_string()),
            },
        ];
        
        registry.load_from_config(tools).await;
        
        assert_eq!(registry.list_scripts().await.len(), 2);
        assert!(registry.has_tool("tool1").await);
        assert!(registry.has_tool("tool2").await);
    }
}
