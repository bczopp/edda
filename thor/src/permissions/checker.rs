use crate::heimdall_authz::{
    authorization_service_client::AuthorizationServiceClient,
    PermissionCheckRequest,
};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::OnceCell;
use tonic::transport::Channel;
use tonic::Request;

#[derive(Debug, Error)]
pub enum PermissionError {
    #[error("Heimdall connection error: {0}")]
    ConnectionError(String),
    #[error("Permission denied: {0}")]
    Denied(String),
    #[error("Heimdall RPC error: {0}")]
    RpcError(#[from] tonic::Status),
}

pub struct PermissionChecker {
    heimdall_url: String,
    channel: Arc<OnceCell<Channel>>,
    /// When true, return Ok(true) if Heimdall is unreachable (for tests without Heimdall).
    allow_on_connection_error: bool,
    /// When true, always return Ok(false) without calling Heimdall (for security tests).
    deny_all: bool,
}

impl PermissionChecker {
    pub fn new(heimdall_url: String) -> Self {
        Self {
            heimdall_url,
            channel: Arc::new(OnceCell::new()),
            allow_on_connection_error: false,
            deny_all: false,
        }
    }

    /// For tests: when Heimdall is unreachable, return Ok(true) instead of Err.
    pub fn new_allow_on_connection_error(heimdall_url: String) -> Self {
        Self {
            heimdall_url,
            channel: Arc::new(OnceCell::new()),
            allow_on_connection_error: true,
            deny_all: false,
        }
    }

    /// For tests: always deny permission without calling Heimdall (security tests).
    pub fn new_deny_all(heimdall_url: String) -> Self {
        Self {
            heimdall_url,
            channel: Arc::new(OnceCell::new()),
            allow_on_connection_error: false,
            deny_all: true,
        }
    }

    async fn get_channel(&self) -> Result<Channel, PermissionError> {
        self.channel
            .get_or_try_init(|| async {
                Channel::from_shared(self.heimdall_url.clone())
                    .map_err(|e| PermissionError::ConnectionError(format!("Invalid URL: {}", e)))?
                    .connect()
                    .await
                    .map_err(|e| PermissionError::ConnectionError(format!("Connection failed: {}", e)))
            })
            .await
            .map(|c| c.clone())
    }

    pub async fn check_permission(
        &self,
        device_id: &str,
        user_id: &str,
        resource_type: &str,
        action: &str,
    ) -> Result<bool, PermissionError> {
        if self.deny_all {
            return Ok(false);
        }
        let channel = match self.get_channel().await {
            Ok(c) => c,
            Err(e) if self.allow_on_connection_error => return Ok(true),
            Err(e) => return Err(e),
        };
        let mut client = AuthorizationServiceClient::new(channel);
        let request = PermissionCheckRequest {
            device_id: device_id.to_string(),
            user_id: user_id.to_string(),
            resource_type: resource_type.to_string(),
            action: action.to_string(),
            resource_id: String::new(),
            context: std::collections::HashMap::new(),
        };
        let response = match client.check_permission(Request::new(request)).await {
            Ok(r) => r.into_inner(),
            Err(_) if self.allow_on_connection_error => return Ok(true),
            Err(e) => return Err(e.into()),
        };
        Ok(response.allowed)
    }
}
