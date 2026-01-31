use crate::keys::{KeyGenerator, SignatureManager, SecureKeyStorage};
use crate::utils::device_repository::DeviceRepository;
use std::path::PathBuf;
use std::time::Duration;
use chrono::Utc;
use thiserror::Error;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Error)]
pub enum ChallengeError {
    #[error("Invalid request signature")]
    InvalidSignature,
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Key storage error: {0}")]
    KeyStorageError(String),
    #[error("Challenge generation failed: {0}")]
    GenerationFailed(String),
}

pub struct Challenge {
    pub challenge: String,
    pub device_id: String,
    pub expires_at: i64,
}

#[allow(dead_code)]
pub struct ChallengeGenerator {
    key_generator: KeyGenerator,
    key_storage: SecureKeyStorage,
    device_repo: Arc<DeviceRepository>,
    challenges: Arc<RwLock<HashMap<String, Challenge>>>,
    challenge_ttl: Duration,
}

impl ChallengeGenerator {
    pub fn new(
        keys_dir: PathBuf,
        device_repo: Arc<DeviceRepository>,
    ) -> Self {
        Self {
            key_generator: KeyGenerator::new(),
            key_storage: SecureKeyStorage::new(keys_dir),
            device_repo,
            challenges: Arc::new(RwLock::new(HashMap::new())),
            challenge_ttl: Duration::from_secs(300), // 5 minutes
        }
    }

    pub async fn generate_challenge(
        &self,
        device_id: &str,
        public_key: &[u8],
        signature: &[u8],
    ) -> Result<(String, i64), ChallengeError> {
        // Verify request signature
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| ChallengeError::DeviceNotFound)?;

        let device_public_key_bytes = general_purpose::STANDARD
            .decode(&device.public_key)
            .map_err(|_| ChallengeError::KeyStorageError("Invalid device public key format".to_string()))?;

        // Verify that provided public_key matches device public_key
        if device_public_key_bytes != public_key {
            return Err(ChallengeError::InvalidSignature);
        }

        // Verify signature of device_id + public_key
        let signed_data = format!("{}.{}", device_id, general_purpose::STANDARD.encode(public_key));
        SignatureManager::verify(&device_public_key_bytes, signed_data.as_bytes(), signature)
            .map_err(|_| ChallengeError::InvalidSignature)?;
        
        // Generate random challenge
        let challenge_bytes = self.key_generator
            .generate_random_bytes(32)
            .map_err(|e| ChallengeError::GenerationFailed(format!("{}", e)))?;
        
        let challenge = hex::encode(challenge_bytes);
        let expires_at = Utc::now() + chrono::Duration::seconds(self.challenge_ttl.as_secs() as i64);
        
        // Store challenge
        let challenge_obj = Challenge {
            challenge: challenge.clone(),
            device_id: device_id.to_string(),
            expires_at: expires_at.timestamp(),
        };
        
        self.challenges.write().await.insert(device_id.to_string(), challenge_obj);
        
        Ok((challenge, expires_at.timestamp()))
    }

    pub async fn validate_proof(
        &self,
        device_id: &str,
        challenge: &str,
        proof: &[u8],
    ) -> Result<bool, ChallengeError> {
        // Get stored challenge
        let challenges = self.challenges.read().await;
        let stored_challenge = challenges
            .get(device_id)
            .ok_or(ChallengeError::DeviceNotFound)?;

        // Check challenge matches
        if stored_challenge.challenge != challenge {
            return Ok(false);
        }

        // Check expiration
        let now = Utc::now().timestamp();
        if now > stored_challenge.expires_at {
            return Ok(false);
        }

        // Verify proof signature with device public key
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| ChallengeError::DeviceNotFound)?;

        let public_key_bytes = general_purpose::STANDARD
            .decode(&device.public_key)
            .map_err(|_| ChallengeError::KeyStorageError("Invalid public key format".to_string()))?;

        let challenge_bytes = challenge.as_bytes();
        SignatureManager::verify(&public_key_bytes, challenge_bytes, proof)
            .map_err(|_e| ChallengeError::InvalidSignature)?;

        // Remove challenge after validation
        drop(challenges);
        self.challenges.write().await.remove(device_id);

        Ok(true)
    }
}
