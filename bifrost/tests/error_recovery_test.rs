//! Error Recovery Tests (Phase 20.1.2).
//! Covers Automatic-Reconnection, Retry-Mechanism, Fallback-Routing.
//! See also: retry_test.rs, reconnection_manager_test.rs, fallback_routing_test.rs.

use bifrost::connection::ConnectionManager;
use bifrost::message::{BifrostMessage, MessageType};
use bifrost::routing::{FallbackRoutingManager, MessageRouter, RetryConfig, RetryManager};
use bifrost::websocket::reconnection::{ReconnectionConfig, ReconnectionManager};
use std::sync::Arc;
use std::time::Duration;

// --- Retry-Mechanism (Phase 10.1.1) ---

#[test]
fn error_recovery_retry_manager_exponential_backoff() {
    let mut mgr = RetryManager::new(3, Duration::from_secs(1));
    assert!(mgr.should_retry());
    mgr.record_attempt();
    assert_eq!(mgr.next_delay(), Duration::from_secs(1));
    mgr.record_attempt();
    assert_eq!(mgr.next_delay(), Duration::from_secs(2));
    mgr.record_attempt();
    assert!(!mgr.should_retry());
}

// --- Automatic-Reconnection (Phase 7.2.1) ---

#[test]
fn error_recovery_reconnection_manager_delay_capped() {
    let config = ReconnectionConfig {
        base_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(60),
        jitter_ratio: 0.0,
    };
    let mut mgr = ReconnectionManager::new(config);
    for _ in 0..10 {
        mgr.record_attempt();
    }
    let delay = mgr.next_delay();
    assert!(delay <= Duration::from_secs(60), "delay must be capped at max_delay");
}

// --- Fallback-Routing (Phase 10.1.2) ---

#[tokio::test]
async fn error_recovery_fallback_returns_error_when_direct_fails_and_no_relay() {
    let manager = Arc::new(ConnectionManager::new());
    let direct = MessageRouter::new(manager).with_retry(RetryConfig {
        max_retries: 1,
        base_delay: Duration::from_millis(1),
    });
    let fallback = FallbackRoutingManager::new(direct, None, None);

    let msg = BifrostMessage {
        message_id: "err-rec-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "missing".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    };

    let res = fallback.route_message(msg).await;
    assert!(res.is_err());
}
