use crate::auth::ChallengeGenerator;
use crate::token::TokenGenerator;
use crate::utils::{DeviceRepository, TokenRepository};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthenticationError {
    #[error("Challenge error: {0}")]
    ChallengeError(#[from] crate::auth::ChallengeError),
    #[error("Token generation error: {0}")]
    TokenGenerationError(String),
    #[error("Device not found")]
    DeviceNotFound,
}

pub struct AuthenticationManager {
    challenge_generator: Arc<ChallengeGenerator>,
    token_generator: Arc<TokenGenerator>,
    device_repo: Arc<DeviceRepository>,
    token_repo: Arc<TokenRepository>,
    pool: PgPool,
}

impl AuthenticationManager {
    pub fn new(
        challenge_generator: Arc<ChallengeGenerator>,
        token_generator: Arc<TokenGenerator>,
        device_repo: Arc<DeviceRepository>,
        token_repo: Arc<TokenRepository>,
        pool: PgPool,
    ) -> Self {
        Self {
            challenge_generator,
            token_generator,
            device_repo,
            token_repo,
            pool,
        }
    }

    pub async fn request_challenge(
        &self,
        device_id: &str,
        public_key: &[u8],
        signature: &[u8],
    ) -> Result<(String, i64), AuthenticationError> {
        self.challenge_generator
            .generate_challenge(device_id, public_key, signature)
            .await
            .map_err(AuthenticationError::ChallengeError)
    }

    pub async fn prove_identity(
        &self,
        device_id: &str,
        challenge: &str,
        proof: &[u8],
    ) -> Result<(String, String, i64, String, i64, Vec<String>), AuthenticationError> {
        // Validate proof
        let valid = self.challenge_generator
            .validate_proof(device_id, challenge, proof)
            .await
            .map_err(AuthenticationError::ChallengeError)?;

        if !valid {
            return Err(AuthenticationError::ChallengeError(
                crate::auth::ChallengeError::InvalidSignature,
            ));
        }

        // Get device
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| AuthenticationError::DeviceNotFound)?;

        // Get permissions from device
        let permissions = self.get_device_permissions(&device.id).await?;

        // Generate tokens
        let (heimdall_token, token_id, expires_at) = self.token_generator
            .generate_heimdall_token(
                &device.device_id,
                &device.user_id.to_string(),
                permissions.clone(),
            )
            .map_err(|e| AuthenticationError::TokenGenerationError(format!("{}", e)))?;

        let (refresh_token, refresh_token_id, refresh_expires_at) = self.token_generator
            .generate_refresh_token(
                &device.device_id,
                &device.user_id.to_string(),
            )
            .map_err(|e| AuthenticationError::TokenGenerationError(format!("{}", e)))?;

        // Store tokens in database
        use chrono::TimeZone;
        let expires_at_dt = chrono::Utc.timestamp_opt(expires_at, 0).unwrap();
        let refresh_expires_at_dt = chrono::Utc.timestamp_opt(refresh_expires_at, 0).unwrap();

        self.token_repo.create(
            &token_id,
            device.id,
            device.user_id,
            "heimdall",
            &heimdall_token,
            expires_at_dt,
        ).await.map_err(|e| AuthenticationError::TokenGenerationError(format!("{}", e)))?;

        self.token_repo.create(
            &refresh_token_id,
            device.id,
            device.user_id,
            "refresh",
            &refresh_token,
            refresh_expires_at_dt,
        ).await.map_err(|e| AuthenticationError::TokenGenerationError(format!("{}", e)))?;

        Ok((heimdall_token, token_id, expires_at, refresh_token, refresh_expires_at, permissions))
    }

    async fn get_device_permissions(&self, device_id: &Uuid) -> Result<Vec<String>, AuthenticationError> {
        use sqlx::Row;
        
        let rows = sqlx::query(
            r#"
            SELECT p.permission_name
            FROM device_permissions dp
            JOIN permissions p ON dp.permission_id = p.id
            WHERE dp.device_id = $1
            UNION
            SELECT p.permission_name
            FROM device_roles dr
            JOIN role_permissions rp ON dr.role_id = rp.role_id
            JOIN permissions p ON rp.permission_id = p.id
            WHERE dr.device_id = $1
            "#,
        )
        .bind(device_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AuthenticationError::TokenGenerationError("Failed to fetch permissions".to_string()))?;

        Ok(rows.iter()
            .map(|row| row.get::<String, _>("permission_name"))
            .collect())
    }
}
