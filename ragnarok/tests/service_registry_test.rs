//! Tests for Service Registry (Einherjar Protocol Integration)

use ragnarok::services::ServiceRegistry;
use ragnarok::services::{ServiceInfo, ServiceStatus};

#[tokio::test]
async fn test_service_registry_register() {
    let mut registry = ServiceRegistry::new();
    
    let service = ServiceInfo {
        name: "odin".to_string(),
        address: "127.0.0.1:50051".to_string(),
        status: ServiceStatus::Available,
        capabilities: vec!["orchestration".to_string(), "chat".to_string()],
    };
    
    registry.register(service.clone()).await;
    
    let retrieved = registry.get("odin").await;
    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().name, "odin");
}

#[tokio::test]
async fn test_service_registry_list_all() {
    let mut registry = ServiceRegistry::new();
    
    let odin = ServiceInfo {
        name: "odin".to_string(),
        address: "127.0.0.1:50051".to_string(),
        status: ServiceStatus::Available,
        capabilities: vec!["orchestration".to_string()],
    };
    
    let thor = ServiceInfo {
        name: "thor".to_string(),
        address: "127.0.0.1:50052".to_string(),
        status: ServiceStatus::Available,
        capabilities: vec!["actions".to_string()],
    };
    
    registry.register(odin).await;
    registry.register(thor).await;
    
    let all = registry.list_all().await;
    assert_eq!(all.len(), 2);
}

#[tokio::test]
async fn test_service_registry_unregister() {
    let mut registry = ServiceRegistry::new();
    
    let service = ServiceInfo {
        name: "test-service".to_string(),
        address: "127.0.0.1:50053".to_string(),
        status: ServiceStatus::Available,
        capabilities: vec![],
    };
    
    registry.register(service).await;
    assert!(registry.get("test-service").await.is_some());
    
    registry.unregister("test-service").await;
    assert!(registry.get("test-service").await.is_none());
}

#[tokio::test]
async fn test_service_registry_update_status() {
    let mut registry = ServiceRegistry::new();
    
    let service = ServiceInfo {
        name: "odin".to_string(),
        address: "127.0.0.1:50051".to_string(),
        status: ServiceStatus::Available,
        capabilities: vec![],
    };
    
    registry.register(service).await;
    
    registry.update_status("odin", ServiceStatus::Unavailable).await;
    
    let retrieved = registry.get("odin").await;
    assert_eq!(retrieved.unwrap().status, ServiceStatus::Unavailable);
}

#[tokio::test]
async fn test_service_registry_find_by_capability() {
    let mut registry = ServiceRegistry::new();
    
    let odin = ServiceInfo {
        name: "odin".to_string(),
        address: "127.0.0.1:50051".to_string(),
        status: ServiceStatus::Available,
        capabilities: vec!["orchestration".to_string(), "chat".to_string()],
    };
    
    let thor = ServiceInfo {
        name: "thor".to_string(),
        address: "127.0.0.1:50052".to_string(),
        status: ServiceStatus::Available,
        capabilities: vec!["actions".to_string()],
    };
    
    registry.register(odin).await;
    registry.register(thor).await;
    
    let chat_services = registry.find_by_capability("chat").await;
    assert_eq!(chat_services.len(), 1);
    assert_eq!(chat_services[0].name, "odin");
    
    let action_services = registry.find_by_capability("actions").await;
    assert_eq!(action_services.len(), 1);
    assert_eq!(action_services[0].name, "thor");
}

#[tokio::test]
async fn test_service_registry_empty() {
    let registry = ServiceRegistry::new();
    
    assert!(registry.get("nonexistent").await.is_none());
    assert_eq!(registry.list_all().await.len(), 0);
    assert_eq!(registry.find_by_capability("any").await.len(), 0);
}
