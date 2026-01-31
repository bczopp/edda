use chrono::{Utc, Duration};
use std::sync::Arc;
use uuid::Uuid;
use crate::token::validator::TokenPayload;
use crate::utils::config::TokenConfiguration;
use crate::keys::SignatureManager;
use ring::signature::Ed25519KeyPair;
use base64::{Engine as _, engine::general_purpose};

pub struct TokenGenerator {
    signing_key: Arc<Ed25519KeyPair>,
    config: TokenConfiguration,
}

impl TokenGenerator {
    pub fn new(signing_key: Arc<Ed25519KeyPair>, config: TokenConfiguration) -> Self {
        Self {
            signing_key,
            config,
        }
    }

    pub fn generate_heimdall_token(
        &self,
        device_id: &str,
        user_id: &str,
        permissions: Vec<String>,
    ) -> Result<(String, String, i64), Box<dyn std::error::Error>> {
        let token_id = Uuid::new_v4().to_string();
        let issued_at = Utc::now();
        let expires_at = issued_at + Duration::hours(self.config.heimdall_token_expiration_hours as i64);

        let payload = TokenPayload {
            token_id: token_id.clone(),
            device_id: device_id.to_string(),
            user_id: user_id.to_string(),
            token_type: "heimdall".to_string(),
            issued_at: issued_at.timestamp(),
            expires_at: expires_at.timestamp(),
            permissions,
        };

        let payload_json = serde_json::to_string(&payload)?;
        let payload_bytes = payload_json.as_bytes();

        // Sign token
        let signature = SignatureManager::sign(self.signing_key.as_ref(), payload_bytes)?;
        
        // Combine payload and signature
        let token_data = format!("{}.{}", general_purpose::STANDARD.encode(payload_bytes), general_purpose::STANDARD.encode(&signature));
        
        Ok((token_data, token_id, expires_at.timestamp()))
    }

    pub fn generate_session_token(
        &self,
        device_id: &str,
        user_id: &str,
    ) -> Result<(String, String, i64), Box<dyn std::error::Error>> {
        let token_id = Uuid::new_v4().to_string();
        let issued_at = Utc::now();
        let expires_at = issued_at + Duration::hours(self.config.session_token_expiration_hours as i64);

        let payload = TokenPayload {
            token_id: token_id.clone(),
            device_id: device_id.to_string(),
            user_id: user_id.to_string(),
            token_type: "session".to_string(),
            issued_at: issued_at.timestamp(),
            expires_at: expires_at.timestamp(),
            permissions: vec![],
        };

        let payload_json = serde_json::to_string(&payload)?;
        let payload_bytes = payload_json.as_bytes();

        // Sign token
        let signature = SignatureManager::sign(self.signing_key.as_ref(), payload_bytes)?;
        
        // Combine payload and signature
        let token_data = format!("{}.{}", general_purpose::STANDARD.encode(payload_bytes), general_purpose::STANDARD.encode(&signature));
        
        Ok((token_data, token_id, expires_at.timestamp()))
    }

    pub fn generate_refresh_token(
        &self,
        device_id: &str,
        user_id: &str,
    ) -> Result<(String, String, i64), Box<dyn std::error::Error>> {
        let token_id = Uuid::new_v4().to_string();
        let issued_at = Utc::now();
        let expires_at = issued_at + Duration::days(self.config.refresh_token_expiration_days as i64);

        let payload = TokenPayload {
            token_id: token_id.clone(),
            device_id: device_id.to_string(),
            user_id: user_id.to_string(),
            token_type: "refresh".to_string(),
            issued_at: issued_at.timestamp(),
            expires_at: expires_at.timestamp(),
            permissions: vec![],
        };

        let payload_json = serde_json::to_string(&payload)?;
        let payload_bytes = payload_json.as_bytes();

        // Sign token
        let signature = SignatureManager::sign(self.signing_key.as_ref(), payload_bytes)?;
        
        // Combine payload and signature
        let token_data = format!("{}.{}", general_purpose::STANDARD.encode(payload_bytes), general_purpose::STANDARD.encode(&signature));
        
        Ok((token_data, token_id, expires_at.timestamp()))
    }
}
