// Tests for Plugin-System (PluginManager, OdinPlugin)
// TDD: Tests first per IMPLEMENTATION_PLAN Phase 6

use odin::plugins::{PluginManager, OdinPlugin, GrpcPluginProxy, ProcessClient};
use odin::grpc::odin::{ProcessRequest, ProcessResponse};
use std::sync::Arc;
use async_trait::async_trait;

/// Minimal mock plugin for tests
struct MockPlugin {
    name: String,
    caps: Vec<String>,
}

#[async_trait]
impl OdinPlugin for MockPlugin {
    fn name(&self) -> &str {
        &self.name
    }
    fn capabilities(&self) -> Vec<String> {
        self.caps.clone()
    }
    async fn process_request(&self, request: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(format!("mock:{}", request))
    }
}

#[tokio::test]
async fn plugin_manager_new_is_empty() {
    let mgr = PluginManager::new();
    let names = mgr.list().await;
    assert!(names.is_empty(), "new manager should list no plugins");
}

#[tokio::test]
async fn plugin_manager_register_and_get() {
    let mgr = PluginManager::new();
    let p = Arc::new(MockPlugin {
        name: "test_plugin".to_string(),
        caps: vec!["cap_a".to_string()],
    });
    mgr.register(p.clone()).await;
    let got = mgr.get("test_plugin").await;
    assert!(got.is_some(), "get should return registered plugin");
    assert_eq!(got.unwrap().name(), "test_plugin");
}

#[tokio::test]
async fn plugin_manager_get_returns_none_for_unknown() {
    let mgr = PluginManager::new();
    let got = mgr.get("unknown").await;
    assert!(got.is_none(), "get unknown should return None");
}

#[tokio::test]
async fn plugin_manager_list_returns_registered_names() {
    let mgr = PluginManager::new();
    mgr.register(Arc::new(MockPlugin {
        name: "a".to_string(),
        caps: vec![],
    })).await;
    mgr.register(Arc::new(MockPlugin {
        name: "b".to_string(),
        caps: vec![],
    })).await;
    let names = mgr.list().await;
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"a".to_string()));
    assert!(names.contains(&"b".to_string()));
}

#[tokio::test]
async fn plugin_process_request_returns_ok() {
    let p = MockPlugin {
        name: "p".to_string(),
        caps: vec![],
    };
    let out = p.process_request("hello").await;
    assert!(out.is_ok());
    assert_eq!(out.unwrap(), "mock:hello");
}

#[tokio::test]
async fn plugin_manager_register_overwrites_same_name() {
    let mgr = PluginManager::new();
    mgr.register(Arc::new(MockPlugin {
        name: "x".to_string(),
        caps: vec!["old".to_string()],
    })).await;
    mgr.register(Arc::new(MockPlugin {
        name: "x".to_string(),
        caps: vec!["new".to_string()],
    })).await;
    let got = mgr.get("x").await.unwrap();
    assert_eq!(got.capabilities(), vec!["new"]);
}

// --- Plugin-Kommunikation via gRPC (Phase 6) ---

/// Mock ProcessClient for tests
struct MockProcessClient {
    response: String,
}

#[async_trait]
impl ProcessClient for MockProcessClient {
    async fn process(&self, _req: ProcessRequest) -> Result<ProcessResponse, Box<dyn std::error::Error + Send + Sync>> {
        Ok(ProcessResponse {
            response: self.response.clone(),
            actions_taken: vec![],
        })
    }
}

#[tokio::test]
async fn grpc_plugin_proxy_process_request_returns_response_from_client() {
    let client = Arc::new(MockProcessClient { response: "remote_ok".to_string() });
    let proxy = GrpcPluginProxy::new("grpc_plugin".to_string(), vec!["cap1".to_string()], client);
    let out = proxy.process_request("hi").await;
    assert!(out.is_ok());
    assert_eq!(out.unwrap(), "remote_ok");
}

#[tokio::test]
async fn grpc_plugin_proxy_name_and_capabilities_return_stored_values() {
    let client = Arc::new(MockProcessClient { response: String::new() });
    let proxy = GrpcPluginProxy::new("v".to_string(), vec!["health".to_string(), "coding".to_string()], client);
    assert_eq!(proxy.name(), "v");
    assert_eq!(proxy.capabilities(), vec!["health", "coding"]);
}

#[tokio::test]
async fn plugin_manager_register_grpc_proxy_then_get_and_process() {
    let mgr = PluginManager::new();
    let client = Arc::new(MockProcessClient { response: "from_grpc".to_string() });
    let proxy = Arc::new(GrpcPluginProxy::new("remote_p".to_string(), vec!["rpc".to_string()], client));
    mgr.register(proxy).await;
    let got = mgr.get("remote_p").await.unwrap();
    assert_eq!(got.name(), "remote_p");
    let out = got.process_request("x").await.unwrap();
    assert_eq!(out, "from_grpc");
}

// --- Einherjar Protocol Integration (gRPC für Capability-Exposure) ---

#[tokio::test]
async fn plugin_manager_register_remote_plugin_unreachable_url_returns_err() {
    let mgr = PluginManager::new();
    let res = mgr
        .register_remote_plugin("v".to_string(), "http://127.0.0.1:1", vec!["health".to_string()])
        .await;
    assert!(res.is_err(), "unreachable URL should yield error");
}
