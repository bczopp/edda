//! Tests for Thjalfi (Service Loader)

use gladsheim::thjalfi::{Thjalfi, ServiceConfig};
use std::time::Duration;

#[tokio::test]
async fn test_thjalfi_creation() {
    let thjalfi = Thjalfi::new();
    assert!(thjalfi.is_ok());
}

#[tokio::test]
async fn test_start_process() {
    let thjalfi = Thjalfi::new().unwrap();
    
    // Start a simple command (echo)
    #[cfg(unix)]
    let result = thjalfi.start_process("echo", &["test"], None, None).await;
    #[cfg(windows)]
    let result = thjalfi.start_process("cmd", &["/C", "echo", "test"], None, None).await;
    
    assert!(result.is_ok());
    
    let process = result.unwrap();
    assert!(process.process_id().is_some());
}

#[tokio::test]
async fn test_stop_process_graceful() {
    let thjalfi = Thjalfi::new().unwrap();
    
    // Start a process that will run for a bit
    #[cfg(unix)]
    let process = thjalfi.start_process("sleep", &["1"], None, None).await.unwrap();
    #[cfg(windows)]
    let process = thjalfi.start_process("timeout", &["/t", "1"], None, None).await.unwrap();
    
    // Stop gracefully
    let result = thjalfi.stop_process(&process, false, Some(Duration::from_secs(2))).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stop_process_force() {
    let thjalfi = Thjalfi::new().unwrap();
    
    // Start a process
    #[cfg(unix)]
    let process = thjalfi.start_process("sleep", &["10"], None, None).await.unwrap();
    #[cfg(windows)]
    let process = thjalfi.start_process("timeout", &["/t", "10"], None, None).await.unwrap();
    
    // Force stop
    let result = thjalfi.stop_process(&process, true, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_process_status() {
    let thjalfi = Thjalfi::new().unwrap();
    
    #[cfg(unix)]
    let process = thjalfi.start_process("sleep", &["1"], None, None).await.unwrap();
    #[cfg(windows)]
    let process = thjalfi.start_process("timeout", &["/t", "1"], None, None).await.unwrap();
    
    let status = process.status().await;
    assert!(status.is_running() || status.is_finished());
}

#[tokio::test]
async fn test_start_service() {
    let thjalfi = Thjalfi::new().unwrap();
    
    let config = gladsheim::thjalfi::ServiceConfig {
        name: "test-service".to_string(),
        command: "echo".to_string(),
        args: vec!["test".to_string()],
        working_directory: None,
        environment_vars: std::collections::HashMap::new(),
    };
    
    let result = thjalfi.start_service(config.clone(), Duration::from_secs(5)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_service_timeout() {
    let thjalfi = Thjalfi::new().unwrap();
    
    // Start a service that will timeout
    let config = ServiceConfig {
        name: "timeout-service".to_string(),
        command: "nonexistent-command".to_string(),
        args: vec![],
        working_directory: None,
        environment_vars: std::collections::HashMap::new(),
    };
    
    let result = thjalfi.start_service(config, Duration::from_millis(100)).await;
    // Should fail due to timeout or command not found
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_running_services() {
    let thjalfi = Thjalfi::new().unwrap();
    
    let services = thjalfi.list_running_services().await;
    assert_eq!(services.len(), 0);
}

#[tokio::test]
async fn test_has_service() {
    let thjalfi = Thjalfi::new().unwrap();
    
    assert!(!thjalfi.has_service("nonexistent").await);
}

#[tokio::test]
async fn test_restart_service() {
    let thjalfi = Thjalfi::new().unwrap();
    let startup = Duration::from_secs(3);
    let shutdown = Some(Duration::from_secs(2));

    #[cfg(unix)]
    let config = ServiceConfig {
        name: "restart-service".to_string(),
        command: "sleep".to_string(),
        args: vec!["5".to_string()],
        working_directory: None,
        environment_vars: std::collections::HashMap::new(),
    };
    #[cfg(windows)]
    let config = ServiceConfig {
        name: "restart-service".to_string(),
        command: "timeout".to_string(),
        args: vec!["/t".to_string(), "5".to_string()],
        working_directory: None,
        environment_vars: std::collections::HashMap::new(),
    };

    thjalfi.start_service(config.clone(), startup).await.unwrap();
    assert!(thjalfi.has_service("restart-service").await);

    let result = thjalfi.restart_service("restart-service", config.clone(), startup, shutdown).await;
    assert!(result.is_ok(), "restart_service failed: {:?}", result.err());
    assert!(thjalfi.has_service("restart-service").await);

    thjalfi.stop_service("restart-service", true, None).await.unwrap();
    assert!(!thjalfi.has_service("restart-service").await);
}
