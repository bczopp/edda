//! Registry für logische Devices (Haus, Fahrzeug, Roboter) mit Zuordnung zu
//! Platformen (Asgard/Midgard). Odin hält nur die Liste; Sensor-/Aktor-Polling
//! und Ausführung laufen in den Platformen bzw. über Thor/Bifrost/Jotunheim.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Art des logischen Devices (Haus, Fahrzeug, Roboter).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogicalDeviceKind {
    House,
    Vehicle,
    Robot,
}

/// Ein logisches Device mit Zuordnung zu einer Platform (Asgard/Midgard).
#[derive(Debug, Clone)]
pub struct LogicalDevice {
    pub id: String,
    pub kind: LogicalDeviceKind,
    /// ID der Platform, die dieses Device steuert (z.B. Asgard- oder Midgard-Instanz).
    pub platform_id: String,
    pub name: String,
}

/// Registry aller bekannten logischen Devices.
pub struct DeviceRegistry {
    devices: Arc<RwLock<HashMap<String, LogicalDevice>>>,
}

impl DeviceRegistry {
    pub fn new() -> Self {
        Self {
            devices: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, device: LogicalDevice) {
        let mut devices = self.devices.write().await;
        devices.insert(device.id.clone(), device);
    }

    pub async fn get(&self, id: &str) -> Option<LogicalDevice> {
        let devices = self.devices.read().await;
        devices.get(id).cloned()
    }

    pub async fn list(&self) -> Vec<LogicalDevice> {
        let devices = self.devices.read().await;
        devices.values().cloned().collect()
    }
}

impl Default for DeviceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
