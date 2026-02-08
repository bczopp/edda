//! Device resolver: map cryptic device IDs to display name and capabilities.
//! Used by Odin or the platform connected to Jotunheim devices (see docs/DEVICE_TOOL_AUTO_DISCOVERY.md).

use crate::grpc::proto::jotunheim_capability::JotunheimCapabilities;
use std::collections::HashMap;
use std::sync::RwLock;

/// Resolved device: friendly name + capabilities for tool generation and UI.
#[derive(Clone, Debug)]
pub struct ResolvedDevice {
    pub device_id: String,
    pub display_name: String,
    pub capabilities: JotunheimCapabilities,
}

/// Resolves a device ID (or hardware ID) to display name and capabilities.
/// Implementations: in-memory map (Jotunheim), Odin device registry, platform-specific store.
pub trait DeviceResolver: Send + Sync {
    fn resolve(&self, device_id: &str) -> Option<ResolvedDevice>;
    fn list_device_ids(&self) -> Vec<String>;
}

/// In-memory resolver for tests or single-platform use. Odin/platform can replace with a persistent registry.
pub struct InMemoryDeviceResolver {
    map: RwLock<HashMap<String, ResolvedDevice>>,
}

impl InMemoryDeviceResolver {
    pub fn new() -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
        }
    }

    pub fn register(&self, device_id: String, display_name: String, capabilities: JotunheimCapabilities) {
        let dev = ResolvedDevice {
            device_id: device_id.clone(),
            display_name,
            capabilities,
        };
        if let Ok(mut m) = self.map.write() {
            m.insert(device_id, dev);
        }
    }

    pub fn unregister(&self, device_id: &str) {
        if let Ok(mut m) = self.map.write() {
            m.remove(device_id);
        }
    }
}

impl Default for InMemoryDeviceResolver {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceResolver for InMemoryDeviceResolver {
    fn resolve(&self, device_id: &str) -> Option<ResolvedDevice> {
        self.map.read().ok()?.get(device_id).cloned()
    }

    fn list_device_ids(&self) -> Vec<String> {
        self.map.read().map(|m| m.keys().cloned().collect()).unwrap_or_default()
    }
}
