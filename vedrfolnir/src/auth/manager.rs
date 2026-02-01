use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;
use tracing::{info, error};
use crate::grpc_client::heimdall::{HeimdallClient, HeimdallClientError};

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    #[error("Heimdall client error: {0}")]
    HeimdallError(#[from] HeimdallClientError),
}

pub struct AuthManager {
    heimdall_client: Arc<RwLock<Option<HeimdallClient>>>,
    heimdall_url: Option<String>,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            heimdall_client: Arc::new(RwLock::new(None)),
            heimdall_url: None,
        }
    }

    pub fn with_heimdall(heimdall_url: String) -> Self {
        Self {
            heimdall_client: Arc::new(RwLock::new(Some(HeimdallClient::new(heimdall_url.clone())))),
            heimdall_url: Some(heimdall_url),
        }
    }

    pub async fn authenticate(&self, device_identity: &str) -> Result<String, AuthError> {
        // If Heimdall URL is configured, use Heimdall for authentication
        if let Some(ref url) = self.heimdall_url {
            let mut client_guard = self.heimdall_client.write().await;
            let client = client_guard.as_mut()
                .ok_or_else(|| AuthError::AuthenticationFailed("Heimdall client not initialized".to_string()))?;
            
            match client.authenticate(device_identity).await {
                Ok(token) => {
                    info!("Authentication successful via Heimdall");
                    Ok(token)
                }
                Err(e) => {
                    error!("Heimdall authentication failed: {}", e);
                    Err(AuthError::from(e))
                }
            }
        } else {
            // Fallback: Return placeholder token if Heimdall is not configured
            // This allows Vedrfolnir to work without Heimdall for testing
            info!("Heimdall not configured, using placeholder token");
            Ok(format!("token_for_{}", device_identity))
        }
    }

    pub async fn validate_token(&self, token: &str, device_id: Option<&str>) -> Result<bool, AuthError> {
        if let Some(ref url) = self.heimdall_url {
            let mut client_guard = self.heimdall_client.write().await;
            let client = client_guard.as_mut()
                .ok_or_else(|| AuthError::AuthenticationFailed("Heimdall client not initialized".to_string()))?;
            
            match client.validate_token(token, device_id).await {
                Ok(_) => Ok(true),
                Err(e) => {
                    error!("Token validation failed: {}", e);
                    Err(AuthError::from(e))
                }
            }
        } else {
            // Fallback: Accept token if Heimdall is not configured
            Ok(true)
        }
    }
}
