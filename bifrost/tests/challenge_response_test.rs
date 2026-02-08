//! Tests for Phase 4.1.2: ChallengeResponseGenerator (random challenge, expiration, signature).

use bifrost::message::{BifrostMessage, MessageHandler, MessageType};
use bifrost::security::{ChallengeResponseGenerator, KeyGenerator};

#[test]
fn build_challenge_response_returns_challenge_response_type() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeResponseGenerator::build_challenge_response(
        "server-1",
        "client-1",
        kp.secret_key(),
        std::time::Duration::from_secs(300),
    )
    .unwrap();
    assert_eq!(msg.message_type, MessageType::ChallengeResponse);
    assert_eq!(msg.source_device_id, "server-1");
    assert_eq!(msg.target_device_id, "client-1");
}

#[test]
fn payload_contains_challenge_string() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeResponseGenerator::build_challenge_response(
        "s", "c", kp.secret_key(), std::time::Duration::from_secs(60),
    )
    .unwrap();
    let challenge = msg.payload.get("challenge").and_then(|v| v.as_str()).unwrap();
    assert!(!challenge.is_empty());
}

#[test]
fn payload_contains_expires_at() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeResponseGenerator::build_challenge_response(
        "s", "c", kp.secret_key(), std::time::Duration::from_secs(60),
    )
    .unwrap();
    let expires = msg.payload.get("expires_at").and_then(|v| v.as_i64()).unwrap();
    assert!(expires > 0);
}

#[test]
fn payload_contains_signature() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeResponseGenerator::build_challenge_response(
        "s", "c", kp.secret_key(), std::time::Duration::from_secs(60),
    )
    .unwrap();
    let sig = msg.payload.get("signature").and_then(|v| v.as_str()).unwrap();
    assert!(!sig.is_empty());
}

#[test]
fn challenge_string_differs_each_time() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg1 = ChallengeResponseGenerator::build_challenge_response(
        "s", "c", kp.secret_key(), std::time::Duration::from_secs(60),
    )
    .unwrap();
    let msg2 = ChallengeResponseGenerator::build_challenge_response(
        "s", "c", kp.secret_key(), std::time::Duration::from_secs(60),
    )
    .unwrap();
    let c1 = msg1.payload.get("challenge").unwrap().as_str().unwrap();
    let c2 = msg2.payload.get("challenge").unwrap().as_str().unwrap();
    assert_ne!(c1, c2);
}

#[test]
fn roundtrip_serialization() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeResponseGenerator::build_challenge_response(
        "s", "c", kp.secret_key(), std::time::Duration::from_secs(60),
    )
    .unwrap();
    let json = MessageHandler::serialize_message(&msg).unwrap();
    let parsed: BifrostMessage = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.message_type, MessageType::ChallengeResponse);
}
