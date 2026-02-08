use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::RwLock;
use tracing::warn;

use crate::protocols::manager::ProtocolManager;
use crate::utils::config::OdinSettings;

pub mod device_registry;

/// Abstraktion für das Aktualisieren von Capabilities aller bekannten Services
/// und Geräte. Diese Abstraktion erlaubt es, den Scheduler unabhängig von der
/// konkreten Implementierung zu testen.
#[async_trait]
pub trait CapabilityDiscoverer: Send + Sync {
    async fn discover_all_capabilities(&self) -> Result<()>;
}

#[async_trait]
impl CapabilityDiscoverer for ProtocolManager {
    async fn discover_all_capabilities(&self) -> Result<()> {
        ProtocolManager::discover_all_capabilities(self).await
    }
}

/// Einfacher Device-Scheduler, der periodisch die bekannten Services/Devices
/// über das Einherjar-Protocol neu abfragt und den Capability-Cache aktualisiert.
///
/// Dies bildet den ersten Schritt zu einem vollwertigen Device-Loop für
/// Asgard/Midgard und angebundene Geräte: aktuell wird nur die Capability-
/// Sicht regelmäßig aufgefrischt, ohne bereits eine eigene Device-Registry
/// zu pflegen.
pub struct DeviceScheduler<D: CapabilityDiscoverer + 'static> {
    settings: Arc<RwLock<OdinSettings>>,
    discoverer: Arc<D>,
}

impl<D: CapabilityDiscoverer + 'static> DeviceScheduler<D> {
    /// Erstellt einen neuen Scheduler.
    pub fn new(settings: Arc<RwLock<OdinSettings>>, discoverer: Arc<D>) -> Self {
        Self {
            settings,
            discoverer,
        }
    }

    /// Startet den Hintergrund-Loop als Tokio-Task.
    ///
    /// Der Loop verwendet `state_sync.sync_interval_ms` als Intervall; falls
    /// nicht gesetzt, wird ein konservativer Default von 1000ms verwendet.
    pub fn start(self: Arc<Self>) {
        tokio::spawn(async move {
            self.run_loop().await;
        });
    }

    async fn run_loop(&self) {
        loop {
            let (interval_ms, scheduler_enabled, capability_refresh_enabled) = {
                let settings = self.settings.read().await;
                let interval = settings
                    .state_sync
                    .sync_interval_ms
                    .unwrap_or(1000)
                    .max(1);
                let enabled = settings.scheduler.enabled;
                let refresh_enabled = settings.scheduler.capability_refresh_enabled;
                (interval, enabled, refresh_enabled)
            };

            if scheduler_enabled && capability_refresh_enabled {
                if let Err(e) = self.poll_once().await {
                    warn!("Device scheduler poll failed: {}", e);
                }
            }

            tokio::time::sleep(Duration::from_millis(interval_ms as u64)).await;
        }
    }

    /// Führt genau einen Poll-Durchlauf aus.
    ///
    /// Diese Funktion ist öffentlich, um in Tests gezielt einen einzelnen
    /// Scheduler-Zyklus auszuführen.
    pub async fn poll_once(&self) -> Result<()> {
        self.discoverer.discover_all_capabilities().await
    }
}

