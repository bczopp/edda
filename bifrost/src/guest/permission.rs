//! Permission Token Manager (Phase 12.2.3). Generate/validate/revoke tokens after user confirmation; 24h expiry.

use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, PartialEq)]
pub enum TokenValidationError {
    #[error("token invalid or expired")]
    InvalidOrExpired,
}

/// Metadata stored with a permission token.
#[derive(Debug, Clone)]
pub struct TokenInfo {
    pub user_id: String,
    pub device_id: String,
    pub mesh_id: String,
}

struct StoredToken {
    info: TokenInfo,
    expires_at: Instant,
}

/// Generates permission tokens after user confirmation; validates and enforces expiration (e.g. 24h).
pub struct PermissionTokenManager {
    tokens: RwLock<HashMap<String, StoredToken>>,
    expiry: Duration,
}

impl PermissionTokenManager {
    pub fn new(expiry: Duration) -> Self {
        Self {
            tokens: RwLock::new(HashMap::new()),
            expiry,
        }
    }

    /// Generates a new permission token for the given user/device/mesh (call after user confirmation).
    pub fn generate(&self, user_id: &str, device_id: &str, mesh_id: &str) -> String {
        let token = format!("pt-{}", Uuid::new_v4());
        let now = Instant::now();
        let expires_at = now + self.expiry;
        let info = TokenInfo {
            user_id: user_id.to_string(),
            device_id: device_id.to_string(),
            mesh_id: mesh_id.to_string(),
        };
        self.tokens
            .write()
            .unwrap()
            .insert(token.clone(), StoredToken { info, expires_at });
        token
    }

    /// Validates the token and returns associated info if valid and not expired.
    pub fn validate(&self, token: &str) -> Result<TokenInfo, TokenValidationError> {
        let map = self.tokens.read().unwrap();
        let st = map
            .get(token)
            .ok_or(TokenValidationError::InvalidOrExpired)?;
        if Instant::now() >= st.expires_at {
            return Err(TokenValidationError::InvalidOrExpired);
        }
        Ok(st.info.clone())
    }

    /// Revokes a token so that future validation fails.
    pub fn revoke(&self, token: &str) {
        self.tokens.write().unwrap().remove(token);
    }
}
