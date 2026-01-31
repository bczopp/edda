use ring::signature::{Ed25519KeyPair, KeyPair};
use ring::aead::{Aad, BoundKey, Nonce, NonceSequence, OpeningKey, SealingKey, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use std::path::PathBuf;
use std::fs;
use thiserror::Error;
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Error)]
pub enum KeyStorageError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Key encoding error: {0}")]
    EncodingError(String),
    #[error("Key decoding error: {0}")]
    DecodingError(String),
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    #[error("Decryption error: {0}")]
    DecryptionError(String),
}

/// Generates encryption key from a master key using PBKDF2
fn derive_encryption_key(master_key: &[u8], salt: &[u8]) -> Result<[u8; 32], KeyStorageError> {
    // Use PBKDF2 to derive a 32-byte key
    let mut key = [0u8; 32];
    ring::pbkdf2::derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA256,
        std::num::NonZeroU32::new(100000).unwrap(), // 100k iterations
        salt,
        master_key,
        &mut key,
    );
    Ok(key)
}

/// Gets or generates master key for encryption
fn get_master_key(keys_dir: &PathBuf) -> Result<[u8; 32], KeyStorageError> {
    let master_key_path = keys_dir.join(".master_key");
    
    if master_key_path.exists() {
        // Load existing master key
        let master_key_b64 = fs::read_to_string(&master_key_path)?;
        let master_key_bytes = general_purpose::STANDARD
            .decode(master_key_b64.trim())
            .map_err(|e| KeyStorageError::DecodingError(format!("{}", e)))?;
        
        if master_key_bytes.len() != 32 {
            return Err(KeyStorageError::DecodingError("Invalid master key length".to_string()));
        }
        
        let mut key = [0u8; 32];
        key.copy_from_slice(&master_key_bytes);
        Ok(key)
    } else {
        // Generate new master key
        let rng = SystemRandom::new();
        let mut master_key = [0u8; 32];
        rng.fill(&mut master_key)
            .map_err(|e| KeyStorageError::EncryptionError(format!("Failed to generate master key: {}", e)))?;
        
        // Store master key (in production, this should be in secure storage)
        let master_key_b64 = general_purpose::STANDARD.encode(&master_key);
        fs::write(&master_key_path, master_key_b64)?;
        
        // Set restrictive permissions (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&master_key_path, fs::Permissions::from_mode(0o600))?;
        }
        
        Ok(master_key)
    }
}

struct CounterNonce {
    value: u64,
}

impl CounterNonce {
    fn new() -> Self {
        Self { value: 0 }
    }
}

impl NonceSequence for CounterNonce {
    fn advance(&mut self) -> Result<Nonce, ring::error::Unspecified> {
        let mut nonce_bytes = [0u8; 12];
        nonce_bytes[4..].copy_from_slice(&self.value.to_be_bytes());
        self.value += 1;
        Ok(Nonce::assume_unique_for_key(nonce_bytes))
    }
}

pub struct SecureKeyStorage {
    keys_dir: PathBuf,
}

impl SecureKeyStorage {
    pub fn new(keys_dir: PathBuf) -> Self {
        Self { keys_dir }
    }

    /// Store private key (encrypted) and public key (unencrypted). Requires PKCS#8 bytes (ring 0.17 has no pkcs8_bytes() on keypair).
    pub fn store_keypair(
        &self,
        key_id: &str,
        pkcs8_bytes: &[u8],
    ) -> Result<(), KeyStorageError> {
        let keypair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes)
            .map_err(|e| KeyStorageError::DecodingError(format!("{}", e)))?;

        // Ensure keys directory exists
        fs::create_dir_all(&self.keys_dir)?;

        // Store public key (unencrypted)
        let public_key = keypair.public_key();
        let public_key_path = self.keys_dir.join(format!("{}.pub", key_id));
        let public_key_b64 = general_purpose::STANDARD.encode(public_key.as_ref());
        fs::write(public_key_path, public_key_b64)?;

        // Store private key (encrypted with AES-256-GCM)
        let private_key_path = self.keys_dir.join(format!("{}.key", key_id));
        
        // Get master key
        let master_key = get_master_key(&self.keys_dir)?;
        
        // Generate salt for this key
        let rng = SystemRandom::new();
        let mut salt = [0u8; 16];
        rng.fill(&mut salt)
            .map_err(|e| KeyStorageError::EncryptionError(format!("Failed to generate salt: {}", e)))?;
        
        // Derive encryption key
        let encryption_key = derive_encryption_key(&master_key, &salt)?;
        
        // Encrypt private key
        let unbound_key = UnboundKey::new(&AES_256_GCM, &encryption_key)
            .map_err(|e| KeyStorageError::EncryptionError(format!("Failed to create encryption key: {:?}", e)))?;
        
        let nonce_seq = CounterNonce::new();
        let mut sealing_key = SealingKey::new(unbound_key, nonce_seq);
        
        let mut in_out = pkcs8_bytes.to_vec();
        let tag = sealing_key.seal_in_place_separate_tag(Aad::empty(), &mut in_out)
            .map_err(|e| KeyStorageError::EncryptionError(format!("Encryption failed: {:?}", e)))?;
        
        // Combine salt + encrypted data + tag
        let mut encrypted_data = Vec::new();
        encrypted_data.extend_from_slice(&salt);
        encrypted_data.extend_from_slice(&in_out);
        encrypted_data.extend_from_slice(tag.as_ref());
        
        // Store encrypted key
        let encrypted_b64 = general_purpose::STANDARD.encode(&encrypted_data);
        fs::write(private_key_path, encrypted_b64)?;

        Ok(())
    }

    /// Copy key files to another key_id (e.g. current -> deprecated for rotation).
    pub fn copy_key(&self, from_id: &str, to_id: &str) -> Result<(), KeyStorageError> {
        for ext in &[".pub", ".key"] {
            let from_path = self.keys_dir.join(format!("{}{}", from_id, ext));
            let to_path = self.keys_dir.join(format!("{}{}", to_id, ext));
            if from_path.exists() {
                fs::copy(&from_path, &to_path)?;
            }
        }
        Ok(())
    }

    /// Load public key
    pub fn load_public_key(&self, key_id: &str) -> Result<Vec<u8>, KeyStorageError> {
        let public_key_path = self.keys_dir.join(format!("{}.pub", key_id));
        let public_key_b64 = fs::read_to_string(public_key_path)?;
        let public_key_bytes = general_purpose::STANDARD
            .decode(public_key_b64)
            .map_err(|e| KeyStorageError::DecodingError(format!("{}", e)))?;
        Ok(public_key_bytes)
    }

    /// Load private key (decrypt)
    pub fn load_keypair(&self, key_id: &str) -> Result<Ed25519KeyPair, KeyStorageError> {
        let private_key_path = self.keys_dir.join(format!("{}.key", key_id));
        let private_key_b64 = fs::read_to_string(private_key_path)?;
        let encrypted_data = general_purpose::STANDARD
            .decode(private_key_b64.trim())
            .map_err(|e| KeyStorageError::DecodingError(format!("{}", e)))?;
        
        // Check if this is old format (unencrypted) or new format (encrypted)
        // Try to parse as PKCS8 first (old format)
        if let Ok(keypair) = Ed25519KeyPair::from_pkcs8(&encrypted_data) {
            return Ok(keypair);
        }
        
        // New encrypted format: salt (16 bytes) + encrypted data + tag (16 bytes)
        if encrypted_data.len() < 32 {
            return Err(KeyStorageError::DecryptionError("Invalid encrypted key format".to_string()));
        }
        
        let salt = &encrypted_data[0..16];
        let tag_start = encrypted_data.len() - 16;
        let encrypted_bytes = &encrypted_data[16..tag_start];
        let tag_bytes = &encrypted_data[tag_start..];
        
        // Get master key
        let master_key = get_master_key(&self.keys_dir)?;
        
        // Derive decryption key
        let decryption_key = derive_encryption_key(&master_key, salt)?;
        
        // Decrypt private key
        let unbound_key = UnboundKey::new(&AES_256_GCM, &decryption_key)
            .map_err(|e| KeyStorageError::DecryptionError(format!("Failed to create decryption key: {:?}", e)))?;
        
        let nonce_seq = CounterNonce::new();
        let mut opening_key = OpeningKey::new(unbound_key, nonce_seq);
        
        let mut in_out = encrypted_bytes.to_vec();
        in_out.extend_from_slice(tag_bytes);
        
        let decrypted_bytes = opening_key.open_in_place(Aad::empty(), &mut in_out)
            .map_err(|e| KeyStorageError::DecryptionError(format!("Decryption failed: {:?}", e)))?;
        
        Ed25519KeyPair::from_pkcs8(decrypted_bytes)
            .map_err(|e| KeyStorageError::DecodingError(format!("Failed to parse keypair: {:?}", e)))
    }
}
