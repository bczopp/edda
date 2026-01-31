use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;
use super::ServiceClientConfig;

pub mod thor {
    tonic::include_proto!("thor");
}

use thor::thor_service_client::ThorServiceClient;
use thor::{ThorAction, ThorResult};

/// Client for Thor service
pub struct ThorClient {
    client: ThorServiceClient<Channel>,
}

impl ThorClient {
    /// Create a new Thor client
    pub async fn new(config: ServiceClientConfig) -> Result<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(config.url)?
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        let client = ThorServiceClient::new(channel);
        
        Ok(Self { client })
    }

    /// Execute an action via Thor
    pub async fn execute_action(&mut self, action: ThorAction) -> Result<ThorResult> {
        let request = tonic::Request::new(action);
        let response = self.client.execute_action(request).await?;
        Ok(response.into_inner())
    }
}
