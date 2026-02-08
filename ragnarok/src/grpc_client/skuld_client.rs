//! Skuld gRPC client (model selection and optimization info).
//! Phase 2: optional gRPC clients.

use tonic::transport::Channel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SkuldClientError {
    #[error("Invalid URI: {0}")]
    InvalidUri(String),
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

#[derive(Clone)]
pub struct SkuldClient {
    client: skuld::skuld_service_client::SkuldServiceClient<Channel>,
}

impl SkuldClient {
    pub async fn new(port: u16) -> Result<Self, SkuldClientError> {
        let endpoint = format!("http://127.0.0.1:{}", port);
        let channel = Channel::from_shared(endpoint)
            .map_err(|e| SkuldClientError::InvalidUri(e.to_string()))?
            .connect()
            .await?;
        
        let client = skuld::skuld_service_client::SkuldServiceClient::new(channel);
        
        Ok(Self { client })
    }

    pub async fn select_model(&mut self, request: skuld::SelectModelRequest) -> Result<skuld::SelectModelResponse, SkuldClientError> {
        let response = self.client
            .select_model(tonic::Request::new(request))
            .await?;
        Ok(response.into_inner())
    }
}

pub mod skuld {
    tonic::include_proto!("skuld");
}
