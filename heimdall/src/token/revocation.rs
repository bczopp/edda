//! Token revocation: immediate revoke, revocation list (DB), cache invalidation.

use crate::utils::token_repository::TokenRepository;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenRevocationError {
    #[error("Database error: {0}")]
    Database(#[from] crate::utils::token_repository::TokenRepositoryError),
}

/// Manages token revocation: immediate revoke in DB, revocation list, optional cache invalidation.
pub struct TokenRevocationManager {
    token_repo: Arc<TokenRepository>,
    cache: Option<Arc<crate::utils::TokenValidationCache>>,
}

impl TokenRevocationManager {
    pub fn new(
        token_repo: Arc<TokenRepository>,
        cache: Option<Arc<crate::utils::TokenValidationCache>>,
    ) -> Self {
        Self { token_repo, cache }
    }

    /// Revoke token immediately (DB + cache invalidation).
    pub async fn revoke(&self, token_id: &str) -> Result<(), TokenRevocationError> {
        self.token_repo.revoke(token_id).await?;
        if let Some(ref cache) = self.cache {
            cache.invalidate(token_id).await;
        }
        Ok(())
    }

    /// Returns true if the token is revoked (or not found in DB).
    pub async fn is_revoked(&self, token_id: &str) -> Result<bool, TokenRevocationError> {
        match self.token_repo.get_by_token_id(token_id).await {
            Ok(token) => Ok(token.is_revoked),
            Err(_) => Ok(false),
        }
    }
}
