//! Tests for Script Engine Trait

use loki::script::engine::{ScriptEngine, ScriptContext};
use loki::tools::config::{ToolDefinition, ScriptSource, ParameterType, ReturnType};

#[tokio::test]
async fn test_script_engine_execute_script() {
    let engine = ScriptEngine::new();
    
    let tool = ToolDefinition {
        name: "test_script".to_string(),
        description: "Test script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("return 'Hello, World!'".to_string()),
    };
    
    let context = ScriptContext::new();
    let result = engine.execute_script(&tool, context).await;
    
    assert!(result.is_ok());
    // Note: Actual execution result depends on Lua engine
}

#[tokio::test]
async fn test_script_engine_load_script() {
    let engine = ScriptEngine::new();
    
    let tool = ToolDefinition {
        name: "test_script".to_string(),
        description: "Test script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("print('test')".to_string()),
    };
    
    let script_def = engine.load_script(&tool).await;
    assert!(script_def.is_ok());
}

#[tokio::test]
async fn test_script_engine_validate_script() {
    let engine = ScriptEngine::new();
    
    let tool = ToolDefinition {
        name: "test_script".to_string(),
        description: "Test script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("return 'valid'".to_string()),
    };
    
    let result = engine.validate_script(&tool).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_script_engine_validate_invalid_script() {
    let engine = ScriptEngine::new();
    
    let tool = ToolDefinition {
        name: "test_script".to_string(),
        description: "Test script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("invalid lua syntax {{{".to_string()),
    };
    
    let result = engine.validate_script(&tool).await;
    assert!(result.is_err());
}
