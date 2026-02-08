//! Freki gRPC client (optional RAG context retrieval).
//! Phase 2: optional Freki-Client.

use tonic::transport::Channel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrekiClientError {
    #[error("Invalid URI: {0}")]
    InvalidUri(String),
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

pub struct FrekiClient {
    client: freki::freki_service_client::FrekiServiceClient<Channel>,
}

impl FrekiClient {
    pub async fn new(port: u16) -> Result<Self, FrekiClientError> {
        let endpoint = format!("http://127.0.0.1:{}", port);
        let channel = Channel::from_shared(endpoint)
            .map_err(|e| FrekiClientError::InvalidUri(e.to_string()))?
            .connect()
            .await?;
        let client = freki::freki_service_client::FrekiServiceClient::new(channel);
        Ok(Self { client })
    }

    pub async fn retrieve_context(
        &mut self,
        request: freki::RetrieveContextRequest,
    ) -> Result<freki::RetrieveContextResponse, FrekiClientError> {
        let response = self
            .client
            .retrieve_context(tonic::Request::new(request))
            .await?;
        Ok(response.into_inner())
    }
}

pub mod freki {
    tonic::include_proto!("freki");
}
