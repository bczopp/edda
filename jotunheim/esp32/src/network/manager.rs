// WiFi Manager (Phase 3.1.1, TDD).

use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Debug, Error)]
pub enum NetworkError {
    #[error("Network initialization failed: {0}")]
    InitializationFailed(String),
    #[error("WiFi connection failed: {0}")]
    ConnectionFailed(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WiFiStatus {
    Disconnected,
    Connecting,
    Connected,
}

pub struct WiFiManager {
    ssid: String,
    password: String,
    status: Arc<RwLock<WiFiStatus>>,
}

impl WiFiManager {
    pub fn new(ssid: String, password: String) -> Self {
        Self {
            ssid,
            password,
            status: Arc::new(RwLock::new(WiFiStatus::Disconnected)),
        }
    }

    pub fn status(&self) -> WiFiStatus {
        self.status.try_read().map(|g| *g).unwrap_or(WiFiStatus::Disconnected)
    }

    pub async fn connect(&self) -> Result<(), NetworkError> {
        {
            let mut s = self.status.write().await;
            *s = WiFiStatus::Connecting;
        }
        // On host: simulate success; on ESP32 would call esp-idf WiFi
        {
            let mut s = self.status.write().await;
            *s = WiFiStatus::Connected;
        }
        Ok(())
    }

    pub async fn reconnect(&self) -> Result<(), NetworkError> {
        self.connect().await
    }
}

// Legacy alias
pub type NetworkManager = WiFiManager;
