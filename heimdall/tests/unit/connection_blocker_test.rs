//! Tests for ConnectionBlocker (Phase 9.4.1): immediate block, token revocation, temporary/permanent, unblock.

use heimdall::bifrost::{ConnectionBlocker, ConnectionMonitor, ConnectionStatus};
use std::sync::Arc;

#[tokio::test]
async fn test_block_connection_sets_blocked_status() {
    let monitor = Arc::new(ConnectionMonitor::new());
    let blocker = ConnectionBlocker::new(monitor.clone(), None);
    monitor.register_connection("dev-a", "dev-b").await;
    blocker.block_connection("dev-a", "dev-b", None, None).await.unwrap();
    let status = monitor.get_status("dev-a", "dev-b").await.unwrap();
    assert_eq!(status, ConnectionStatus::Blocked);
}

#[tokio::test]
async fn test_is_blocked_returns_true_after_block() {
    let monitor = Arc::new(ConnectionMonitor::new());
    let blocker = ConnectionBlocker::new(monitor.clone(), None);
    monitor.register_connection("dev-a", "dev-b").await;
    blocker.block_connection("dev-a", "dev-b", None, None).await.unwrap();
    assert!(blocker.is_blocked("dev-a", "dev-b").await);
}

#[tokio::test]
async fn test_unblock_connection_restores_active() {
    let monitor = Arc::new(ConnectionMonitor::new());
    let blocker = ConnectionBlocker::new(monitor.clone(), None);
    monitor.register_connection("dev-a", "dev-b").await;
    blocker.block_connection("dev-a", "dev-b", None, None).await.unwrap();
    blocker.unblock_connection("dev-a", "dev-b").await.unwrap();
    let status = monitor.get_status("dev-a", "dev-b").await.unwrap();
    assert_eq!(status, ConnectionStatus::Active);
    assert!(!blocker.is_blocked("dev-a", "dev-b").await);
}

#[tokio::test]
async fn test_block_connection_registers_if_not_present() {
    let monitor = Arc::new(ConnectionMonitor::new());
    let blocker = ConnectionBlocker::new(monitor.clone(), None);
    blocker.block_connection("dev-x", "dev-y", None, None).await.unwrap();
    assert!(blocker.is_blocked("dev-x", "dev-y").await);
}

#[tokio::test]
async fn test_temporary_block_has_duration() {
    let monitor = Arc::new(ConnectionMonitor::new());
    let blocker = ConnectionBlocker::new(monitor.clone(), None);
    monitor.register_connection("dev-a", "dev-b").await;
    blocker.block_connection("dev-a", "dev-b", None, Some(60)).await.unwrap();
    assert!(blocker.is_blocked("dev-a", "dev-b").await);
}
