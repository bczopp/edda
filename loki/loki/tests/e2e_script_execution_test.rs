//! E2E tests for Script-Execution (Phase 14.1.1).
//! Tool-Config/Registry → Execute → Result; Fenrir, Jörmungandr, Hel via Coordinator.

use loki::coordination::ServiceCoordinator;
use loki::script::{ScriptContext, ScriptEngine, ScriptManager};
use loki::script_registry::ScriptRegistry;
use loki::tools::config::{ReturnType, ScriptSource, ToolDefinition};
use std::sync::Arc;

#[tokio::test(flavor = "multi_thread")]
async fn e2e_tool_registry_load_execute_result() {
    let registry = Arc::new(ScriptRegistry::new());
    let engine = Arc::new(ScriptEngine::new());
    let manager = ScriptManager::new(registry.clone(), engine).unwrap();

    registry
        .register_tool(ToolDefinition {
            name: "e2e_simple".to_string(),
            description: "E2E simple".to_string(),
            parameters: vec![],
            return_type: ReturnType::String,
            script: ScriptSource::inline("return 42".to_string()),
        })
        .await;

    let ctx = ScriptContext::new();
    let result = manager.execute_script("e2e_simple", ctx).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "42");
}

#[tokio::test(flavor = "multi_thread")]
async fn e2e_coordinator_fenrir_hardware_script() {
    let coord = ServiceCoordinator::new().unwrap();
    let script = "return tostring(fenrir:gpio_read(1))";
    let result = coord.execute_script(script).await;
    assert!(result.is_ok());
    let s = result.unwrap();
    assert!(s == "true" || s == "false");
}

#[tokio::test(flavor = "multi_thread")]
async fn e2e_coordinator_hel_storage_script() {
    let coord = ServiceCoordinator::new().unwrap();
    let script = r#"
        hel:storage_set("e2e_k", "e2e_v")
        return hel:storage_get("e2e_k")
    "#;
    let result = coord.execute_script(script).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "e2e_v");
}

#[tokio::test(flavor = "multi_thread")]
async fn e2e_coordinator_jormungandr_http_script() {
    let mut server = mockito::Server::new();
    let _m = server.mock("GET", "/").with_body("hello e2e").create();
    let url = server.url();

    let coord = ServiceCoordinator::new().unwrap();
    let script = format!("return jormungandr:http_get(\"{}\")", url);
    let result = coord.execute_script(&script).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello e2e");
}
