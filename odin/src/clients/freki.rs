use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;
use super::ServiceClientConfig;

pub mod freki {
    tonic::include_proto!("freki");
}

use freki::freki_service_client::FrekiServiceClient;
use freki::{RetrieveContextRequest, RetrieveContextResponse, IndexDocumentRequest, IndexDocumentResponse};

/// Client for Freki service
pub struct FrekiClient {
    client: FrekiServiceClient<Channel>,
}

impl FrekiClient {
    /// Create a new Freki client
    pub async fn new(config: ServiceClientConfig) -> Result<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(config.url)?
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        let client = FrekiServiceClient::new(channel);
        
        Ok(Self { client })
    }

    /// Retrieve context from Freki
    pub async fn retrieve_context(&mut self, request: RetrieveContextRequest) -> Result<RetrieveContextResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.retrieve_context(req).await?;
        Ok(response.into_inner())
    }

    /// Index a document in Freki
    pub async fn index_document(&mut self, request: IndexDocumentRequest) -> Result<IndexDocumentResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.index_document(req).await?;
        Ok(response.into_inner())
    }
}
