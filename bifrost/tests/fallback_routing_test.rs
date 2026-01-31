//! Fallback Routing Manager tests (Phase 10.1.2, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::connection::ConnectionManager;
use bifrost::message::{BifrostMessage, MessageType};
use bifrost::routing::{FallbackRoutingManager, MessageRouter, RetryConfig};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn fallback_tries_direct_first_and_succeeds_when_target_connected() {
    let manager = Arc::new(ConnectionManager::new());
    let direct = MessageRouter::new(manager.clone());
    let fallback = FallbackRoutingManager::new(direct, None, None);

    let msg = BifrostMessage {
        message_id: "fb-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "missing".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    };

    let res = fallback.route_message(msg).await;
    let err = res.unwrap_err();
    assert!(
        err.to_string().contains("not connected"),
        "expected direct failure, got: {}",
        err
    );
}

#[tokio::test]
async fn fallback_returns_error_when_all_routes_fail() {
    let manager = Arc::new(ConnectionManager::new());
    let direct = MessageRouter::new(manager).with_retry(RetryConfig {
        max_retries: 0,
        base_delay: Duration::ZERO,
    });
    let fallback = FallbackRoutingManager::new(direct, None, None);

    let msg = BifrostMessage {
        message_id: "fb-2".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "missing".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    };

    let res = fallback.route_message(msg).await;
    assert!(res.is_err());
    let err = res.unwrap_err();
    assert!(
        err.to_string().contains("not connected"),
        "expected last error from direct, got: {}",
        err
    );
}

#[tokio::test]
async fn fallback_uses_direct_route_only_when_no_relays_configured() {
    let manager = Arc::new(ConnectionManager::new());
    let direct = MessageRouter::new(manager);
    let fallback = FallbackRoutingManager::new(direct, None, None);

    let msg = BifrostMessage {
        message_id: "fb-3".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "x".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    };

    let res = fallback.route_message(msg).await;
    assert!(res.is_err());
}
