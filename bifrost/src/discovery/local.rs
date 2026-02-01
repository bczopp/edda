//! Local Discovery Manager (Phase 8.1)
//!
//! Manages local device discovery using mDNS/Bonjour.
//! Handles:
//! - Discovery requests
//! - Discovery responses processing
//! - Discovery timeouts
//! - Device list updates

use crate::discovery::mdns::{DiscoveredDevice, MDNSService};
use anyhow::{Context, Result};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// Configuration for LocalDiscoveryManager
#[derive(Debug, Clone)]
pub struct LocalDiscoveryConfig {
    /// Timeout for discovery operations
    pub discovery_timeout: Duration,
    /// Interval for continuous discovery (if enabled)
    pub continuous_interval: Option<Duration>,
}

impl Default for LocalDiscoveryConfig {
    fn default() -> Self {
        Self {
            discovery_timeout: Duration::from_secs(5),
            continuous_interval: None, // Disabled by default
        }
    }
}

/// Local Discovery Manager
pub struct LocalDiscoveryManager {
    mdns_service: Arc<MDNSService>,
    config: LocalDiscoveryConfig,
    devices: Arc<RwLock<Vec<DiscoveredDevice>>>,
    pub own_device_id: String,
}

impl LocalDiscoveryManager {
    /// Create a new LocalDiscoveryManager
    pub fn new(
        mdns_service: MDNSService,
        config: LocalDiscoveryConfig,
        own_device_id: String,
    ) -> Self {
        info!(
            device_id = %own_device_id,
            timeout_secs = config.discovery_timeout.as_secs(),
            "LocalDiscoveryManager created"
        );
        
        Self {
            mdns_service: Arc::new(mdns_service),
            config,
            devices: Arc::new(RwLock::new(Vec::new())),
            own_device_id,
        }
    }

    /// Start announcing this device in the local network
    pub async fn start_announcement(&self) -> Result<()> {
        self.mdns_service.register().await
            .context("Failed to start mDNS announcement")?;
        
        info!("Started local device announcement");
        Ok(())
    }

    /// Stop announcing this device
    pub async fn stop_announcement(&self) -> Result<()> {
        self.mdns_service.unregister().await
            .context("Failed to stop mDNS announcement")?;
        
        info!("Stopped local device announcement");
        Ok(())
    }

    /// Discover devices in the local network (single scan)
    pub async fn discover_devices(&self) -> Result<Vec<DiscoveredDevice>> {
        debug!("Starting device discovery");
        
        // Clear previous devices
        self.mdns_service.clear_discovered_devices().await;
        
        // Browse for devices
        let discovered = self.mdns_service.browse(self.config.discovery_timeout).await
            .context("mDNS browse failed")?;
        
        // Filter out own device (extra safety check)
        let filtered: Vec<DiscoveredDevice> = discovered
            .into_iter()
            .filter(|d| d.device_id != self.own_device_id)
            .collect();
        
        info!(
            count = filtered.len(),
            timeout_secs = self.config.discovery_timeout.as_secs(),
            "Device discovery completed"
        );
        
        // Update internal device list
        let mut devices = self.devices.write().await;
        *devices = filtered.clone();
        
        Ok(filtered)
    }

    /// Get currently discovered devices (from cache)
    pub async fn get_devices(&self) -> Vec<DiscoveredDevice> {
        let devices = self.devices.read().await;
        devices.clone()
    }

    /// Start continuous discovery in the background
    pub async fn start_continuous_discovery(self: Arc<Self>) -> Result<()> {
        let interval = self.config.continuous_interval
            .context("Continuous discovery not configured")?;
        
        info!(
            interval_secs = interval.as_secs(),
            "Starting continuous device discovery"
        );
        
        tokio::spawn(async move {
            loop {
                if let Err(e) = self.discover_devices().await {
                    error!("Continuous discovery error: {}", e);
                }
                
                tokio::time::sleep(interval).await;
            }
        });
        
        Ok(())
    }

    /// Clear all discovered devices
    pub async fn clear_devices(&self) {
        let mut devices = self.devices.write().await;
        devices.clear();
        self.mdns_service.clear_discovered_devices().await;
        debug!("Cleared all discovered devices");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::discovery::mdns::{MDNSService, MDNSServiceStub};

    fn create_test_manager() -> LocalDiscoveryManager {
        let stub = MDNSServiceStub::new("test-device".to_string(), 9001);
        let service = MDNSService::new(Arc::new(stub));
        let config = LocalDiscoveryConfig::default();
        LocalDiscoveryManager::new(service, config, "test-device".to_string())
    }

    #[test]
    fn test_local_discovery_manager_creation() {
        let manager = create_test_manager();
        assert_eq!(manager.own_device_id, "test-device");
    }

    #[tokio::test]
    async fn test_local_discovery_manager_empty_discovery() {
        let manager = create_test_manager();
        
        // Should return empty list if no devices found
        let devices = manager.discover_devices().await.unwrap();
        assert_eq!(devices.len(), 0);
    }

    #[tokio::test]
    async fn test_local_discovery_manager_get_devices() {
        let manager = create_test_manager();
        
        // Initially empty
        let devices = manager.get_devices().await;
        assert_eq!(devices.len(), 0);
    }

    #[tokio::test]
    async fn test_local_discovery_manager_clear() {
        let manager = create_test_manager();
        
        manager.clear_devices().await;
        
        let devices = manager.get_devices().await;
        assert_eq!(devices.len(), 0);
    }
}
