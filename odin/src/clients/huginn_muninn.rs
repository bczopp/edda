use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;
use super::ServiceClientConfig;

pub mod huginn_muninn {
    tonic::include_proto!("huginn_muninn");
}

use huginn_muninn::huginn_muninn_service_client::HuginnMuninnServiceClient;
use huginn_muninn::{TranscribeRequest, TranscribeResponse, SynthesizeRequest, SynthesizeResponse, ForwardDataRequest, ForwardDataResponse};

/// Client for Huginn-Muninn service (STT/TTS)
pub struct HuginnMuninnClient {
    client: HuginnMuninnServiceClient<Channel>,
}

impl HuginnMuninnClient {
    /// Create a new Huginn-Muninn client
    pub async fn new(config: ServiceClientConfig) -> Result<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(config.url)?
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        let client = HuginnMuninnServiceClient::new(channel);
        
        Ok(Self { client })
    }

    /// Transcribe audio to text (STT)
    pub async fn transcribe(&mut self, request: TranscribeRequest) -> Result<TranscribeResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.transcribe(req).await?;
        Ok(response.into_inner())
    }

    /// Synthesize text to audio (TTS)
    pub async fn synthesize(&mut self, request: SynthesizeRequest) -> Result<SynthesizeResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.synthesize(req).await?;
        Ok(response.into_inner())
    }

    /// Forward data (text, images, videos, video streams)
    pub async fn forward_data(&mut self, request: ForwardDataRequest) -> Result<ForwardDataResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.forward_data(req).await?;
        Ok(response.into_inner())
    }
}
