use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkuldSettings {
    pub grpc_port: u16,
    pub database_url: String,
}

impl Default for SkuldSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50055,
            database_url: "postgres://localhost/skuld".to_string(),
        }
    }
}

pub struct SettingsManager {
    config_path: PathBuf,
    settings: Arc<RwLock<SkuldSettings>>,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            config_path,
            settings: Arc::new(RwLock::new(SkuldSettings::default())),
        }
    }

    pub async fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config_path.exists() {
            let content = tokio::fs::read_to_string(&self.config_path).await?;
            let settings: SkuldSettings = serde_json::from_str(&content)?;
            *self.settings.write().await = settings;
            info!("Configuration loaded from {}", self.config_path.display());
        } else {
            let default_settings = SkuldSettings::default();
            let content = serde_json::to_string_pretty(&default_settings)?;
            tokio::fs::write(&self.config_path, content).await?;
            *self.settings.write().await = default_settings;
            info!("Default configuration created at {}", self.config_path.display());
        }
        Ok(())
    }

    pub async fn get(&self) -> SkuldSettings {
        self.settings.read().await.clone()
    }

    pub fn start_hot_reload(&self) -> Result<(), Box<dyn std::error::Error>> {
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
                                let new_settings: SkuldSettings = serde_json::from_str(&content)?;
                                *settings.write().await = new_settings;
                                Ok::<(), Box<dyn std::error::Error>>(())
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
