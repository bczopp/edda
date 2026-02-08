//! Tests for Loki gRPC server

use loki::grpc::server::LokiServiceImpl;
use loki::grpc::loki::*;
use loki::coordination::ServiceCoordinator;
use loki::script_registry::ScriptRegistry;
use loki::script::manager::ScriptManager;
use loki::script::engine::ScriptEngine;
use loki::tools::config_loader::ToolConfigLoader;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::Request;
use std::path::PathBuf;
use tempfile::TempDir;

fn create_test_service() -> LokiServiceImpl {
    let coordinator = Arc::new(ServiceCoordinator::new().expect("Failed to create coordinator"));
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("tools.toml");
    std::fs::write(&config_path, "").unwrap();
    let tool_config = Arc::new(RwLock::new(ToolConfigLoader::new(config_path).unwrap()));
    let script_registry = Arc::new(ScriptRegistry::new());
    let script_engine = Arc::new(ScriptEngine::new().unwrap());
    let script_manager = Arc::new(ScriptManager::new(Arc::clone(&script_registry), script_engine).unwrap());
    
    LokiServiceImpl::new(
        coordinator,
        tool_config,
        script_registry,
        script_manager,
    )
}

#[tokio::test]
async fn test_get_capabilities() {
    let service = create_test_service();
    
    let request = Request::new(GetCapabilitiesRequest {});
    let response = service.get_capabilities(request).await;
    
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    // Should return empty capabilities if no scripts registered
    assert!(response.capabilities.is_empty());
}

#[tokio::test]
async fn test_get_children_status() {
    let service = create_test_service();
    
    let request = Request::new(GetChildrenStatusRequest {});
    let response = service.get_children_status(request).await;
    
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    // Should return status for all 3 children (Fenrir, Jörmungandr, Hel)
    assert_eq!(response.children.len(), 3);
    
    let names: Vec<String> = response.children.iter().map(|c| c.name.clone()).collect();
    assert!(names.contains(&"Fenrir".to_string()));
    assert!(names.contains(&"Jörmungandr".to_string()));
    assert!(names.contains(&"Hel".to_string()));
}

#[tokio::test]
async fn test_list_scripts_empty() {
    let service = create_test_service();
    
    let request = Request::new(ListScriptsRequest {
        name_pattern: None,
    });
    let response = service.list_scripts(request).await;
    
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    // Should return empty list if no scripts registered
    assert!(response.scripts.is_empty());
}

#[tokio::test]
async fn test_list_scripts_with_pattern() {
    let service = create_test_service();
    
    // Register a test script
    use loki::tools::config::{ToolDefinition, ScriptSource, ReturnType};
    let tool = ToolDefinition {
        name: "test_script".to_string(),
        description: "Test script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("return 'test'".to_string()),
    };
    service.script_registry.register_tool(tool).await;
    
    let request = Request::new(ListScriptsRequest {
        name_pattern: Some("test".to_string()),
    });
    let response = service.list_scripts(request).await;
    
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert_eq!(response.scripts.len(), 1);
    assert_eq!(response.scripts[0].name, "test_script");
}

#[tokio::test]
async fn test_register_script_success() {
    let service = create_test_service();
    
    let request = Request::new(RegisterScriptRequest {
        name: "new_script".to_string(),
        description: "New script".to_string(),
        language: "lua".to_string(),
        parameters: vec![],
        return_type: "String".to_string(),
        inline_script: "return 'hello'".to_string(),
        script_path: String::new(),
    });
    let response = service.register_script(request).await;
    
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert!(response.success);
    assert_eq!(response.script_name, "new_script");
    assert!(response.error_message.is_empty());
    
    // Verify script is registered
    assert!(service.script_registry.has_tool("new_script").await);
}

#[tokio::test]
async fn test_register_script_duplicate() {
    let service = create_test_service();
    
    // Register script first time
    use loki::tools::config::{ToolDefinition, ScriptSource, ReturnType};
    let tool = ToolDefinition {
        name: "existing_script".to_string(),
        description: "Existing script".to_string(),
        parameters: vec![],
        return_type: ReturnType::String,
        script: ScriptSource::inline("return 'test'".to_string()),
    };
    service.script_registry.register_tool(tool).await;
    
    // Try to register again
    let request = Request::new(RegisterScriptRequest {
        name: "existing_script".to_string(),
        description: "Duplicate".to_string(),
        language: "lua".to_string(),
        parameters: vec![],
        return_type: "String".to_string(),
        inline_script: "return 'test'".to_string(),
        script_path: String::new(),
    });
    let response = service.register_script(request).await;
    
    assert!(response.is_ok());
    let response = response.unwrap().into_inner();
    assert!(!response.success);
    assert!(response.error_message.contains("already exists"));
}

#[tokio::test]
async fn test_register_script_invalid_source() {
    let service = create_test_service();
    
    // Try to register with both inline_script and script_path empty
    let request = Request::new(RegisterScriptRequest {
        name: "invalid_script".to_string(),
        description: "Invalid".to_string(),
        language: "lua".to_string(),
        parameters: vec![],
        return_type: "String".to_string(),
        inline_script: String::new(),
        script_path: String::new(),
    });
    let response = service.register_script(request).await;
    
    assert!(response.is_err() || {
        let resp = response.unwrap().into_inner();
        !resp.success && resp.error_message.contains("Must specify")
    });
}

#[tokio::test]
async fn test_execute_script() {
    let service = create_test_service();
    
    let request = Request::new(ExecuteScriptRequest {
        script_id: "test_script".to_string(),
        script_content: "return 'hello'".to_string(),
        script_type: "lua".to_string(),
        parameters: std::collections::HashMap::new(),
    });
    let response = service.execute_script(request).await;
    
    // Should handle execution (may succeed or fail depending on coordinator implementation)
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_stream_script_execution_not_found() {
    let service = create_test_service();
    
    let request = Request::new(StreamScriptExecutionRequest {
        script_name: "non_existent_script".to_string(),
        input: Some(ScriptInput {
            parameters: std::collections::HashMap::new(),
        }),
    });
    let response = service.stream_script_execution(request).await;
    
    assert!(response.is_ok());
    let mut stream = response.unwrap().into_inner();
    
    // Should receive error chunk
    let chunk = stream.message().await;
    assert!(chunk.is_ok());
    let chunk = chunk.unwrap();
    assert!(chunk.is_some());
    let chunk = chunk.unwrap();
    
    if let Some(loki::script_chunk::ChunkType::Error(err)) = chunk.chunk_type {
        assert!(err.error_message.contains("not found"));
        assert_eq!(err.error_type, "script_not_found");
    } else {
        panic!("Expected error chunk");
    }
}
