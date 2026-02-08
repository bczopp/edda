//! Tests for Loki script registry

use loki::script_registry::ScriptRegistry;
use loki::tools::config::{ToolDefinition, ScriptSource, ParameterType, ReturnType};
use shared::models::{ScriptLanguage, ScriptDefinition};

#[tokio::test]
async fn test_script_registry_new() {
    let registry = ScriptRegistry::new();
    assert_eq!(registry.list_scripts().await.len(), 0);
}

#[tokio::test]
async fn test_script_registry_register() {
    let registry = ScriptRegistry::new();
    
    let tool = ToolDefinition {
        name: "test_script".to_string(),
        description: "Test script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("return 'test'".to_string()),
    };
    
    registry.register_tool(tool.clone()).await;
    
    assert_eq!(registry.list_scripts().await.len(), 1);
    assert!(registry.get_tool("test_script").await.is_some());
}

#[tokio::test]
async fn test_script_registry_unregister() {
    let registry = ScriptRegistry::new();
    
    let tool = ToolDefinition {
        name: "test_script".to_string(),
        description: "Test script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("return 'test'".to_string()),
    };
    
    registry.register_tool(tool.clone()).await;
    assert_eq!(registry.list_scripts().await.len(), 1);
    
    registry.unregister_tool("test_script").await;
    assert_eq!(registry.list_scripts().await.len(), 0);
}

#[tokio::test]
async fn test_script_registry_get_nonexistent() {
    let registry = ScriptRegistry::new();
    assert!(registry.get_tool("nonexistent").await.is_none());
}
