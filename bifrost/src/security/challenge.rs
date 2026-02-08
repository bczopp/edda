//! Challenge-Request Handler (Phase 4.1.1). Challenge-Response Generator (Phase 4.1.2).

use crate::message::{BifrostMessage, MessageType};
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::RngCore;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
#[error("challenge request build failed")]
pub struct ChallengeRequestError;

#[derive(Error, Debug)]
#[error("challenge response build failed")]
pub struct ChallengeResponseError;

#[derive(Error, Debug)]
#[error("challenge proof build failed")]
pub struct ChallengeProofError;

#[derive(Error, Debug)]
pub enum ChallengeProofValidationError {
    #[error("wrong message type")]
    WrongMessageType,
    #[error("missing payload field: {0}")]
    MissingPayload(&'static str),
    #[error("invalid signature")]
    InvalidSignature,
    #[error("challenge expired")]
    Expired,
}

/// Builds challenge-request messages with device-id, public-key, and Ed25519 signature.
pub struct ChallengeRequestHandler;

impl ChallengeRequestHandler {
    pub fn new() -> Self {
        Self
    }

    /// Builds a CHALLENGE_REQUEST message with device-id, public-key in payload, and signature over (source|target|timestamp|public_key).
    pub fn build_challenge_request(
        source_device_id: &str,
        target_device_id: &str,
        public_key: &[u8; 32],
        secret_key: &[u8; 32],
    ) -> Result<BifrostMessage, ChallengeRequestError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let public_key_b64 =
            base64::engine::general_purpose::STANDARD.encode(public_key.as_slice());
        let to_sign = format!(
            "{}|{}|{}|{}",
            source_device_id,
            target_device_id,
            timestamp,
            public_key_b64
        );
        let signing_key = SigningKey::from_bytes(secret_key);
        let signature = signing_key.sign(to_sign.as_bytes());
        let signature_b64 =
            base64::engine::general_purpose::STANDARD.encode(signature.to_bytes().as_slice());
        let payload = serde_json::json!({
            "public_key": public_key_b64,
            "signature": signature_b64,
        });
        Ok(BifrostMessage {
            message_id: Uuid::new_v4().to_string(),
            message_type: MessageType::ChallengeRequest,
            source_device_id: source_device_id.to_string(),
            target_device_id: target_device_id.to_string(),
            payload,
            timestamp,
            protocol_version: Some(1),
        })
    }
}

impl Default for ChallengeRequestHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Generates CHALLENGE_RESPONSE: random challenge string, expiration, Ed25519 signature.
pub struct ChallengeResponseGenerator;

impl ChallengeResponseGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Builds a CHALLENGE_RESPONSE message with random challenge, expires_at, and signature over (challenge|expires_at|source|target).
    pub fn build_challenge_response(
        source_device_id: &str,
        target_device_id: &str,
        secret_key: &[u8; 32],
        validity_duration: Duration,
    ) -> Result<BifrostMessage, ChallengeResponseError> {
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let expires_at = now_secs
            .saturating_add(validity_duration.as_secs() as i64);

        let mut challenge_bytes = [0u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut challenge_bytes);
        let challenge_b64 =
            base64::engine::general_purpose::STANDARD.encode(challenge_bytes.as_slice());

        let to_sign = format!(
            "{}|{}|{}|{}",
            challenge_b64,
            expires_at,
            source_device_id,
            target_device_id
        );
        let signing_key = SigningKey::from_bytes(secret_key);
        let signature = signing_key.sign(to_sign.as_bytes());
        let signature_b64 =
            base64::engine::general_purpose::STANDARD.encode(signature.to_bytes().as_slice());

        let payload = serde_json::json!({
            "challenge": challenge_b64,
            "expires_at": expires_at,
            "signature": signature_b64,
        });
        Ok(BifrostMessage {
            message_id: Uuid::new_v4().to_string(),
            message_type: MessageType::ChallengeResponse,
            source_device_id: source_device_id.to_string(),
            target_device_id: target_device_id.to_string(),
            payload,
            timestamp: now_secs,
            protocol_version: Some(1),
        })
    }
}

impl Default for ChallengeResponseGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Builds CHALLENGE_PROOF: client signs the challenge with private key; proof message with signature.
pub struct ChallengeProofHandler;

impl ChallengeProofHandler {
    pub fn new() -> Self {
        Self
    }

    /// Builds a CHALLENGE_PROOF message: challenge string + Ed25519 signature over challenge (proof of possession).
    pub fn build_challenge_proof(
        source_device_id: &str,
        target_device_id: &str,
        challenge: &str,
        secret_key: &[u8; 32],
    ) -> Result<BifrostMessage, ChallengeProofError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let signing_key = SigningKey::from_bytes(secret_key);
        let signature = signing_key.sign(challenge.as_bytes());
        let proof_signature_b64 =
            base64::engine::general_purpose::STANDARD.encode(signature.to_bytes().as_slice());
        let payload = serde_json::json!({
            "challenge": challenge,
            "proof_signature": proof_signature_b64,
        });
        Ok(BifrostMessage {
            message_id: Uuid::new_v4().to_string(),
            message_type: MessageType::ChallengeProof,
            source_device_id: source_device_id.to_string(),
            target_device_id: target_device_id.to_string(),
            payload,
            timestamp,
            protocol_version: Some(1),
        })
    }
}

impl Default for ChallengeProofHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates CHALLENGE_PROOF: verify signature with public key, check expiration.
pub struct ChallengeProofValidator;

impl ChallengeProofValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validates proof message: message type, signature with public key, current time <= expires_at.
    pub fn validate_proof(
        proof_message: &BifrostMessage,
        public_key: &[u8; 32],
        expires_at: i64,
    ) -> Result<(), ChallengeProofValidationError> {
        if proof_message.message_type != MessageType::ChallengeProof {
            return Err(ChallengeProofValidationError::WrongMessageType);
        }
        let challenge = proof_message
            .payload
            .get("challenge")
            .and_then(|v| v.as_str())
            .ok_or(ChallengeProofValidationError::MissingPayload("challenge"))?;
        let proof_signature_b64 = proof_message
            .payload
            .get("proof_signature")
            .and_then(|v| v.as_str())
            .ok_or(ChallengeProofValidationError::MissingPayload("proof_signature"))?;
        let signature_bytes = base64::engine::general_purpose::STANDARD
            .decode(proof_signature_b64)
            .map_err(|_| ChallengeProofValidationError::InvalidSignature)?;
        let signature: [u8; 64] = signature_bytes
            .try_into()
            .map_err(|_| ChallengeProofValidationError::InvalidSignature)?;
        let signature = Signature::from_bytes(&signature);
        let verifying_key =
            VerifyingKey::from_bytes(public_key).map_err(|_| ChallengeProofValidationError::InvalidSignature)?;
        verifying_key
            .verify(challenge.as_bytes(), &signature)
            .map_err(|_| ChallengeProofValidationError::InvalidSignature)?;
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        if now_secs > expires_at {
            return Err(ChallengeProofValidationError::Expired);
        }
        Ok(())
    }
}

impl Default for ChallengeProofValidator {
    fn default() -> Self {
        Self::new()
    }
}
