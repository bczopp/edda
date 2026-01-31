use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;
use super::ServiceClientConfig;

pub mod heimdall {
    tonic::include_proto!("heimdall");
}

use heimdall::{
    authentication_service_client::AuthenticationServiceClient,
    authorization_service_client::AuthorizationServiceClient,
    token_service_client::TokenServiceClient,
    bifrost_validation_service_client::BifrostValidationServiceClient,
};
use heimdall::{
    ChallengeRequest, ChallengeResponse, ProofRequest, AuthenticationTokenResponse,
    PermissionCheckRequest, PermissionCheckResponse,
    ValidateTokenRequest, ValidateTokenResponse,
    ConnectionValidationRequest, ConnectionValidationResponse,
};

/// Client for Heimdall service (Security)
pub struct HeimdallClient {
    auth_client: AuthenticationServiceClient<Channel>,
    authz_client: AuthorizationServiceClient<Channel>,
    token_client: TokenServiceClient<Channel>,
    bifrost_client: BifrostValidationServiceClient<Channel>,
}

impl HeimdallClient {
    /// Create a new Heimdall client
    pub async fn new(config: ServiceClientConfig) -> Result<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(config.url)?
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        
        Ok(Self {
            auth_client: AuthenticationServiceClient::new(channel.clone()),
            authz_client: AuthorizationServiceClient::new(channel.clone()),
            token_client: TokenServiceClient::new(channel.clone()),
            bifrost_client: BifrostValidationServiceClient::new(channel),
        })
    }

    /// Request authentication challenge
    pub async fn request_challenge(&mut self, request: ChallengeRequest) -> Result<ChallengeResponse> {
        let req = tonic::Request::new(request);
        let response = self.auth_client.request_challenge(req).await?;
        Ok(response.into_inner())
    }

    /// Prove identity and get token
    pub async fn prove_identity(&mut self, request: ProofRequest) -> Result<AuthenticationTokenResponse> {
        let req = tonic::Request::new(request);
        let response = self.auth_client.prove_identity(req).await?;
        Ok(response.into_inner())
    }

    /// Check permission
    pub async fn check_permission(&mut self, request: PermissionCheckRequest) -> Result<PermissionCheckResponse> {
        let req = tonic::Request::new(request);
        let response = self.authz_client.check_permission(req).await?;
        Ok(response.into_inner())
    }

    /// Validate token
    pub async fn validate_token(&mut self, request: ValidateTokenRequest) -> Result<ValidateTokenResponse> {
        let req = tonic::Request::new(request);
        let response = self.token_client.validate_token(req).await?;
        Ok(response.into_inner())
    }

    /// Validate Bifrost connection
    pub async fn validate_connection(&mut self, request: ConnectionValidationRequest) -> Result<ConnectionValidationResponse> {
        let req = tonic::Request::new(request);
        let response = self.bifrost_client.validate_connection(req).await?;
        Ok(response.into_inner())
    }
}
