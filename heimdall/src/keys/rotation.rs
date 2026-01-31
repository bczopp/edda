//! Key rotation: automatic rotation, event-based rotation, grace-period rollover.

use crate::keys::{KeyGenerator, KeyGenerationError, SecureKeyStorage};
use ring::signature::Ed25519KeyPair;
use std::path::PathBuf;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyRotationError {
    #[error("Key generation failed: {0}")]
    Generation(#[from] KeyGenerationError),
    #[error("Key storage error: {0}")]
    Storage(#[from] crate::keys::KeyStorageError),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid rotation metadata: {0}")]
    Metadata(String),
}

const DEPRECATED_SUFFIX: &str = ".deprecated";
const ROTATED_AT_PREFIX: &str = ".rotated_at.";

/// Manages key rotation: automatic interval, event-based, and grace-period rollover.
pub struct KeyRotationManager {
    key_generator: KeyGenerator,
    key_storage: SecureKeyStorage,
    keys_dir: PathBuf,
    rotation_interval: Duration,
    grace_period: Duration,
}

impl KeyRotationManager {
    pub fn new(
        key_generator: KeyGenerator,
        key_storage: SecureKeyStorage,
        keys_dir: PathBuf,
        rotation_interval: Duration,
        grace_period: Duration,
    ) -> Self {
        Self {
            key_generator,
            key_storage,
            keys_dir,
            rotation_interval,
            grace_period,
        }
    }

    /// Rotate key: generate new keypair, copy current to deprecated, store new as current.
    pub fn rotate_key(&self, key_id: &str) -> Result<(), KeyRotationError> {
        if self.key_storage.load_keypair(key_id).is_ok() {
            let deprecated_id = format!("{}{}", key_id, DEPRECATED_SUFFIX);
            self.key_storage.copy_key(key_id, &deprecated_id)?;
        }

        let (_new_key, new_pkcs8) = self.key_generator.generate_ed25519_keypair()?;
        self.key_storage.store_keypair(key_id, &new_pkcs8)?;
        self.write_rotated_at(key_id)?;
        Ok(())
    }

    /// Current keypair for `key_id` (active key).
    pub fn get_current_keypair(&self, key_id: &str) -> Result<Option<Ed25519KeyPair>, KeyRotationError> {
        match self.key_storage.load_keypair(key_id) {
            Ok(kp) => Ok(Some(kp)),
            Err(_) => Ok(None),
        }
    }

    /// Deprecated keypair during grace period (for verification rollover).
    pub fn get_deprecated_keypair(&self, key_id: &str) -> Result<Option<Ed25519KeyPair>, KeyRotationError> {
        let deprecated_id = format!("{}{}", key_id, DEPRECATED_SUFFIX);
        match self.key_storage.load_keypair(&deprecated_id) {
            Ok(kp) => Ok(Some(kp)),
            Err(_) => Ok(None),
        }
    }

    /// Remove deprecated key and metadata if grace period has passed.
    pub fn cleanup_deprecated(&self, key_id: &str) -> Result<(), KeyRotationError> {
        let rotated_at = match self.read_rotated_at(key_id)? {
            Some(t) => t,
            None => return Ok(()),
        };
        let now_secs = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|e| KeyRotationError::Metadata(format!("system time error: {}", e)))?
            .as_secs();
        if now_secs.saturating_sub(rotated_at) < self.grace_period.as_secs() {
            return Ok(());
        }
        let deprecated_id = format!("{}{}", key_id, DEPRECATED_SUFFIX);
        let _ = std::fs::remove_file(self.keys_dir.join(format!("{}.pub", deprecated_id)));
        let _ = std::fs::remove_file(self.keys_dir.join(format!("{}.key", deprecated_id)));
        Ok(())
    }

    /// True if key should be rotated (no key yet, or last rotation older than interval).
    pub fn should_rotate(&self, key_id: &str) -> Result<bool, KeyRotationError> {
        if self.get_current_keypair(key_id)?.is_none() {
            return Ok(true);
        }
        let rotated_at = match self.read_rotated_at(key_id)? {
            Some(t) => t,
            None => return Ok(false),
        };
        let now_secs = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|e| KeyRotationError::Metadata(format!("system time error: {}", e)))?
            .as_secs();
        Ok(now_secs.saturating_sub(rotated_at) >= self.rotation_interval.as_secs())
    }

    fn rotated_at_path(&self, key_id: &str) -> PathBuf {
        self.keys_dir.join(format!("{}{}", ROTATED_AT_PREFIX, key_id))
    }

    fn write_rotated_at(&self, key_id: &str) -> Result<(), KeyRotationError> {
        let now_secs = SystemTime::now().duration_since(UNIX_EPOCH)
            .map_err(|e| KeyRotationError::Metadata(format!("system time error: {}", e)))?
            .as_secs();
        std::fs::write(self.rotated_at_path(key_id), now_secs.to_string())?;
        Ok(())
    }

    fn read_rotated_at(&self, key_id: &str) -> Result<Option<u64>, KeyRotationError> {
        let path = self.rotated_at_path(key_id);
        if !path.exists() {
            return Ok(None);
        }
        let s = std::fs::read_to_string(&path)?;
        let t = s.trim().parse::<u64>().map_err(|e| {
            KeyRotationError::Metadata(format!("invalid timestamp: {}", e))
        })?;
        Ok(Some(t))
    }
}
