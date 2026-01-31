//! Message format and serialization tests (Phase 2.1, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::message::{BifrostMessage, MessageHandler, MessageType};
use serde_json;

#[test]
fn message_type_serializes_to_uppercase_string() {
    let json = serde_json::to_string(&MessageType::Message).unwrap();
    assert_eq!(json, r#""MESSAGE""#);
    let json = serde_json::to_string(&MessageType::Heartbeat).unwrap();
    assert_eq!(json, r#""HEARTBEAT""#);
}

#[test]
fn message_type_deserializes_from_uppercase_string() {
    let t: MessageType = serde_json::from_str(r#""CONNECTION_REQUEST""#).unwrap();
    assert!(matches!(t, MessageType::ConnectionRequest));
    let t: MessageType = serde_json::from_str(r#""HEARTBEAT""#).unwrap();
    assert!(matches!(t, MessageType::Heartbeat));
}

#[test]
fn message_type_roundtrip() {
    for variant in [
        MessageType::ConnectionRequest,
        MessageType::ConnectionResponse,
        MessageType::Message,
        MessageType::Heartbeat,
        MessageType::Disconnect,
        MessageType::Error,
        MessageType::VersionNegotiation,
        MessageType::ChallengeRequest,
        MessageType::ChallengeResponse,
        MessageType::ChallengeProof,
        MessageType::AuthenticationResult,
    ] {
        let json = serde_json::to_string(&variant).unwrap();
        let back: MessageType = serde_json::from_str(&json).unwrap();
        assert_eq!(std::mem::discriminant(&variant), std::mem::discriminant(&back));
    }
}

#[test]
fn invalid_message_type_deserialize_fails() {
    let r: Result<MessageType, _> = serde_json::from_str(r#""UNKNOWN_TYPE""#);
    assert!(r.is_err());
}

#[test]
fn bifrost_message_roundtrip() {
    let msg = BifrostMessage {
        message_id: "id-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "dev-a".to_string(),
        target_device_id: "dev-b".to_string(),
        payload: serde_json::json!({"key": "value"}),
        timestamp: 12345,
        protocol_version: Some(1),
    };
    let json = MessageHandler::serialize_message(&msg).unwrap();
    let parsed = MessageHandler::parse_message(&json).unwrap();
    assert_eq!(parsed.message_id, msg.message_id);
    assert!(matches!(parsed.message_type, MessageType::Message));
    assert_eq!(parsed.source_device_id, msg.source_device_id);
    assert_eq!(parsed.target_device_id, msg.target_device_id);
    assert_eq!(parsed.timestamp, msg.timestamp);
    assert_eq!(parsed.protocol_version, Some(1));
}

#[test]
fn bifrost_message_without_version_deserializes() {
    let json = r#"{"message_id":"x","message_type":"HEARTBEAT","source_device_id":"a","target_device_id":"b","payload":{},"timestamp":0}"#;
    let msg = MessageHandler::parse_message(json).unwrap();
    assert!(matches!(msg.message_type, MessageType::Heartbeat));
    assert_eq!(msg.protocol_version, None);
}
