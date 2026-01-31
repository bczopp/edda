//! gRPC-backed plugin proxy: forwards [`OdinPlugin`] calls to a remote OdinService Process RPC.

use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;

use tonic::transport::Channel;
use crate::grpc::odin::odin_service_client::OdinServiceClient;
use crate::grpc::odin::{ProcessRequest, ProcessResponse};

use super::OdinPlugin;

/// Abstraction for calling the Odin Process RPC (testable; production uses tonic client).
#[async_trait]
pub trait ProcessClient: Send + Sync {
    /// Send a process request to the remote service.
    async fn process(
        &self,
        req: ProcessRequest,
    ) -> Result<ProcessResponse, Box<dyn std::error::Error + Send + Sync>>;
}

/// Plugin that delegates [`OdinPlugin::process_request`] to a remote service via Process RPC.
pub struct GrpcPluginProxy {
    name: String,
    capabilities: Vec<String>,
    client: Arc<dyn ProcessClient>,
}

impl GrpcPluginProxy {
    /// Build a proxy with the given name, capability labels, and process client.
    pub fn new(name: String, capabilities: Vec<String>, client: Arc<dyn ProcessClient>) -> Self {
        Self {
            name,
            capabilities,
            client,
        }
    }
}

#[async_trait]
impl OdinPlugin for GrpcPluginProxy {
    fn name(&self) -> &str {
        &self.name
    }

    fn capabilities(&self) -> Vec<String> {
        self.capabilities.clone()
    }

    async fn process_request(&self, request: &str) -> Result<String, Box<dyn std::error::Error>> {
        let req = ProcessRequest {
            request_id: String::new(),
            user_id: String::new(),
            device_id: String::new(),
            input: request.to_string(),
            input_type: "text".to_string(),
        };
        let res = self.client.process(req).await?;
        Ok(res.response)
    }
}

/// Production ProcessClient that calls the OdinService Process RPC at a given URL.
pub struct OdinGrpcProcessClient {
    client: tokio::sync::Mutex<OdinServiceClient<Channel>>,
}

impl OdinGrpcProcessClient {
    /// Connect to the given base URL and create a client (e.g. `http://[::1]:50051`).
    pub async fn new(base_url: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let endpoint = tonic::transport::Endpoint::from_shared(base_url.to_string())?
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(5));
        let channel = endpoint.connect().await?;
        let client = OdinServiceClient::new(channel);
        Ok(Self {
            client: tokio::sync::Mutex::new(client),
        })
    }
}

#[async_trait]
impl ProcessClient for OdinGrpcProcessClient {
    async fn process(
        &self,
        req: ProcessRequest,
    ) -> Result<ProcessResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut guard = self.client.lock().await;
        let resp = guard.process(tonic::Request::new(req)).await?;
        Ok(resp.into_inner())
    }
}
