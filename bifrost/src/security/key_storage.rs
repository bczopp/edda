//! Key Storage (Phase 3.3.2). Store public key unencrypted, private key encrypted; key loading.

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm,
};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use thiserror::Error;
use tokio::fs;
use tokio::io::AsyncWriteExt;

#[derive(Error, Debug)]
pub enum KeyStorageError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("decryption failed (wrong passphrase or corrupted)")]
    Decrypt,
}

/// Stores public key unencrypted and private key encrypted (passphrase-based). Key loading.
pub struct KeyStorage {
    base_path: PathBuf,
}

impl KeyStorage {
    pub fn new(base_path: PathBuf) -> Self {
        Self { base_path }
    }

    fn public_key_path(&self, key_id: &str) -> PathBuf {
        self.base_path.join(format!("{}.pub", key_id))
    }

    fn private_key_path(&self, key_id: &str) -> PathBuf {
        self.base_path.join(format!("{}.key", key_id))
    }

    fn derive_key(passphrase: &str) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(passphrase.as_bytes());
        let result = hasher.finalize();
        let mut key = [0u8; 32];
        key.copy_from_slice(&result);
        key
    }

    /// Saves public key unencrypted to {base_path}/{key_id}.pub
    pub async fn save_public_key(&self, key_id: &str, public_key: &[u8]) -> Result<(), KeyStorageError> {
        fs::create_dir_all(&self.base_path).await?;
        let path = self.public_key_path(key_id);
        fs::write(path, public_key).await?;
        Ok(())
    }

    /// Loads public key from {base_path}/{key_id}.pub
    pub async fn load_public_key(&self, key_id: &str) -> Result<Vec<u8>, KeyStorageError> {
        let path = self.public_key_path(key_id);
        let bytes = fs::read(path).await?;
        Ok(bytes)
    }

    /// Saves private key encrypted with passphrase to {base_path}/{key_id}.key (nonce + ciphertext)
    pub async fn save_private_key(
        &self,
        key_id: &str,
        private_key: &[u8],
        passphrase: &str,
    ) -> Result<(), KeyStorageError> {
        fs::create_dir_all(&self.base_path).await?;
        let key = Self::derive_key(passphrase);
        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| KeyStorageError::Decrypt)?;
        let nonce = Aes256Gcm::generate_nonce(&mut rand::rngs::OsRng);
        let ciphertext = cipher
            .encrypt(&nonce, private_key)
            .map_err(|_| KeyStorageError::Decrypt)?;
        let path = self.private_key_path(key_id);
        let mut file = fs::File::create(path).await?;
        file.write_all(nonce.as_slice()).await?;
        file.write_all(&ciphertext).await?;
        file.flush().await?;
        Ok(())
    }

    /// Loads and decrypts private key with passphrase.
    pub async fn load_private_key(
        &self,
        key_id: &str,
        passphrase: &str,
    ) -> Result<Vec<u8>, KeyStorageError> {
        let path = self.private_key_path(key_id);
        let bytes = fs::read(path).await?;
        if bytes.len() < 12 {
            return Err(KeyStorageError::Decrypt);
        }
        let (nonce_slice, ciphertext) = bytes.split_at(12);
        let key = Self::derive_key(passphrase);
        let cipher = Aes256Gcm::new_from_slice(&key).map_err(|_| KeyStorageError::Decrypt)?;
        let nonce = aes_gcm::Nonce::from_slice(nonce_slice);
        let plaintext = cipher
            .decrypt(&nonce, ciphertext)
            .map_err(|_| KeyStorageError::Decrypt)?;
        Ok(plaintext)
    }
}
