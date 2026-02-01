use tonic::transport::Channel;
use tonic::Request;
use thiserror::Error;
use tracing::{info, error};

// Generated code from proto files
pub mod heimdall_auth {
    tonic::include_proto!("heimdall.authentication");
}

pub mod heimdall_token {
    tonic::include_proto!("heimdall.token");
}

use heimdall_auth::authentication_service_client::AuthenticationServiceClient;
use heimdall_auth::{ChallengeRequest, ProofRequest, AuthenticationTokenResponse};
use heimdall_token::token_service_client::TokenServiceClient;
use heimdall_token::{ValidateTokenRequest, ValidateTokenResponse};

#[derive(Debug, Error)]
pub enum HeimdallClientError {
    #[error("Heimdall connection error: {0}")]
    ConnectionError(String),
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    #[error("Token validation failed: {0}")]
    TokenValidationFailed(String),
}

pub struct HeimdallClient {
    auth_client: Option<AuthenticationServiceClient<Channel>>,
    token_client: Option<TokenServiceClient<Channel>>,
    heimdall_url: String,
}

impl HeimdallClient {
    pub fn new(heimdall_url: String) -> Self {
        Self {
            auth_client: None,
            token_client: None,
            heimdall_url,
        }
    }

    pub async fn connect(&mut self) -> Result<(), HeimdallClientError> {
        let channel = Channel::from_shared(self.heimdall_url.clone())
            .map_err(|e| HeimdallClientError::ConnectionError(format!("Invalid URL: {}", e)))?
            .connect()
            .await
            .map_err(|e| HeimdallClientError::ConnectionError(format!("Connection failed: {}", e)))?;

        self.auth_client = Some(AuthenticationServiceClient::new(channel.clone()));
        self.token_client = Some(TokenServiceClient::new(channel));
        
        info!("Connected to Heimdall at {}", self.heimdall_url);
        Ok(())
    }

    pub async fn authenticate(
        &mut self,
        device_identity: &str,
    ) -> Result<String, HeimdallClientError> {
        // For now, use a simplified authentication flow
        // In a full implementation, this would use the challenge-response protocol
        
        // Connect if not already connected
        if self.auth_client.is_none() {
            self.connect().await?;
        }

        // TODO: Implement full challenge-response authentication flow
        // For now, return a placeholder token
        // This will be replaced with actual Heimdall authentication
        Ok(format!("token_for_{}", device_identity))
    }

    pub async fn validate_token(
        &mut self,
        token: &str,
        device_id: Option<&str>,
    ) -> Result<ValidateTokenResponse, HeimdallClientError> {
        // Connect if not already connected
        if self.token_client.is_none() {
            self.connect().await?;
        }

        let client = self.token_client.as_mut()
            .ok_or_else(|| HeimdallClientError::ConnectionError("Not connected".to_string()))?;

        let request = ValidateTokenRequest {
            token: token.to_string(),
            device_id: device_id.map(|s| s.to_string()).unwrap_or_default(),
        };

        let response = client.validate_token(Request::new(request))
            .await
            .map_err(|e| HeimdallClientError::TokenValidationFailed(format!("gRPC error: {}", e)))?
            .into_inner();

        if !response.valid {
            return Err(HeimdallClientError::TokenValidationFailed(
                response.reason.unwrap_or_else(|| "Token invalid".to_string())
            ));
        }

        Ok(response)
    }
}
