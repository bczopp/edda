//! Message validation: signature verification, reject invalid messages.

use crate::keys::SignatureManager;
use thiserror::Error;

const ED25519_PUBLIC_KEY_LEN: usize = 32;
const ED25519_SIGNATURE_LEN: usize = 64;

#[derive(Debug, Error)]
pub enum MessageValidationError {
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),
    #[error("Invalid signature length")]
    InvalidSignatureLength,
    #[error("Signature verification failed: {0}")]
    Verification(#[from] crate::keys::SignatureError),
}

/// Validates Bifrost messages: signature check; invalid messages rejected.
pub struct MessageValidator;

impl MessageValidator {
    /// Verifies message signature. Returns true if valid, false if invalid, Err on bad key/signature format.
    pub fn verify_message(
        &self,
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool, MessageValidationError> {
        if public_key.len() != ED25519_PUBLIC_KEY_LEN {
            return Err(MessageValidationError::InvalidPublicKey(format!(
                "expected {} bytes, got {}",
                ED25519_PUBLIC_KEY_LEN,
                public_key.len()
            )));
        }
        if signature.len() != ED25519_SIGNATURE_LEN {
            return Err(MessageValidationError::InvalidSignatureLength);
        }
        match SignatureManager::verify(public_key, message, signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
