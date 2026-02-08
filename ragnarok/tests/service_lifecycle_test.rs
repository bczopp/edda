//! Tests for Service Lifecycle Manager

use ragnarok::services::{ServiceLifecycleManager, ServiceStatus};
use std::time::Duration;

#[tokio::test]
async fn test_lifecycle_manager_health_check() {
    let mut manager = ServiceLifecycleManager::new();
    
    // Register a service that's unavailable (unreachable address)
    manager.register_service(
        "test-service".to_string(),
        "127.0.0.1:59999".to_string(), // Unreachable port
        vec![],
    ).await;
    
    // Health check should mark it as unavailable
    manager.check_health("test-service").await;
    
    let status = manager.get_status("test-service").await;
    assert_eq!(status, Some(ServiceStatus::Unavailable));
}

#[tokio::test]
async fn test_lifecycle_manager_register_service() {
    let mut manager = ServiceLifecycleManager::new();
    
    manager.register_service(
        "odin".to_string(),
        "127.0.0.1:50051".to_string(),
        vec!["orchestration".to_string()],
    ).await;
    
    let services = manager.list_services().await;
    let odin = services.iter().find(|s| s.name == "odin");
    assert!(odin.is_some(), "expected 'odin' in list_services");
    assert_eq!(odin.unwrap().name, "odin");
}

#[tokio::test]
async fn test_lifecycle_manager_unregister_service() {
    let mut manager = ServiceLifecycleManager::new();
    
    manager.register_service(
        "test".to_string(),
        "127.0.0.1:50051".to_string(),
        vec![],
    ).await;
    
    manager.unregister_service("test").await;
    
    let status = manager.get_status("test").await;
    assert!(status.is_none());
}

#[tokio::test]
async fn test_lifecycle_manager_list_services() {
    let mut manager = ServiceLifecycleManager::new();
    
    manager.register_service("odin".to_string(), "127.0.0.1:50051".to_string(), vec![]).await;
    manager.register_service("thor".to_string(), "127.0.0.1:50052".to_string(), vec![]).await;
    
    let services = manager.list_services().await;
    assert_eq!(services.len(), 2);
}

#[tokio::test]
async fn test_lifecycle_manager_start_monitoring() {
    let mut manager = ServiceLifecycleManager::new();
    
    manager.register_service(
        "test".to_string(),
        "127.0.0.1:59999".to_string(),
        vec![],
    ).await;
    
    // Start monitoring (short interval for testing)
    manager.start_monitoring(Duration::from_millis(100));
    
    // Wait for at least one health check
    tokio::time::sleep(Duration::from_millis(150)).await;
    
    // Stop monitoring
    manager.stop_monitoring();
    
    // Service should be marked as unavailable
    let status = manager.get_status("test").await;
    assert_eq!(status, Some(ServiceStatus::Unavailable));
}
