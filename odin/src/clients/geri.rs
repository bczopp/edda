use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;
use super::ServiceClientConfig;

pub mod geri {
    tonic::include_proto!("geri");
}

use geri::geri_service_client::GeriServiceClient;
use geri::{ProcessPromptRequest, ProcessPromptResponse, ProcessVisionRequest, ProcessVisionResponse};

/// Client for Geri service
pub struct GeriClient {
    client: GeriServiceClient<Channel>,
}

impl GeriClient {
    /// Create a new Geri client
    pub async fn new(config: ServiceClientConfig) -> Result<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(config.url)?
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        let client = GeriServiceClient::new(channel);
        
        Ok(Self { client })
    }

    /// Process a prompt via Geri
    pub async fn process_prompt(&mut self, request: ProcessPromptRequest) -> Result<ProcessPromptResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.process_prompt(req).await?;
        Ok(response.into_inner())
    }

    /// Process a vision request via Geri
    pub async fn process_vision(&mut self, request: ProcessVisionRequest) -> Result<ProcessVisionResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.process_vision(req).await?;
        Ok(response.into_inner())
    }
}
