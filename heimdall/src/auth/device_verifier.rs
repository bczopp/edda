//! Device identity verification: public key validation, signature verification.

use crate::keys::SignatureManager;
use base64::{Engine as _, engine::general_purpose};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceVerifierError {
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),
    #[error("Invalid base64: {0}")]
    InvalidBase64(String),
    #[error("Signature verification failed: {0}")]
    SignatureError(#[from] crate::keys::SignatureError),
}

/// Ed25519 public key length in bytes.
pub const ED25519_PUBLIC_KEY_LEN: usize = 32;

/// Verifies device identity: public key format and signature verification.
pub struct DeviceIdentityVerifier;

impl DeviceIdentityVerifier {
    /// Validates device public key format (Ed25519: 32 bytes, non-zero).
    pub fn validate_public_key(&self, public_key: &[u8]) -> Result<(), DeviceVerifierError> {
        if public_key.len() != ED25519_PUBLIC_KEY_LEN {
            return Err(DeviceVerifierError::InvalidPublicKey(format!(
                "expected {} bytes, got {}",
                ED25519_PUBLIC_KEY_LEN,
                public_key.len()
            )));
        }
        if public_key.iter().all(|&b| b == 0) {
            return Err(DeviceVerifierError::InvalidPublicKey(
                "all-zero public key is invalid".to_string(),
            ));
        }
        Ok(())
    }

    /// Validates device public key given as base64 string.
    pub fn validate_public_key_base64(&self, public_key_b64: &str) -> Result<(), DeviceVerifierError> {
        let bytes = general_purpose::STANDARD.decode(public_key_b64.trim()).map_err(|e| {
            DeviceVerifierError::InvalidBase64(e.to_string())
        })?;
        self.validate_public_key(&bytes)
    }

    /// Verifies that `signature` was produced by the device holding `public_key` for `message`.
    /// Returns `Ok(true)` if valid, `Ok(false)` if signature invalid, `Err` if public key invalid.
    pub fn verify_identity(
        &self,
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, DeviceVerifierError> {
        self.validate_public_key(public_key)?;
        match SignatureManager::verify(public_key, message, signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
