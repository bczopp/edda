use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
}

pub struct AuthManager;

impl AuthManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn authenticate(&self, credentials: &str) -> Result<String, AuthError> {
        // TODO: Authenticate with Yggdrasil
        Ok("token".to_string())
    }
}
