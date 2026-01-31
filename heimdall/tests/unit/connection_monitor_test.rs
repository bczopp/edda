//! Tests for ConnectionMonitor (Phase 9.3.1): status tracking, heartbeat, message monitoring.

use heimdall::bifrost::{ConnectionMonitor, ConnectionStatus};

#[tokio::test]
async fn test_register_and_get_status() {
    let monitor = ConnectionMonitor::new();
    monitor.register_connection("dev-a", "dev-b").await;
    let status = monitor.get_status("dev-a", "dev-b").await.unwrap();
    assert_eq!(status, ConnectionStatus::Active);
}

#[tokio::test]
async fn test_set_status_blocked() {
    let monitor = ConnectionMonitor::new();
    monitor.register_connection("dev-a", "dev-b").await;
    monitor.set_status("dev-a", "dev-b", ConnectionStatus::Blocked).await;
    let status = monitor.get_status("dev-a", "dev-b").await.unwrap();
    assert_eq!(status, ConnectionStatus::Blocked);
}

#[tokio::test]
async fn test_record_heartbeat_updates_activity() {
    let monitor = ConnectionMonitor::new();
    monitor.register_connection("dev-a", "dev-b").await;
    monitor.record_heartbeat("dev-a", "dev-b").await;
    let valid = monitor.is_heartbeat_valid("dev-a", "dev-b", 300).await;
    assert!(valid);
}

#[tokio::test]
async fn test_heartbeat_invalid_after_max_idle() {
    let monitor = ConnectionMonitor::new();
    monitor.register_connection("dev-a", "dev-b").await;
    monitor.record_heartbeat("dev-a", "dev-b").await;
    // max_idle_seconds=0: valid only if idle <= 0 (same second); wait 2s so idle > 0
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let valid = monitor.is_heartbeat_valid("dev-a", "dev-b", 0).await;
    assert!(!valid);
}

#[tokio::test]
async fn test_record_message() {
    let monitor = ConnectionMonitor::new();
    monitor.register_connection("dev-a", "dev-b").await;
    monitor.record_message("dev-a", "dev-b").await;
    let status = monitor.get_status("dev-a", "dev-b").await;
    assert!(status.is_some());
}

#[tokio::test]
async fn test_get_status_returns_none_for_unknown_connection() {
    let monitor = ConnectionMonitor::new();
    let status = monitor.get_status("unknown", "unknown").await;
    assert!(status.is_none());
}
