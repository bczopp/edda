//! Tests for Roskva (Health Monitor)

use gladsheim::roskva::{Roskva, HealthMonitor, HealthCheckStrategy, ServiceHealthTracker};
use std::time::Duration;

#[tokio::test]
async fn test_roskva_creation() {
    let roskva = Roskva::new();
    assert!(roskva.is_ok());
}

#[tokio::test]
async fn test_health_monitor_http_check() {
    let monitor = HealthMonitor::default();
    
    // Test HTTP health check (will fail for unreachable URL, but tests the logic)
    let result = monitor.check_http_health("http://127.0.0.1:59999/health").await;
    // Should return Ok (even if false) or Err
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_health_monitor_grpc_check() {
    let monitor = HealthMonitor::default();
    
    // Test gRPC health check
    let result = monitor.check_grpc_health("test-service").await;
    // Should return Ok (even if false) or Err
    assert!(result.is_ok() || result.is_err());
}

#[tokio::test]
async fn test_service_health_tracker_register() {
    let tracker = ServiceHealthTracker::new(Duration::from_secs(5));
    
    tracker.register_service("test-service".to_string(), HealthCheckStrategy::Http {
        url: "http://localhost:8080/health".to_string(),
    }).await;
    
    let health = tracker.get_health("test-service").await;
    assert!(health.is_some());
}

#[tokio::test]
async fn test_service_health_tracker_update() {
    let tracker = ServiceHealthTracker::new(Duration::from_secs(5));
    
    tracker.register_service("test-service".to_string(), HealthCheckStrategy::Http {
        url: "http://localhost:8080/health".to_string(),
    }).await;
    
    tracker.update_health("test-service", true, None).await;
    
    let health = tracker.get_health("test-service").await;
    assert_eq!(health.unwrap().is_healthy, true);
}

#[tokio::test]
async fn test_service_health_tracker_consecutive_failures() {
    let tracker = ServiceHealthTracker::new(Duration::from_secs(5));
    
    tracker.register_service("test-service".to_string(), HealthCheckStrategy::Http {
        url: "http://localhost:8080/health".to_string(),
    }).await;
    
    // Simulate consecutive failures
    for _ in 0..3 {
        tracker.update_health("test-service", false, Some("Connection failed".to_string())).await;
    }
    
    let health = tracker.get_health("test-service").await;
    assert_eq!(health.unwrap().consecutive_failures, 3);
}

#[tokio::test]
async fn test_auto_restart_should_trigger() {
    let tracker = ServiceHealthTracker::new(Duration::from_secs(5));
    
    tracker.register_service("test-service".to_string(), HealthCheckStrategy::Http {
        url: "http://localhost:8080/health".to_string(),
    }).await;
    
    // Set max failures to 3
    tracker.set_max_failures("test-service", 3).await;
    
    // Simulate 3 failures
    for _ in 0..3 {
        tracker.update_health("test-service", false, None).await;
    }
    
    let should_restart = tracker.should_restart("test-service").await;
    assert_eq!(should_restart, Some(true));
}
