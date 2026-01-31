//! Yggdrasil Discovery Client (Phase 8.3.1). Discovery request/response, device list from Yggdrasil.

use async_trait::async_trait;
use std::sync::Arc;

/// Device info returned by Yggdrasil discovery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    pub device_id: String,
    pub user_id: String,
    pub host: String,
    pub port: u16,
}

impl DeviceInfo {
    /// WebSocket URL for this device (ws://host:port).
    pub fn ws_url(&self) -> String {
        format!("ws://{}:{}", self.host, self.port)
    }
}

/// Provider for device discovery (Yggdrasil API). Implement or use stub for tests.
#[async_trait]
pub trait YggdrasilDiscoveryProvider: Send + Sync {
    async fn fetch_devices(
        &self,
        user_id: &str,
    ) -> Result<Vec<DeviceInfo>, Box<dyn std::error::Error + Send + Sync>>;
}

/// Client that obtains device list from Yggdrasil (sends discovery request, parses response).
pub struct YggdrasilDiscoveryClient {
    provider: Arc<dyn YggdrasilDiscoveryProvider>,
}

impl YggdrasilDiscoveryClient {
    pub fn new(provider: Arc<dyn YggdrasilDiscoveryProvider>) -> Self {
        Self { provider }
    }

    /// Sends discovery request for user_id and returns device list from Yggdrasil response.
    pub async fn list_devices(
        &self,
        user_id: &str,
    ) -> Result<Vec<DeviceInfo>, Box<dyn std::error::Error + Send + Sync>> {
        self.provider.fetch_devices(user_id).await
    }
}

/// Stub provider for tests: returns a fixed device list or a fixed error.
pub struct YggdrasilDiscoveryStub {
    devices: Option<Vec<DeviceInfo>>,
    error: Option<String>,
}

impl YggdrasilDiscoveryStub {
    pub fn with_devices(devices: Vec<DeviceInfo>) -> Self {
        Self {
            devices: Some(devices),
            error: None,
        }
    }

    pub fn failing(message: &str) -> Self {
        Self {
            devices: None,
            error: Some(message.to_string()),
        }
    }
}

#[async_trait]
impl YggdrasilDiscoveryProvider for YggdrasilDiscoveryStub {
    async fn fetch_devices(
        &self,
        _user_id: &str,
    ) -> Result<Vec<DeviceInfo>, Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref err) = self.error {
            return Err(err.clone().into());
        }
        Ok(self.devices.clone().unwrap_or_default())
    }
}
