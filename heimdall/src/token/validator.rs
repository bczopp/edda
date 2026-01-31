use chrono::Utc;
use crate::keys::{SignatureManager, SecureKeyStorage};
use base64::{Engine as _, engine::general_purpose};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenPayload {
    pub token_id: String,
    pub device_id: String,
    pub user_id: String,
    pub token_type: String,
    pub issued_at: i64,
    pub expires_at: i64,
    pub permissions: Vec<String>,
}

#[derive(Debug, Error)]
pub enum TokenValidationError {
    #[error("Invalid token format")]
    InvalidFormat,
    #[error("Token expired")]
    Expired,
    #[error("Invalid signature: {0}")]
    InvalidSignature(String),
    #[error("Token not found in database")]
    NotFound,
    #[error("Token revoked")]
    Revoked,
    #[error("Key storage error: {0}")]
    KeyStorageError(String),
}

pub struct TokenValidator {
    key_storage: SecureKeyStorage,
    cache: Option<Arc<crate::utils::TokenValidationCache>>,
}

impl TokenValidator {
    pub fn new(keys_dir: PathBuf) -> Self {
        Self {
            key_storage: SecureKeyStorage::new(keys_dir),
            cache: None,
        }
    }

    pub fn with_cache(keys_dir: PathBuf, cache: Arc<crate::utils::TokenValidationCache>) -> Self {
        Self {
            key_storage: SecureKeyStorage::new(keys_dir),
            cache: Some(cache),
        }
    }

    pub async fn validate_token(&self, token: &str) -> Result<TokenPayload, TokenValidationError> {
        // Parse token (format: base64(payload).base64(signature))
        let parts: Vec<&str> = token.split('.').collect();
        if parts.len() != 2 {
            return Err(TokenValidationError::InvalidFormat);
        }

        let payload_bytes = general_purpose::STANDARD
            .decode(parts[0])
            .map_err(|_| TokenValidationError::InvalidFormat)?;
        
        let signature_bytes = general_purpose::STANDARD
            .decode(parts[1])
            .map_err(|_| TokenValidationError::InvalidFormat)?;

        // Deserialize payload
        let payload: TokenPayload = serde_json::from_slice(&payload_bytes)
            .map_err(|_| TokenValidationError::InvalidFormat)?;

        // Check cache first
        if let Some(ref cache) = self.cache {
            if let Some(cached_valid) = cache.get(&payload.token_id).await {
                if cached_valid {
                    // Check expiration even for cached tokens
                    let now = Utc::now().timestamp();
                    if now > payload.expires_at {
                        cache.invalidate(&payload.token_id).await;
                        return Err(TokenValidationError::Expired);
                    }
                    return Ok(payload);
                } else {
                    return Err(TokenValidationError::InvalidSignature("Cached invalid".to_string()));
                }
            }
        }

        // Check expiration
        let now = Utc::now().timestamp();
        if now > payload.expires_at {
            return Err(TokenValidationError::Expired);
        }

        // Verify signature (load Heimdall public key)
        let public_key = self.key_storage
            .load_public_key("heimdall")
            .map_err(|e| TokenValidationError::KeyStorageError(format!("{}", e)))?;

        let is_valid = SignatureManager::verify(&public_key, &payload_bytes, &signature_bytes).is_ok();

        // Cache result
        if let Some(ref cache) = self.cache {
            cache.set(payload.token_id.clone(), is_valid).await;
        }

        if !is_valid {
            return Err(TokenValidationError::InvalidSignature("Signature verification failed".to_string()));
        }

        Ok(payload)
    }
}
