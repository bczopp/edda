//! Muninn gRPC client (optional TTS).
//! Phase 2: optional Muninn-Client.

use tonic::transport::Channel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MuninnClientError {
    #[error("Invalid URI: {0}")]
    InvalidUri(String),
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

pub struct MuninnClient {
    client: muninn::muninn_tts_service_client::MuninnTtsServiceClient<Channel>,
}

impl MuninnClient {
    pub async fn new(port: u16) -> Result<Self, MuninnClientError> {
        let endpoint = format!("http://127.0.0.1:{}", port);
        let channel = Channel::from_shared(endpoint)
            .map_err(|e| MuninnClientError::InvalidUri(e.to_string()))?
            .connect()
            .await?;
        let client = muninn::muninn_tts_service_client::MuninnTtsServiceClient::new(channel);
        Ok(Self { client })
    }

    pub async fn generate_speech(
        &mut self,
        request: muninn::TtsRequest,
    ) -> Result<muninn::TtsResponse, MuninnClientError> {
        let response = self
            .client
            .generate_speech(tonic::Request::new(request))
            .await?;
        Ok(response.into_inner())
    }
}

pub mod muninn {
    tonic::include_proto!("muninn");
}
