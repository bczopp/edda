//! Thor gRPC client (optional action execution).
//! Phase 2/4: Thor-Client-Integration.

use tonic::transport::Channel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThorClientError {
    #[error("Invalid URI: {0}")]
    InvalidUri(String),
    #[error("gRPC error: {0}")]
    GrpcError(#[from] tonic::Status),
    #[error("Transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),
}

pub struct ThorClient {
    client: thor::thor_service_client::ThorServiceClient<Channel>,
}

impl ThorClient {
    pub async fn new(port: u16) -> Result<Self, ThorClientError> {
        let endpoint = format!("http://127.0.0.1:{}", port);
        let channel = Channel::from_shared(endpoint)
            .map_err(|e| ThorClientError::InvalidUri(e.to_string()))?
            .connect()
            .await?;
        let client = thor::thor_service_client::ThorServiceClient::new(channel);
        Ok(Self { client })
    }

    pub async fn execute_action(
        &mut self,
        action: thor::ThorAction,
    ) -> Result<thor::ThorResult, ThorClientError> {
        let response = self
            .client
            .execute_action(tonic::Request::new(action))
            .await?;
        Ok(response.into_inner())
    }

    pub async fn execute_action_stream(
        &mut self,
        stream: impl tonic::IntoStreamingRequest<Message = thor::ThorActionStreamRequest>,
    ) -> Result<tonic::Response<tonic::Streaming<thor::ThorActionStreamResponse>>, ThorClientError> {
        let response = self
            .client
            .execute_action_stream(stream)
            .await?;
        Ok(response)
    }
}

pub mod thor {
    tonic::include_proto!("thor");
}
