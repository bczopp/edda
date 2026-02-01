//! Tests for Phase 4.1.1: ChallengeRequestHandler (message, device-id, public-key, signature).

use bifrost::message::{BifrostMessage, MessageHandler, MessageType};
use bifrost::security::challenge::ChallengeRequestHandler;
use bifrost::security::KeyGenerator;

#[test]
fn build_challenge_request_returns_challenge_request_type() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeRequestHandler::build_challenge_request("dev-1", "target-1", kp.public_key(), kp.secret_key()).unwrap();
    assert_eq!(msg.message_type, MessageType::ChallengeRequest);
    assert_eq!(msg.source_device_id, "dev-1");
    assert_eq!(msg.target_device_id, "target-1");
}

#[test]
fn build_challenge_request_includes_public_key_in_payload() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeRequestHandler::build_challenge_request("dev-1", "target-1", kp.public_key(), kp.secret_key()).unwrap();
    let pk = msg.payload.get("public_key").and_then(|v| v.as_str()).unwrap();
    assert!(!pk.is_empty());
}

#[test]
fn build_challenge_request_includes_signature_in_payload() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeRequestHandler::build_challenge_request("dev-1", "target-1", kp.public_key(), kp.secret_key()).unwrap();
    let sig = msg.payload.get("signature").and_then(|v| v.as_str()).unwrap();
    assert!(!sig.is_empty());
}

#[test]
fn build_challenge_request_roundtrips_serialization() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeRequestHandler::build_challenge_request("dev-1", "target-1", kp.public_key(), kp.secret_key()).unwrap();
    let json = MessageHandler::serialize_message(&msg).unwrap();
    let parsed: BifrostMessage = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.message_type, MessageType::ChallengeRequest);
    assert_eq!(parsed.source_device_id, "dev-1");
}

#[test]
fn handler_new_exists() {
    let _ = ChallengeRequestHandler::new();
}
