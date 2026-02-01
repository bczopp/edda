//! Tests for Phase 4.1.3: ChallengeProofHandler (sign challenge, proof message, signature).

use bifrost::message::{BifrostMessage, MessageHandler, MessageType};
use bifrost::security::{ChallengeProofHandler, KeyGenerator};

#[test]
fn build_challenge_proof_returns_challenge_proof_type() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeProofHandler::build_challenge_proof(
        "client-1",
        "server-1",
        "challenge-abc123",
        kp.secret_key(),
    )
    .unwrap();
    assert_eq!(msg.message_type, MessageType::ChallengeProof);
    assert_eq!(msg.source_device_id, "client-1");
    assert_eq!(msg.target_device_id, "server-1");
}

#[test]
fn payload_contains_challenge() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let challenge = "my-challenge-string";
    let msg = ChallengeProofHandler::build_challenge_proof(
        "c", "s", challenge, kp.secret_key(),
    )
    .unwrap();
    let payload_challenge = msg.payload.get("challenge").and_then(|v| v.as_str()).unwrap();
    assert_eq!(payload_challenge, challenge);
}

#[test]
fn payload_contains_proof_signature() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeProofHandler::build_challenge_proof(
        "c", "s", "ch", kp.secret_key(),
    )
    .unwrap();
    let sig = msg.payload.get("proof_signature").and_then(|v| v.as_str()).unwrap();
    assert!(!sig.is_empty());
}

#[test]
fn same_challenge_same_key_produces_same_signature() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let challenge = "deterministic-challenge";
    let msg1 = ChallengeProofHandler::build_challenge_proof("c", "s", challenge, kp.secret_key()).unwrap();
    let msg2 = ChallengeProofHandler::build_challenge_proof("c", "s", challenge, kp.secret_key()).unwrap();
    let sig1 = msg1.payload.get("proof_signature").unwrap().as_str().unwrap();
    let sig2 = msg2.payload.get("proof_signature").unwrap().as_str().unwrap();
    assert_eq!(sig1, sig2);
}

#[test]
fn roundtrip_serialization() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let msg = ChallengeProofHandler::build_challenge_proof(
        "c", "s", "ch", kp.secret_key(),
    )
    .unwrap();
    let json = MessageHandler::serialize_message(&msg).unwrap();
    let parsed: BifrostMessage = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed.message_type, MessageType::ChallengeProof);
}
