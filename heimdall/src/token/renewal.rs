//! Token renewal: proactive renewal (before expiry), renewal via refresh token.

use crate::token::validator::TokenPayload;
use crate::token::{TokenGenerator, TokenValidator};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenRenewalError {
    #[error("Invalid refresh token: {0}")]
    InvalidRefreshToken(String),
    #[error("Refresh token expired")]
    Expired,
    #[error("Token validation error")]
    Validation(#[from] crate::token::TokenValidationError),
}

/// Manages token renewal: proactive (before expiry) and via refresh token.
pub struct TokenRenewalManager {
    validator: Arc<TokenValidator>,
    generator: TokenGenerator,
    proactive_renewal_seconds: u64,
}

impl TokenRenewalManager {
    pub fn new(
        validator: Arc<TokenValidator>,
        generator: TokenGenerator,
        proactive_renewal_seconds: u64,
    ) -> Self {
        Self {
            validator,
            generator,
            proactive_renewal_seconds,
        }
    }

    /// Returns true if the token should be renewed (expires within proactive threshold).
    pub fn should_renew(&self, payload: &TokenPayload) -> bool {
        let now = chrono::Utc::now().timestamp();
        let secs_until_expiry = payload.expires_at.saturating_sub(now);
        secs_until_expiry <= self.proactive_renewal_seconds as i64
    }

    /// Validates the refresh token and issues a new Heimdall token (same device_id, user_id).
    pub async fn renew_heimdall_with_refresh_token(
        &self,
        refresh_token: &str,
    ) -> Result<(String, String, i64), TokenRenewalError> {
        let payload = self
            .validator
            .validate_token(refresh_token)
            .await
            .map_err(TokenRenewalError::Validation)?;

        if payload.token_type != "refresh" {
            return Err(TokenRenewalError::InvalidRefreshToken(
                "token is not a refresh token".to_string(),
            ));
        }

        let (token_data, token_id, expires_at) = self
            .generator
            .generate_heimdall_token(
                &payload.device_id,
                &payload.user_id,
                payload.permissions.clone(),
            )
            .map_err(|e| TokenRenewalError::InvalidRefreshToken(e.to_string()))?;

        Ok((token_data, token_id, expires_at))
    }
}
