use crate::messages::*;
use ed25519_dalek::{SigningKey, VerifyingKey, Signer, Verifier, Signature};
use sha2::{Sha256, Digest};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SecurityError {
    #[error("Signature verification failed")]
    SignatureError,
    #[error("Nonce already used (replay attack detected)")]
    NonceReplay,
    #[error("Failed to create signature")]
    SigningError,
}

/// NonceManager manages nonces to prevent replay attacks
#[derive(Clone)]
pub struct NonceManager {
    used_nonces: Arc<Mutex<HashSet<Vec<u8>>>>,
    nonce_size: usize,
}

impl NonceManager {
    /// Create a new NonceManager
    pub fn new() -> Self {
        Self {
            used_nonces: Arc::new(Mutex::new(HashSet::new())),
            nonce_size: 16,
        }
    }

    /// Create a new NonceManager with custom nonce size
    pub fn with_nonce_size(nonce_size: usize) -> Self {
        Self {
            used_nonces: Arc::new(Mutex::new(HashSet::new())),
            nonce_size,
        }
    }

    /// Generate a new nonce
    pub fn generate_nonce(&self) -> Vec<u8> {
        use rand::RngCore;
        let mut rng = rand::thread_rng();
        let mut nonce = vec![0u8; self.nonce_size];
        rng.fill_bytes(&mut nonce);
        nonce
    }

    /// Validate and record a nonce (returns error if nonce was already used)
    pub fn validate_and_record_nonce(&self, nonce: &[u8]) -> Result<(), SecurityError> {
        let mut used = self.used_nonces.lock().unwrap();
        if used.contains(nonce) {
            return Err(SecurityError::NonceReplay);
        }
        used.insert(nonce.to_vec());
        Ok(())
    }

    /// Clear old nonces (for memory management)
    pub fn clear_old_nonces(&self) {
        let mut used = self.used_nonces.lock().unwrap();
        // In a real implementation, you might want to implement a time-based cleanup
        // For now, we just clear all (this is a simple example)
        used.clear();
    }
}

impl Default for NonceManager {
    fn default() -> Self {
        Self::new()
    }
}

/// MessageSigner handles message signing and verification
#[derive(Clone)]
pub struct MessageSigner {
    signing_key: SigningKey,
}

impl MessageSigner {
    /// Create a new MessageSigner with a signing key
    pub fn new(signing_key: SigningKey) -> Self {
        Self { signing_key }
    }

    /// Create a new MessageSigner from bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, ed25519_dalek::SignatureError> {
        let signing_key = SigningKey::from_bytes(bytes.try_into().map_err(|e| {
            ed25519_dalek::SignatureError::from_source(e)
        })?);
        Ok(Self { signing_key })
    }

    /// Get the verifying key
    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    /// Sign a request
    pub fn sign_request(&self, request: &mut RatatoskrRequest) -> Result<(), SecurityError> {
        // Create message to sign: request_id + device_id + user_id + timestamp + nonce + payload
        let mut message = Vec::new();
        message.extend_from_slice(request.request_id.as_bytes());
        message.extend_from_slice(request.device_id.as_bytes());
        message.extend_from_slice(request.user_id.as_bytes());
        message.extend_from_slice(&request.timestamp.to_be_bytes());
        message.extend_from_slice(&request.nonce);
        message.extend_from_slice(&request.payload);

        // Hash the message
        let mut hasher = Sha256::new();
        hasher.update(&message);
        let hash = hasher.finalize();

        // Sign the hash
        let signature = self.signing_key.sign(&hash);
        request.signature = signature.to_bytes().to_vec();

        Ok(())
    }

    /// Verify a request signature
    pub fn verify_request(
        &self,
        request: &RatatoskrRequest,
        verifying_key: &VerifyingKey,
    ) -> Result<(), SecurityError> {
        // Recreate message to verify
        let mut message = Vec::new();
        message.extend_from_slice(request.request_id.as_bytes());
        message.extend_from_slice(request.device_id.as_bytes());
        message.extend_from_slice(request.user_id.as_bytes());
        message.extend_from_slice(&request.timestamp.to_be_bytes());
        message.extend_from_slice(&request.nonce);
        message.extend_from_slice(&request.payload);

        // Hash the message
        let mut hasher = Sha256::new();
        hasher.update(&message);
        let hash = hasher.finalize();

        // Verify the signature
        let signature = Signature::from_bytes(
            request.signature.as_slice().try_into().map_err(|_| {
                SecurityError::SigningError
            })?
        );
        verifying_key.verify(&hash, &signature).map_err(|_| SecurityError::SignatureError)?;

        Ok(())
    }
}
