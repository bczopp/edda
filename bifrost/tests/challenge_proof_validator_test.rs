//! Tests for Phase 4.1.4: ChallengeProofValidator (verify with public key, expiration, signature).

use bifrost::message::MessageType;
use bifrost::security::{ChallengeProofHandler, ChallengeProofValidator, KeyGenerator};
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn validate_accepts_valid_proof() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let challenge = "valid-challenge";
    let proof = ChallengeProofHandler::build_challenge_proof(
        "client-1", "server-1", challenge, kp.secret_key(),
    )
    .unwrap();
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let expires_at = now_secs + 300;
    let result = ChallengeProofValidator::validate_proof(&proof, kp.public_key(), expires_at);
    assert!(result.is_ok());
}

#[test]
fn validate_rejects_expired_challenge() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let proof = ChallengeProofHandler::build_challenge_proof(
        "c", "s", "ch", kp.secret_key(),
    )
    .unwrap();
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let expires_at = now_secs - 1;
    let result = ChallengeProofValidator::validate_proof(&proof, kp.public_key(), expires_at);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().to_lowercase().contains("expir"));
}

#[test]
fn validate_rejects_wrong_public_key() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let other_kp = KeyGenerator::generate_ed25519().unwrap();
    let proof = ChallengeProofHandler::build_challenge_proof(
        "c", "s", "ch", kp.secret_key(),
    )
    .unwrap();
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let result = ChallengeProofValidator::validate_proof(&proof, other_kp.public_key(), now_secs + 300);
    assert!(result.is_err());
}

#[test]
fn validate_rejects_wrong_message_type() {
    let kp = KeyGenerator::generate_ed25519().unwrap();
    let mut proof = ChallengeProofHandler::build_challenge_proof(
        "c", "s", "ch", kp.secret_key(),
    )
    .unwrap();
    proof.message_type = MessageType::Message;
    let now_secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let result = ChallengeProofValidator::validate_proof(&proof, kp.public_key(), now_secs + 300);
    assert!(result.is_err());
}

#[test]
fn validator_new_exists() {
    let _ = ChallengeProofValidator::new();
}
