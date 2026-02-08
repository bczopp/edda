//! Huginn gRPC client (optional STT / media forwarding).
//! Phase 2: optional Huginn-Client.

use tonic::transport::Channel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HuginnClientError {
    #[error("Invalid URI: {0}")]
    InvalidUri(String),
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

pub struct HuginnClient {
    client: huginn::huginn_media_service_client::HuginnMediaServiceClient<Channel>,
}

impl HuginnClient {
    pub async fn new(port: u16) -> Result<Self, HuginnClientError> {
        let endpoint = format!("http://127.0.0.1:{}", port);
        let channel = Channel::from_shared(endpoint)
            .map_err(|e| HuginnClientError::InvalidUri(e.to_string()))?
            .connect()
            .await?;
        let client = huginn::huginn_media_service_client::HuginnMediaServiceClient::new(channel);
        Ok(Self { client })
    }

    pub async fn transcribe_audio(
        &mut self,
        request: huginn::TranscribeAudioRequest,
    ) -> Result<huginn::TranscribeAudioResponse, HuginnClientError> {
        let response = self
            .client
            .transcribe_audio(tonic::Request::new(request))
            .await?;
        Ok(response.into_inner())
    }
}

pub mod raven {
    tonic::include_proto!("raven");
}
pub mod huginn {
    tonic::include_proto!("huginn");
}
