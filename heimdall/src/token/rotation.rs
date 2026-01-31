//! Token rotation: regular interval, event-based, automatic rotation.

use crate::token::validator::TokenPayload;
use crate::token::{TokenGenerator, TokenRevocationManager, TokenValidator};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenRotationError {
    #[error("Token validation error")]
    Validation(#[from] crate::token::TokenValidationError),
    #[error("Token generation error: {0}")]
    Generation(String),
    #[error("Token revocation error: {0}")]
    Revocation(#[from] crate::token::TokenRevocationError),
    #[error("Token is not a heimdall token")]
    NotHeimdallToken,
}

/// Manages token rotation: regular interval, event-based, automatic.
pub struct TokenRotationManager {
    validator: Arc<TokenValidator>,
    generator: TokenGenerator,
    revocation_manager: Arc<TokenRevocationManager>,
    rotation_interval_seconds: u64,
}

impl TokenRotationManager {
    pub fn new(
        validator: Arc<TokenValidator>,
        generator: TokenGenerator,
        revocation_manager: Arc<TokenRevocationManager>,
        rotation_interval_seconds: u64,
    ) -> Self {
        Self {
            validator,
            generator,
            revocation_manager,
            rotation_interval_seconds,
        }
    }

    /// Returns true if the token should be rotated (issued longer ago than rotation interval).
    pub fn should_rotate(&self, payload: &TokenPayload) -> bool {
        let now = chrono::Utc::now().timestamp();
        let age_seconds = now.saturating_sub(payload.issued_at);
        age_seconds >= self.rotation_interval_seconds as i64
    }

    /// Rotate heimdall token: validate current, issue new, revoke old. Event-based or on-demand.
    pub async fn rotate_heimdall_token(
        &self,
        current_token: &str,
    ) -> Result<(String, String, i64), TokenRotationError> {
        let payload = self
            .validator
            .validate_token(current_token)
            .await
            .map_err(TokenRotationError::Validation)?;

        if payload.token_type != "heimdall" {
            return Err(TokenRotationError::NotHeimdallToken);
        }

        let (new_token_data, new_token_id, expires_at) = self
            .generator
            .generate_heimdall_token(
                &payload.device_id,
                &payload.user_id,
                payload.permissions.clone(),
            )
            .map_err(|e| TokenRotationError::Generation(e.to_string()))?;

        self.revocation_manager.revoke(&payload.token_id).await?;

        Ok((new_token_data, new_token_id, expires_at))
    }
}
