// Loki gRPC client for Jotunheim (Phase 2.2.2).

use crate::grpc::proto::loki::{
    loki_service_client::LokiServiceClient,
    GetCapabilitiesRequest, GetChildrenStatusRequest, ListScriptsRequest,
    ExecuteScriptRequest,
};
use std::time::Duration;
use tonic::transport::Channel;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum LokiClientError {
    #[error("gRPC transport: {0}")]
    Transport(#[from] tonic::transport::Error),
    #[error("gRPC status: {0}")]
    Status(#[from] tonic::Status),
    #[error("invalid endpoint: {0}")]
    InvalidEndpoint(String),
}

/// Client for Loki Script Execution Service (gRPC).
pub struct LokiClient {
    inner: LokiServiceClient<Channel>,
}

impl LokiClient {
    /// Build endpoint from address and port (e.g. "http://192.168.1.10:50057").
    pub fn endpoint(addr: &str, port: u16) -> Result<tonic::transport::Endpoint, LokiClientError> {
        let url = format!("http://{}:{}", addr, port);
        tonic::transport::Endpoint::from_shared(url).map_err(|e| {
            LokiClientError::InvalidEndpoint(format!("{}: {}", url, e))
        })
    }

    /// Create client from a channel (for tests / custom transport).
    pub fn from_channel(channel: Channel) -> Self {
        Self {
            inner: LokiServiceClient::new(channel),
        }
    }

    /// Connect to Loki service at the given endpoint.
    pub async fn connect(
        addr: &str,
        port: u16,
        timeout_secs: u64,
    ) -> Result<Self, LokiClientError> {
        let endpoint = Self::endpoint(addr, port)?
            .timeout(Duration::from_secs(timeout_secs))
            .connect()
            .await?;
        Ok(Self {
            inner: LokiServiceClient::new(endpoint),
        })
    }

    /// Get capabilities of Loki and its children.
    pub async fn get_capabilities(
        &mut self,
    ) -> Result<
        tonic::Response<crate::grpc::proto::loki::GetCapabilitiesResponse>,
        LokiClientError,
    > {
        let req = tonic::Request::new(GetCapabilitiesRequest {});
        Ok(self.inner.get_capabilities(req).await?)
    }

    /// Get status of child services (Fenrir, Jörmungandr, Hel).
    pub async fn get_children_status(
        &mut self,
    ) -> Result<
        tonic::Response<crate::grpc::proto::loki::GetChildrenStatusResponse>,
        LokiClientError,
    > {
        let req = tonic::Request::new(GetChildrenStatusRequest {});
        Ok(self.inner.get_children_status(req).await?)
    }

    /// List available scripts, optionally filtered by name pattern.
    pub async fn list_scripts(
        &mut self,
        name_pattern: impl Into<String>,
    ) -> Result<
        tonic::Response<crate::grpc::proto::loki::ListScriptsResponse>,
        LokiClientError,
    > {
        let req = tonic::Request::new(ListScriptsRequest {
            name_pattern: name_pattern.into(),
        });
        Ok(self.inner.list_scripts(req).await?)
    }

    /// Legacy: call a script by id/content (ExecuteScript RPC).
    pub async fn call_function(
        &mut self,
        script_id: &str,
        script_content: &str,
        script_type: &str,
        parameters: std::collections::HashMap<String, String>,
    ) -> Result<String, LokiClientError> {
        let req = tonic::Request::new(ExecuteScriptRequest {
            script_id: script_id.to_string(),
            script_content: script_content.to_string(),
            script_type: script_type.to_string(),
            parameters,
        });
        let res = self.inner.execute_script(req).await?.into_inner();
        Ok(res.output)
    }
}
