use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("invalid websocket_port: {0} (must be 1-65535)")]
    InvalidPort(u16),
    #[error("invalid max_connections: {0} (must be >= 1)")]
    InvalidMaxConnections(u32),
    #[error("invalid message_timeout_seconds: {0} (must be >= 1)")]
    InvalidMessageTimeout(u64),
    #[error("heimdall_url must not be empty")]
    EmptyHeimdallUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BifrostSettings {
    pub websocket_port: u16,
    pub heimdall_url: String,
    pub max_connections: u32,
    pub message_timeout_seconds: u64,
}

impl Default for BifrostSettings {
    fn default() -> Self {
        Self {
            websocket_port: 8080,
            heimdall_url: "http://localhost:50051".to_string(),
            max_connections: 1000,
            message_timeout_seconds: 30,
        }
    }
}

impl BifrostSettings {
    /// Validates settings. Returns `Ok(())` if valid.
    pub fn validate(&self) -> Result<(), SettingsError> {
        if self.websocket_port == 0 {
            return Err(SettingsError::InvalidPort(self.websocket_port));
        }
        if self.max_connections == 0 {
            return Err(SettingsError::InvalidMaxConnections(self.max_connections));
        }
        if self.message_timeout_seconds == 0 {
            return Err(SettingsError::InvalidMessageTimeout(self.message_timeout_seconds));
        }
        if self.heimdall_url.trim().is_empty() {
            return Err(SettingsError::EmptyHeimdallUrl);
        }
        Ok(())
    }
}

pub struct SettingsManager {
    config_path: PathBuf,
    settings: Arc<RwLock<BifrostSettings>>,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            config_path,
            settings: Arc::new(RwLock::new(BifrostSettings::default())),
        }
    }

    pub async fn load(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if self.config_path.exists() {
            let content = tokio::fs::read_to_string(&self.config_path).await?;
            let settings: BifrostSettings = serde_json::from_str(&content)?;
            settings.validate().map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
            *self.settings.write().await = settings;
            info!("Configuration loaded from {}", self.config_path.display());
        } else {
            let default_settings = BifrostSettings::default();
            let content = serde_json::to_string_pretty(&default_settings)?;
            tokio::fs::write(&self.config_path, content).await?;
            *self.settings.write().await = default_settings;
            info!("Default configuration created at {}", self.config_path.display());
        }
        Ok(())
    }

    pub async fn get(&self) -> BifrostSettings {
        self.settings.read().await.clone()
    }

    pub fn start_hot_reload(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let config_path = self.config_path.clone();
        let settings = Arc::clone(&self.settings);
        
        let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |result: Result<Event, notify::Error>| {
            match result {
                Ok(event) => {
                    if let EventKind::Modify(_) = event.kind {
                        if event.paths.iter().any(|p| p == &config_path) {
                            info!("Configuration file changed, reloading...");
                            let rt = tokio::runtime::Runtime::new().unwrap();
                            if let Err(e) = rt.block_on(async {
                                let content = tokio::fs::read_to_string(&config_path).await?;
                                let new_settings: BifrostSettings = serde_json::from_str(&content)?;
                                new_settings
                                    .validate()
                                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                                *settings.write().await = new_settings;
                                Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
                            }) {
                                error!("Failed to reload configuration: {}", e);
                            } else {
                                info!("Configuration reloaded successfully");
                            }
                        }
                    }
                }
                Err(e) => error!("File watcher error: {}", e),
            }
        })?;
        
        watcher.watch(&self.config_path.parent().unwrap(), RecursiveMode::NonRecursive)?;
        info!("Hot-reload watcher started for configuration");
        
        Ok(())
    }
}
