//! mDNS/Bonjour Service for Local Device Discovery (Phase 8.1)
//!
//! Provides automatic device discovery in the local network using mDNS/Bonjour protocol.
//! Supports:
//! - Service announcement (register device)
//! - Service discovery (find other devices)
//! - Service record parsing
//!
//! **Implementation Note**: This module uses a trait-based abstraction to enable testing
//! in container environments where mDNS multicast may not work. The production implementation
//! will use an actual mDNS library (e.g., `mdns-sd`), while tests use a stub.

use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Service type for Bifrost devices in mDNS
pub const BIFROST_SERVICE_TYPE: &str = "_bifrost._tcp.local.";

/// Discovered device information from mDNS
#[derive(Debug, Clone, PartialEq)]
pub struct DiscoveredDevice {
    pub device_id: String,
    pub ip_address: IpAddr,
    pub port: u16,
    pub hostname: String,
}

/// Trait for mDNS service providers (enables testing with stubs)
#[async_trait]
pub trait MDNSServiceProvider: Send + Sync {
    /// Register this device in the local network
    async fn register(&self) -> Result<()>;
    
    /// Unregister this device from the local network
    async fn unregister(&self) -> Result<()>;
    
    /// Browse for Bifrost devices in the local network
    async fn browse(&self, timeout: Duration) -> Result<Vec<DiscoveredDevice>>;
    
    /// Get all currently discovered devices (from cache)
    async fn get_discovered_devices(&self) -> Vec<DiscoveredDevice>;
    
    /// Clear all discovered devices
    async fn clear_discovered_devices(&self);
}

/// mDNS Service implementation
pub struct MDNSService {
    provider: Arc<dyn MDNSServiceProvider>,
}

impl MDNSService {
    /// Create a new mDNS service with a provider
    pub fn new(provider: Arc<dyn MDNSServiceProvider>) -> Self {
        info!("MDNSService created");
        Self { provider }
    }
    
    /// Register this device in the local network (service announcement)
    pub async fn register(&self) -> Result<()> {
        self.provider.register().await
    }
    
    /// Unregister this device from the local network
    pub async fn unregister(&self) -> Result<()> {
        self.provider.unregister().await
    }
    
    /// Browse for Bifrost devices in the local network
    pub async fn browse(&self, timeout: Duration) -> Result<Vec<DiscoveredDevice>> {
        self.provider.browse(timeout).await
    }
    
    /// Get all currently discovered devices
    pub async fn get_discovered_devices(&self) -> Vec<DiscoveredDevice> {
        self.provider.get_discovered_devices().await
    }
    
    /// Clear all discovered devices
    pub async fn clear_discovered_devices(&self) {
        self.provider.clear_discovered_devices().await;
    }
}

/// Stub mDNS provider for testing (simulates mDNS without network)
pub struct MDNSServiceStub {
    own_device_id: String,
    _port: u16,
    registered: Arc<RwLock<bool>>,
    discovered_devices: Arc<RwLock<HashMap<String, DiscoveredDevice>>>,
    simulated_devices: Arc<RwLock<Vec<DiscoveredDevice>>>,
}

impl MDNSServiceStub {
    /// Create a new stub mDNS service
    pub fn new(device_id: String, port: u16) -> Self {
        info!(device_id = %device_id, port = port, "MDNSServiceStub created");
        
        Self {
            own_device_id: device_id,
            _port: port,
            registered: Arc::new(RwLock::new(false)),
            discovered_devices: Arc::new(RwLock::new(HashMap::new())),
            simulated_devices: Arc::new(RwLock::new(Vec::new())),
        }
    }
    
    /// Add a simulated device for testing discovery
    pub async fn add_simulated_device(&self, device: DiscoveredDevice) {
        let mut devices = self.simulated_devices.write().await;
        devices.push(device);
    }
    
    /// Remove all simulated devices
    pub async fn clear_simulated_devices(&self) {
        let mut devices = self.simulated_devices.write().await;
        devices.clear();
    }
    
    /// Check if service is registered
    pub async fn is_registered(&self) -> bool {
        *self.registered.read().await
    }
}

#[async_trait]
impl MDNSServiceProvider for MDNSServiceStub {
    async fn register(&self) -> Result<()> {
        let mut registered = self.registered.write().await;
        *registered = true;
        info!(device_id = %self.own_device_id, "MDNSServiceStub registered");
        Ok(())
    }
    
    async fn unregister(&self) -> Result<()> {
        let mut registered = self.registered.write().await;
        *registered = false;
        info!(device_id = %self.own_device_id, "MDNSServiceStub unregistered");
        Ok(())
    }
    
    async fn browse(&self, _timeout: Duration) -> Result<Vec<DiscoveredDevice>> {
        debug!("MDNSServiceStub browse started");
        
        // Simulate discovery: return simulated devices, filter out own device
        let simulated = self.simulated_devices.read().await;
        let filtered: Vec<DiscoveredDevice> = simulated
            .iter()
            .filter(|d| d.device_id != self.own_device_id)
            .cloned()
            .collect();
        
        // Update discovered devices cache
        let mut discovered = self.discovered_devices.write().await;
        discovered.clear();
        for device in &filtered {
            discovered.insert(device.device_id.clone(), device.clone());
        }
        
        info!(count = filtered.len(), "MDNSServiceStub browse completed");
        Ok(filtered)
    }
    
    async fn get_discovered_devices(&self) -> Vec<DiscoveredDevice> {
        let devices = self.discovered_devices.read().await;
        devices.values().cloned().collect()
    }
    
    async fn clear_discovered_devices(&self) {
        let mut devices = self.discovered_devices.write().await;
        devices.clear();
        debug!("MDNSServiceStub cleared all discovered devices");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_mdns_service_stub_creation() {
        let device_id = "test-device-001".to_string();
        let port = 9001;
        
        let stub = MDNSServiceStub::new(device_id.clone(), port);
        assert_eq!(stub.own_device_id, device_id);
    }

    #[tokio::test]
    async fn test_mdns_service_stub_register_unregister() {
        let stub = MDNSServiceStub::new("test-device".to_string(), 9001);
        
        // Initially not registered
        assert!(!stub.is_registered().await);
        
        // Register
        stub.register().await.unwrap();
        assert!(stub.is_registered().await);
        
        // Unregister
        stub.unregister().await.unwrap();
        assert!(!stub.is_registered().await);
    }

    #[tokio::test]
    async fn test_mdns_service_stub_discovery() {
        let stub = MDNSServiceStub::new("own-device".to_string(), 9001);
        
        // Add simulated devices
        stub.add_simulated_device(DiscoveredDevice {
            device_id: "device-1".to_string(),
            ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
            port: 9001,
            hostname: "device-1.local".to_string(),
        }).await;
        
        stub.add_simulated_device(DiscoveredDevice {
            device_id: "own-device".to_string(), // Should be filtered
            ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 101)),
            port: 9001,
            hostname: "own-device.local".to_string(),
        }).await;
        
        // Browse
        let devices = stub.browse(Duration::from_millis(100)).await.unwrap();
        
        // Only device-1 should be found (own-device filtered)
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].device_id, "device-1");
    }

    #[tokio::test]
    async fn test_mdns_service_wrapper() {
        let stub = Arc::new(MDNSServiceStub::new("test-device".to_string(), 9001));
        let service = MDNSService::new(stub.clone());
        
        // Register
        service.register().await.unwrap();
        assert!(stub.is_registered().await);
        
        // Unregister
        service.unregister().await.unwrap();
        assert!(!stub.is_registered().await);
    }
}
