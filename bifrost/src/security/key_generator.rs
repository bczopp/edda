//! Key Generator (Phase 3.3.1). Ed25519 key-pair generation for device identity.

use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("key generation failed")]
pub struct KeyGenerationError;

/// Ed25519 key pair (public key + secret key seed) for device identity.
#[derive(Clone)]
pub struct Ed25519KeyPair {
    public_key: [u8; 32],
    secret_key: [u8; 32],
}

impl Ed25519KeyPair {
    pub fn public_key(&self) -> &[u8; 32] {
        &self.public_key
    }

    pub fn secret_key(&self) -> &[u8; 32] {
        &self.secret_key
    }
}

/// Generates key pairs (Ed25519) for device identity at first start.
pub struct KeyGenerator;

impl KeyGenerator {
    pub fn new() -> Self {
        Self
    }

    /// Generates a new Ed25519 key pair. Use at first device start.
    pub fn generate_ed25519() -> Result<Ed25519KeyPair, KeyGenerationError> {
        let signing_key = SigningKey::generate(&mut OsRng);
        let public_key = signing_key.verifying_key();
        Ok(Ed25519KeyPair {
            public_key: public_key.to_bytes(),
            secret_key: signing_key.to_bytes(),
        })
    }
}

impl Default for KeyGenerator {
    fn default() -> Self {
        Self::new()
    }
}
