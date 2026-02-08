//! Geri gRPC client (optional direct LLM prompt / model list).
//! Phase 2: optional Geri-Client.

use tonic::transport::Channel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GeriClientError {
    #[error("Invalid URI: {0}")]
    InvalidUri(String),
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

pub struct GeriClient {
    client: geri::geri_service_client::GeriServiceClient<Channel>,
}

impl GeriClient {
    pub async fn new(port: u16) -> Result<Self, GeriClientError> {
        let endpoint = format!("http://127.0.0.1:{}", port);
        let channel = Channel::from_shared(endpoint)
            .map_err(|e| GeriClientError::InvalidUri(e.to_string()))?
            .connect()
            .await?;
        let client = geri::geri_service_client::GeriServiceClient::new(channel);
        Ok(Self { client })
    }

    pub async fn process_prompt(
        &mut self,
        request: geri::ProcessPromptRequest,
    ) -> Result<geri::ProcessPromptResponse, GeriClientError> {
        let response = self
            .client
            .process_prompt(tonic::Request::new(request))
            .await?;
        Ok(response.into_inner())
    }

    pub async fn list_models(
        &mut self,
        request: geri::ListModelsRequest,
    ) -> Result<geri::ListModelsResponse, GeriClientError> {
        let response = self
            .client
            .list_models(tonic::Request::new(request))
            .await?;
        Ok(response.into_inner())
    }
}

pub mod geri {
    tonic::include_proto!("geri");
}
