use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::Ed25519KeyPair;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyGenerationError {
    #[error("Failed to generate key: {0}")]
    GenerationFailed(String),
    #[error("Invalid key format: {0}")]
    InvalidFormat(String),
}

pub struct KeyGenerator {
    rng: SystemRandom,
}

impl KeyGenerator {
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
        }
    }

    /// Generate Ed25519 key pair and PKCS#8 bytes (for storage; ring 0.17 has no pkcs8_bytes() on keypair).
    pub fn generate_ed25519_keypair(&self) -> Result<(Ed25519KeyPair, Vec<u8>), KeyGenerationError> {
        let pkcs8_doc = Ed25519KeyPair::generate_pkcs8(&self.rng)
            .map_err(|e| KeyGenerationError::GenerationFailed(format!("{}", e)))?;
        let pkcs8_bytes = pkcs8_doc.as_ref().to_vec();
        let keypair = Ed25519KeyPair::from_pkcs8(pkcs8_doc.as_ref())
            .map_err(|e| KeyGenerationError::InvalidFormat(format!("{}", e)))?;
        Ok((keypair, pkcs8_bytes))
    }

    /// Generate random bytes using CSPRNG
    pub fn generate_random_bytes(&self, len: usize) -> Result<Vec<u8>, KeyGenerationError> {
        let mut bytes = vec![0u8; len];
        self.rng.fill(&mut bytes)
            .map_err(|e| KeyGenerationError::GenerationFailed(format!("{}", e)))?;
        Ok(bytes)
    }
}

impl Default for KeyGenerator {
    fn default() -> Self {
        Self::new()
    }
}
