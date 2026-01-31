//! Heartbeat mechanism tests (Phase 6.2.3, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::websocket::HeartbeatManager;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn should_send_heartbeat_returns_true_after_interval() {
    let mut mgr = HeartbeatManager::new(Duration::from_millis(50), Duration::from_secs(10));
    assert!(!mgr.should_send_heartbeat());
    mgr.record_sent();
    assert!(!mgr.should_send_heartbeat());
    sleep(Duration::from_millis(60)).await;
    assert!(mgr.should_send_heartbeat());
}

#[tokio::test]
async fn record_sent_resets_should_send_heartbeat() {
    let mut mgr = HeartbeatManager::new(Duration::from_millis(30), Duration::from_secs(10));
    sleep(Duration::from_millis(40)).await;
    assert!(mgr.should_send_heartbeat());
    mgr.record_sent();
    assert!(!mgr.should_send_heartbeat());
    sleep(Duration::from_millis(35)).await;
    assert!(mgr.should_send_heartbeat());
}

#[tokio::test]
async fn should_timeout_returns_true_after_no_heartbeat_received() {
    let mut mgr = HeartbeatManager::new(Duration::from_secs(10), Duration::from_millis(50));
    assert!(!mgr.should_timeout());
    mgr.record_received();
    assert!(!mgr.should_timeout());
    sleep(Duration::from_millis(60)).await;
    assert!(mgr.should_timeout());
}

#[tokio::test]
async fn record_received_resets_should_timeout() {
    let mut mgr = HeartbeatManager::new(Duration::from_secs(10), Duration::from_millis(40));
    sleep(Duration::from_millis(50)).await;
    assert!(mgr.should_timeout());
    mgr.record_received();
    assert!(!mgr.should_timeout());
    sleep(Duration::from_millis(45)).await;
    assert!(mgr.should_timeout());
}

#[test]
fn new_with_zero_interval_does_not_panic() {
    let _ = HeartbeatManager::new(Duration::ZERO, Duration::from_secs(5));
}
