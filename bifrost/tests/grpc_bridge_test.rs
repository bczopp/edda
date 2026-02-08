//! Tests for Phase 17.1.1: GRPCBridge (tunnel gRPC requests/responses over Bifrost WebSocket).

use base64::Engine;
use bifrost::grpc_bridge::{GrpcBridge, GrpcBridgeError};
use bifrost::message::{BifrostMessage, MessageType};
use bifrost::connection::ConnectionManager;
use bifrost::routing::MessageRouter;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn send_request_builds_grpc_request_message() {
    let manager = Arc::new(ConnectionManager::new());
    let router = Arc::new(MessageRouter::new(Arc::clone(&manager)));
    let bridge = GrpcBridge::new(Arc::clone(&router), Duration::from_secs(5));

    let (_, msg) = bridge
        .build_request("src-dev", "tgt-dev", "thor.Thor", "Execute", b"payload")
        .unwrap();

    assert_eq!(msg.message_type, MessageType::GrpcRequest);
    assert_eq!(msg.source_device_id, "src-dev");
    assert_eq!(msg.target_device_id, "tgt-dev");
    assert!(msg.payload.get("request_id").and_then(|v| v.as_str()).is_some());
    assert_eq!(msg.payload.get("service").and_then(|v| v.as_str()), Some("thor.Thor"));
    assert_eq!(msg.payload.get("method").and_then(|v| v.as_str()), Some("Execute"));
    assert!(msg.payload.get("body").and_then(|v| v.as_str()).is_some());
}

#[tokio::test]
async fn complete_request_returns_response_body_when_response_received() {
    let manager = Arc::new(ConnectionManager::new());
    let router = Arc::new(MessageRouter::new(Arc::clone(&manager)));
    let bridge = GrpcBridge::new(Arc::clone(&router), Duration::from_secs(5));

    let request_id = "req-1".to_string();
    let rx = bridge.register_pending(&request_id).unwrap();

    let body = b"response-body";
    bridge.on_grpc_response(&request_id, body, true);

    let result = rx.await.unwrap();
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_slice(), b"response-body");
}

#[tokio::test]
async fn complete_request_returns_error_when_ok_false() {
    let manager = Arc::new(ConnectionManager::new());
    let router = Arc::new(MessageRouter::new(Arc::clone(&manager)));
    let bridge = GrpcBridge::new(Arc::clone(&router), Duration::from_secs(5));

    let request_id = "req-1".to_string();
    let rx = bridge.register_pending(&request_id).unwrap();

    bridge.on_grpc_response(&request_id, b"err", false);

    let result = rx.await.unwrap();
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), GrpcBridgeError::RemoteError(_)));
}

#[tokio::test]
async fn parse_response_extracts_request_id_and_body() {
    let manager = Arc::new(ConnectionManager::new());
    let router = Arc::new(MessageRouter::new(Arc::clone(&manager)));
    let _bridge = GrpcBridge::new(Arc::clone(&router), Duration::from_secs(5));

    let msg = BifrostMessage {
        message_id: "m1".to_string(),
        message_type: MessageType::GrpcResponse,
        source_device_id: "tgt".to_string(),
        target_device_id: "src".to_string(),
        payload: serde_json::json!({
            "request_id": "req-1",
            "body": base64::engine::general_purpose::STANDARD.encode(b"data"),
            "ok": true
        }),
        timestamp: 0,
        protocol_version: None,
    };

    let parsed = bifrost::grpc_bridge::parse_grpc_response_payload(&msg).unwrap();
    assert_eq!(parsed.request_id, "req-1");
    assert_eq!(parsed.body.as_slice(), b"data");
    assert!(parsed.ok);
}
