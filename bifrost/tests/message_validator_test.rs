//! Tests for Phase 15.1.1: MessageValidator (format, signature stub, sanitization).

use bifrost::message::{BifrostMessage, MessageType, MessageValidator, ValidationError};

fn valid_message() -> BifrostMessage {
    BifrostMessage {
        message_id: "msg-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "tgt".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    }
}

#[test]
fn valid_message_passes() {
    let validator = MessageValidator::new(1024, true);
    let msg = valid_message();
    assert!(validator.validate(&msg).is_ok());
}

#[test]
fn empty_message_id_fails() {
    let validator = MessageValidator::new(1024, true);
    let mut msg = valid_message();
    msg.message_id = String::new();
    let res = validator.validate(&msg);
    assert!(res.is_err());
    assert!(matches!(res.unwrap_err(), ValidationError::InvalidFormat(_)));
}

#[test]
fn empty_source_device_id_fails() {
    let validator = MessageValidator::new(1024, true);
    let mut msg = valid_message();
    msg.source_device_id = String::new();
    let res = validator.validate(&msg);
    assert!(res.is_err());
}

#[test]
fn payload_over_limit_fails() {
    let validator = MessageValidator::new(10, true);
    let mut msg = valid_message();
    msg.payload = serde_json::json!({"x": "long payload here"});
    let res = validator.validate(&msg);
    assert!(res.is_err());
    assert!(matches!(res.unwrap_err(), ValidationError::PayloadTooLarge));
}

#[test]
fn sanitize_removes_control_chars() {
    let validator = MessageValidator::new(1024, true);
    let mut msg = valid_message();
    msg.message_id = "ok\x00\x01\x1f".to_string();
    let sanitized = validator.sanitize(msg);
    assert!(!sanitized.message_id.contains('\x00'));
    assert!(!sanitized.message_id.contains('\x1f'));
}

#[test]
fn sanitize_truncates_overlong_ids() {
    let validator = MessageValidator::new(1024, true);
    let mut msg = valid_message();
    msg.message_id = "a".repeat(2000);
    let sanitized = validator.sanitize(msg);
    assert!(sanitized.message_id.len() <= 512);
}

#[test]
fn validate_after_sanitize_passes() {
    let validator = MessageValidator::new(1024, true);
    let mut msg = valid_message();
    msg.message_id = "id\x00\x01";
    let sanitized = validator.sanitize(msg);
    assert!(validator.validate(&sanitized).is_ok());
}
