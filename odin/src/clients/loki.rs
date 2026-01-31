use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;
use super::ServiceClientConfig;

pub mod loki {
    tonic::include_proto!("loki");
}

use loki::loki_service_client::LokiServiceClient;
use loki::{ExecuteScriptRequest, ExecuteScriptResponse, CallToolRequest, CallToolResponse};

/// Client for Loki service (Script Execution & IoT Tool Calling)
pub struct LokiClient {
    client: LokiServiceClient<Channel>,
}

impl LokiClient {
    /// Create a new Loki client
    pub async fn new(config: ServiceClientConfig) -> Result<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(config.url)?
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        let client = LokiServiceClient::new(channel);
        
        Ok(Self { client })
    }

    /// Execute a script via Loki
    pub async fn execute_script(&mut self, request: ExecuteScriptRequest) -> Result<ExecuteScriptResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.execute_script(req).await?;
        Ok(response.into_inner())
    }

    /// Call a tool on an IoT device via Loki
    pub async fn call_tool(&mut self, request: CallToolRequest) -> Result<CallToolResponse> {
        let req = tonic::Request::new(request);
        let response = self.client.call_tool(req).await?;
        Ok(response.into_inner())
    }
}
