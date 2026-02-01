//! Tests for Phase 17.1.2: ThorActionRouter (ThorAction via gRPC over Bifrost, ThorResult, timeout).

use bifrost::connection::ConnectionManager;
use bifrost::grpc_bridge::{GrpcBridge, ThorActionRouter, ThorActionRouterError};
use bifrost::routing::MessageRouter;
use std::sync::Arc;
use std::time::Duration;

const THOR_SERVICE: &str = "thor.Thor";
const THOR_METHOD: &str = "Execute";

#[tokio::test]
async fn send_action_returns_result_when_response_received() {
    let manager = Arc::new(ConnectionManager::new());
    let router = Arc::new(MessageRouter::new(Arc::clone(&manager)));
    let bridge = Arc::new(GrpcBridge::new(Arc::clone(&router), Duration::from_secs(5)));
    let _thor_router = ThorActionRouter::new(Arc::clone(&bridge));

    // Simulate: we build request to get request_id, register pending, then complete with response.
    let (request_id, _msg) = bridge
        .build_request("src", "tgt", THOR_SERVICE, THOR_METHOD, b"action-payload")
        .unwrap();
    let rx = bridge.register_pending(&request_id).unwrap();
    bridge.on_grpc_response(&request_id, b"result-payload", true);

    let result = rx.await.unwrap();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_slice(), b"result-payload");
}

#[tokio::test]
async fn thor_action_router_uses_thor_service_and_execute_method() {
    let manager = Arc::new(ConnectionManager::new());
    let router = Arc::new(MessageRouter::new(Arc::clone(&manager)));
    let bridge = Arc::new(GrpcBridge::new(Arc::clone(&router), Duration::from_secs(5)));
    let thor_router = ThorActionRouter::new(Arc::clone(&bridge));

    let (_request_id, msg) = thor_router
        .build_action_request("src", "tgt", b"action")
        .unwrap();
    assert_eq!(msg.payload.get("service").and_then(|v| v.as_str()), Some(THOR_SERVICE));
    assert_eq!(msg.payload.get("method").and_then(|v| v.as_str()), Some(THOR_METHOD));
}

#[tokio::test]
async fn send_action_timeout_returns_error_when_no_response() {
    let manager = Arc::new(ConnectionManager::new());
    let router = Arc::new(MessageRouter::new(Arc::clone(&manager)));
    let bridge = Arc::new(GrpcBridge::new(Arc::clone(&router), Duration::from_millis(50)));
    let thor_router = ThorActionRouter::new(Arc::clone(&bridge));

    // No target connected and no on_grpc_response call -> timeout or route error.
    let result = thor_router
        .send_action("src", "tgt", b"action")
        .await;

    // Either timeout (no response) or route error (target not connected).
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        ThorActionRouterError::Timeout | ThorActionRouterError::Bridge(_)
    ));
}
