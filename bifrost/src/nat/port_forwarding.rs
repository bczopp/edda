//! Port Forwarding Configurator (Phase 13.4 – NAT Traversal Fallback).
//!
//! Manual or automatic (UPnP/NAT-PMP) port forwarding. Uses a provider trait so tests
//! can use a stub; production can use UPnP/NAT-PMP libraries.

use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tracing::info;

/// Provider trait for port forwarding (enables stub in tests / real impl in production).
#[async_trait]
pub trait PortForwardingConfiguratorProvider: Send + Sync {
    /// Try to configure port forwarding for the given port.
    /// Protocol: Some("tcp") or Some("udp"), or None for both.
    /// Returns true if forwarding was configured (or already present), false otherwise.
    async fn try_configure_forward(&self, port: u16, protocol: Option<String>) -> Result<bool>;
    /// Remove port forwarding for the given port. Returns true if removed or not present.
    async fn remove_forward(&self, port: u16) -> Result<bool>;
}

/// Port forwarding configurator wrapper.
pub struct PortForwardingConfigurator {
    provider: Arc<dyn PortForwardingConfiguratorProvider>,
}

impl PortForwardingConfigurator {
    pub fn new(provider: Arc<dyn PortForwardingConfiguratorProvider>) -> Self {
        info!("PortForwardingConfigurator created");
        Self { provider }
    }

    pub async fn try_configure_forward(&self, port: u16, protocol: Option<String>) -> Result<bool> {
        self.provider.try_configure_forward(port, protocol).await
    }

    pub async fn remove_forward(&self, port: u16) -> Result<bool> {
        self.provider.remove_forward(port).await
    }
}

/// Stub configurator for tests and environments without UPnP/NAT-PMP.
pub struct PortForwardingConfiguratorStub {
    /// Whether try_configure_forward and remove_forward return success.
    success: bool,
}

impl PortForwardingConfiguratorStub {
    pub fn new(success: bool) -> Self {
        info!(success, "PortForwardingConfiguratorStub created");
        Self { success }
    }
}

#[async_trait]
impl PortForwardingConfiguratorProvider for PortForwardingConfiguratorStub {
    async fn try_configure_forward(&self, _port: u16, _protocol: Option<String>) -> Result<bool> {
        Ok(self.success)
    }

    async fn remove_forward(&self, _port: u16) -> Result<bool> {
        Ok(self.success)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_stub_success() {
        let stub = PortForwardingConfiguratorStub::new(true);
        assert!(stub.try_configure_forward(9000, None).await.unwrap());
        assert!(stub.remove_forward(9000).await.unwrap());
    }

    #[tokio::test]
    async fn test_stub_failure() {
        let stub = PortForwardingConfiguratorStub::new(false);
        assert!(!stub.try_configure_forward(9000, None).await.unwrap());
    }

    #[tokio::test]
    async fn test_configurator_wraps_stub() {
        let stub = Arc::new(PortForwardingConfiguratorStub::new(true));
        let configurator = PortForwardingConfigurator::new(stub);
        assert!(configurator.try_configure_forward(3478, Some("udp".to_string())).await.unwrap());
    }
}
