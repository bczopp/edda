use tonic::transport::Channel;
use thiserror::Error;
use std::sync::Arc;
use tokio::sync::OnceCell;

#[derive(Debug, Error)]
pub enum PermissionError {
    #[error("Heimdall connection error: {0}")]
    ConnectionError(String),
    #[error("Permission denied: {0}")]
    Denied(String),
}

pub struct PermissionChecker {
    #[allow(dead_code)]
    heimdall_url: String,
    #[allow(dead_code)]
    client: Arc<OnceCell<tonic::client::Grpc<Channel>>>,
}

impl PermissionChecker {
    pub fn new(heimdall_url: String) -> Self {
        Self {
            heimdall_url,
            client: Arc::new(OnceCell::new()),
        }
    }

    #[allow(dead_code)]
    async fn get_client(&self) -> Result<tonic::client::Grpc<Channel>, PermissionError> {
        self.client
            .get_or_try_init(|| async {
                let channel = Channel::from_shared(self.heimdall_url.clone())
                    .map_err(|e| PermissionError::ConnectionError(format!("Invalid URL: {}", e)))?
                    .connect()
                    .await
                    .map_err(|e| PermissionError::ConnectionError(format!("Connection failed: {}", e)))?;
                Ok(tonic::client::Grpc::new(channel))
            })
            .await
            .map(|c| c.clone())
    }

    pub async fn check_permission(
        &self,
        _device_id: &str,
        _user_id: &str,
        _resource_type: &str,
        _action: &str,
    ) -> Result<bool, PermissionError> {
        // For now, return true (allow all) - Heimdall integration will be added later
        // This allows the system to function while Heimdall integration is being implemented
        Ok(true)
    }
}
