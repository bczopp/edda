use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;
use super::ServiceClientConfig;

pub mod skuld {
    tonic::include_proto!("skuld");
}

use skuld::skuld_service_client::SkuldServiceClient;
use skuld::{SelectModelRequest, SelectModelResponse};

/// Client for Skuld service (LLM Model Selection)
pub struct SkuldClient {
    client: SkuldServiceClient<Channel>,
}

impl SkuldClient {
    /// Create a new Skuld client
    pub async fn new(config: ServiceClientConfig) -> Result<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(config.url)?
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        let client = SkuldServiceClient::new(channel);
        
        Ok(Self { client })
    }

    /// Select appropriate model for a prompt
    pub async fn select_model(
        &mut self,
        request: SelectModelRequest,
    ) -> Result<SelectModelResponse> {
        let request = tonic::Request::new(request);
        let response = self.client.select_model(request).await?;
        Ok(response.into_inner())
    }
}
