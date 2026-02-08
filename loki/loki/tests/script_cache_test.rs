//! Tests for Script-Cache (Phase 11.1.1): cache hit and invalidation.

use loki::script::{ScriptContext, ScriptEngine, ScriptManager};
use loki::tools::config::{ReturnType, ScriptSource, ToolDefinition};
use loki::script_registry::ScriptRegistry;
use std::sync::Arc;

#[tokio::test]
async fn script_cache_compiled_script_caching() {
    let registry = Arc::new(ScriptRegistry::new());
    let engine = Arc::new(ScriptEngine::new());
    let manager = ScriptManager::new(registry.clone(), engine).unwrap();

    registry
        .register_tool(ToolDefinition {
            name: "cached".to_string(),
            description: "Cached".to_string(),
            parameters: vec![],
            return_type: ReturnType::String,
            script: ScriptSource::inline("return 1 + 1".to_string()),
        })
        .await;

    let (before, _) = manager.cache_stats().await;
    assert_eq!(before, 0);

    let _ = manager
        .execute_script("cached", ScriptContext::new())
        .await
        .unwrap();
    let (after_first, _) = manager.cache_stats().await;
    assert_eq!(after_first, 1);

    let _ = manager
        .execute_script("cached", ScriptContext::new())
        .await
        .unwrap();
    let (after_second, _) = manager.cache_stats().await;
    assert_eq!(after_second, 1);
}

#[tokio::test]
async fn script_cache_invalidate_all() {
    let registry = Arc::new(ScriptRegistry::new());
    let engine = Arc::new(ScriptEngine::new());
    let manager = ScriptManager::new(registry.clone(), engine).unwrap();

    registry
        .register_tool(ToolDefinition {
            name: "a".to_string(),
            description: "".to_string(),
            parameters: vec![],
            return_type: ReturnType::String,
            script: ScriptSource::inline("return 'a'".to_string()),
        })
        .await;
    let _ = manager.execute_script("a", ScriptContext::new()).await.unwrap();
    manager.clear_cache().await;
    let (n, _) = manager.cache_stats().await;
    assert_eq!(n, 0);
}
