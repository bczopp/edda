use ring::aead;
use ring::rand::{SecureRandom, SystemRandom};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EncryptionError {
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),
    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),
    #[error("Invalid encrypted data format")]
    InvalidFormat,
}

pub struct EncryptionManager {
    key: aead::LessSafeKey,
    rng: SystemRandom,
}

impl EncryptionManager {
    pub fn new(key_bytes: &[u8]) -> Result<Self, EncryptionError> {
        if key_bytes.len() != 32 {
            return Err(EncryptionError::EncryptionFailed("Key must be 32 bytes for AES-256".to_string()));
        }
        let key = aead::UnboundKey::new(&aead::AES_256_GCM, key_bytes)
            .map_err(|e| EncryptionError::EncryptionFailed(format!("{}", e)))?;
        Ok(Self {
            key: aead::LessSafeKey::new(key),
            rng: SystemRandom::new(),
        })
    }

    /// Encrypts data with a random nonce. Returns encrypted data with nonce prepended.
    /// Format: [12-byte nonce][encrypted data with tag]
    pub fn encrypt(&self, data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        // Generate random nonce (12 bytes for AES-GCM)
        let mut nonce_bytes = [0u8; 12];
        self.rng.fill(&mut nonce_bytes)
            .map_err(|e| EncryptionError::EncryptionFailed(format!("Failed to generate nonce: {}", e)))?;
        
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
        let mut in_out = data.to_vec();
        
        self.key.seal_in_place_append_tag(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|e| EncryptionError::EncryptionFailed(format!("{}", e)))?;
        
        // Prepend nonce to encrypted data
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&in_out);
        Ok(result)
    }

    /// Decrypts data. Expects format: [12-byte nonce][encrypted data with tag]
    pub fn decrypt(&self, encrypted_data: &[u8]) -> Result<Vec<u8>, EncryptionError> {
        if encrypted_data.len() < 12 {
            return Err(EncryptionError::InvalidFormat);
        }
        
        // Extract nonce (first 12 bytes)
        let nonce_bytes: [u8; 12] = encrypted_data[0..12].try_into()
            .map_err(|_| EncryptionError::InvalidFormat)?;
        let nonce = aead::Nonce::assume_unique_for_key(nonce_bytes);
        
        // Extract encrypted data (rest of the bytes)
        let mut in_out = encrypted_data[12..].to_vec();
        
        self.key.open_in_place(nonce, aead::Aad::empty(), &mut in_out)
            .map_err(|e| EncryptionError::DecryptionFailed(format!("{}", e)))?;
        
        Ok(in_out)
    }
}
