//! Tests for Phase 7.1.2: ConnectionInitiator (request, response, establishment).

use bifrost::message::{BifrostMessage, MessageType};
use bifrost::websocket::ConnectionInitiator;

#[test]
fn build_connection_request_returns_request_message() {
    let initiator = ConnectionInitiator::new();
    let msg = ConnectionInitiator::build_connection_request("dev-1", "user-1", "target-1");
    assert_eq!(msg.message_type, MessageType::ConnectionRequest);
    assert_eq!(msg.source_device_id, "dev-1");
    assert_eq!(msg.target_device_id, "target-1");
}

#[test]
fn parse_connection_response_accepts_connection_response() {
    let response = BifrostMessage {
        message_id: "r1".to_string(),
        message_type: MessageType::ConnectionResponse,
        source_device_id: "srv".to_string(),
        target_device_id: "dev-1".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: Some(1),
    };
    let json = bifrost::message::MessageHandler::serialize_message(&response).unwrap();
    let parsed = ConnectionInitiator::parse_connection_response(&json).unwrap();
    assert_eq!(parsed.message_type, MessageType::ConnectionResponse);
    assert_eq!(parsed.message_id, "r1");
}

#[test]
fn parse_connection_response_rejects_wrong_type() {
    let msg = BifrostMessage {
        message_id: "m1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "s".to_string(),
        target_device_id: "t".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    };
    let json = bifrost::message::MessageHandler::serialize_message(&msg).unwrap();
    let result = ConnectionInitiator::parse_connection_response(&json);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("CONNECTION_RESPONSE"));
}

#[test]
fn initiator_new_creates() {
    let _ = ConnectionInitiator::new();
}
