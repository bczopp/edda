//! Tests for Script Manager

use loki::script::manager::ScriptManager;
use loki::script::engine::{ScriptContext, ScriptEngine};
use loki::tools::config::{ToolDefinition, ScriptSource, ReturnType};
use loki::script_registry::ScriptRegistry;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_script_manager_new() {
    let registry = Arc::new(ScriptRegistry::new());
    let engine = Arc::new(ScriptEngine::new());
    
    let manager = ScriptManager::new(registry, engine);
    assert!(manager.is_ok());
}

#[tokio::test]
async fn test_script_manager_execute_script() {
    let registry = Arc::new(ScriptRegistry::new());
    let engine = Arc::new(ScriptEngine::new());
    
    let manager = ScriptManager::new(registry.clone(), engine).unwrap();
    
    let tool = ToolDefinition {
        name: "test_script".to_string(),
        description: "Test script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("return 'Hello, World!'".to_string()),
    };
    
    registry.register_tool(tool).await;
    
    let context = ScriptContext::new();
    let result = manager.execute_script("test_script", context).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_script_manager_execute_nonexistent_script() {
    let registry = Arc::new(ScriptRegistry::new());
    let engine = Arc::new(ScriptEngine::new());
    
    let manager = ScriptManager::new(registry, engine).unwrap();
    
    let context = ScriptContext::new();
    let result = manager.execute_script("nonexistent", context).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_script_manager_validate_script() {
    let registry = Arc::new(ScriptRegistry::new());
    let engine = Arc::new(ScriptEngine::new());
    
    let manager = ScriptManager::new(registry.clone(), engine).unwrap();
    
    let tool = ToolDefinition {
        name: "valid_script".to_string(),
        description: "Valid script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("return 'valid'".to_string()),
    };
    
    registry.register_tool(tool).await;
    
    let result = manager.validate_script("valid_script").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_script_manager_cache() {
    let registry = Arc::new(ScriptRegistry::new());
    let engine = Arc::new(ScriptEngine::new());
    
    let manager = ScriptManager::new(registry.clone(), engine).unwrap();
    
    let tool = ToolDefinition {
        name: "cached_script".to_string(),
        description: "Cached script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("return 'cached'".to_string()),
    };
    
    registry.register_tool(tool).await;
    
    // First execution should load and cache
    let context1 = ScriptContext::new();
    let result1 = manager.execute_script("cached_script", context1).await;
    assert!(result1.is_ok());
    
    // Second execution should use cache
    let context2 = ScriptContext::new();
    let result2 = manager.execute_script("cached_script", context2).await;
    assert!(result2.is_ok());

    let (cached, _max) = manager.cache_stats().await;
    assert_eq!(cached, 1);
}

#[tokio::test]
async fn test_script_manager_invalidate_script_on_update() {
    let registry = Arc::new(ScriptRegistry::new());
    let engine = Arc::new(ScriptEngine::new());
    let manager = ScriptManager::new(registry.clone(), engine).unwrap();

    registry
        .register_tool(ToolDefinition {
            name: "updatable".to_string(),
            description: "Script".to_string(),
            parameters: vec![],
            return_type: ReturnType::String,
            script: ScriptSource::inline("return 'v1'".to_string()),
        })
        .await;

    let ctx = ScriptContext::new();
    let r1 = manager.execute_script("updatable", ctx).await.unwrap();
    assert_eq!(r1, "v1");

    manager.invalidate_script("updatable").await;

    registry
        .register_tool(ToolDefinition {
            name: "updatable".to_string(),
            description: "Updated".to_string(),
            parameters: vec![],
            return_type: ReturnType::String,
            script: ScriptSource::inline("return 'v2'".to_string()),
        })
        .await;

    let ctx2 = ScriptContext::new();
    let r2 = manager.execute_script("updatable", ctx2).await.unwrap();
    assert_eq!(r2, "v2");
}
