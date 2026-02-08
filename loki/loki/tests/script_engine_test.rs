//! Tests for Script Engine

use loki::script::{ScriptEngine, ScriptContext};
use shared::{ScriptDefinition, ScriptLanguage, ResourceLimits};

#[tokio::test]
async fn test_script_engine_creation() {
    let engine = ScriptEngine::new();
    assert!(engine.is_ready());
}

#[tokio::test]
async fn test_execute_simple_lua_script() {
    let engine = ScriptEngine::new();
    
    let script = ScriptDefinition::new(
        "test_script".to_string(),
        ScriptLanguage::Lua,
        "return 42".to_string(),
    );
    
    let context = ScriptContext::new();
    let result = engine.execute(&script, context).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_execute_lua_with_print() {
    let engine = ScriptEngine::new();
    
    let script = ScriptDefinition::new(
        "print_test".to_string(),
        ScriptLanguage::Lua,
        r#"print("Hello from Lua")"#.to_string(),
    );
    
    let context = ScriptContext::new();
    let result = engine.execute(&script, context).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_execute_invalid_lua_script() {
    let engine = ScriptEngine::new();
    
    let script = ScriptDefinition::new(
        "invalid_script".to_string(),
        ScriptLanguage::Lua,
        "invalid lua syntax!@#".to_string(),
    );
    
    let context = ScriptContext::new();
    let result = engine.execute(&script, context).await;
    
    assert!(result.is_err());
}

#[tokio::test]
async fn test_execute_lua_with_variables() {
    let engine = ScriptEngine::new();
    
    let script = ScriptDefinition::new(
        "var_test".to_string(),
        ScriptLanguage::Lua,
        r#"
        local x = 10
        local y = 20
        return x + y
        "#.to_string(),
    );
    
    let context = ScriptContext::new();
    let result = engine.execute(&script, context).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_execute_lua_with_context_data() {
    let engine = ScriptEngine::new();
    
    let script = ScriptDefinition::new(
        "context_test".to_string(),
        ScriptLanguage::Lua,
        "return data.value".to_string(),
    );
    
    let mut context = ScriptContext::new();
    context.set("data", serde_json::json!({"value": 100}));
    
    let result = engine.execute(&script, context).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_resource_limits_exceeded() {
    let engine = ScriptEngine::new();
    
    // Infinite loop script
    let script = ScriptDefinition::new(
        "infinite_loop".to_string(),
        ScriptLanguage::Lua,
        "while true do end".to_string(),
    ).with_limits(ResourceLimits {
        max_memory_mb: 10,
        max_execution_time_ms: 100, // 100ms timeout
        max_cpu_percent: 50,
    });
    
    let context = ScriptContext::new();
    let result = engine.execute(&script, context).await;
    
    // Should timeout or error
    assert!(result.is_err());
}

#[test]
fn test_script_context_creation() {
    let context = ScriptContext::new();
    assert!(context.get("nonexistent").is_none());
}

#[test]
fn test_script_context_set_get() {
    let mut context = ScriptContext::new();
    
    context.set("key1", serde_json::json!("value1"));
    context.set("key2", serde_json::json!(42));
    
    assert!(context.get("key1").is_some());
    assert!(context.get("key2").is_some());
    assert!(context.get("key3").is_none());
}
