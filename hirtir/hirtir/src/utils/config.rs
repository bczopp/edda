use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tracing::{info, error};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DainnConfig {
    pub index_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DvalinnConfig {
    pub schema_directory: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuneyrrConfig {
    pub batch_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurathrorConfig {
    pub s3_endpoint: String,
    pub s3_bucket: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HirtirSettings {
    pub grpc_port: u16,
    pub dainn: DainnConfig,
    pub dvalinn: DvalinnConfig,
    pub duneyrr: DuneyrrConfig,
    pub durathror: DurathrorConfig,
}

impl Default for HirtirSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50067,
            dainn: DainnConfig {
                index_path: "./indexes".to_string(),
            },
            dvalinn: DvalinnConfig {
                schema_directory: "./schemas".to_string(),
            },
            duneyrr: DuneyrrConfig {
                batch_size: 100,
            },
            durathror: DurathrorConfig {
                s3_endpoint: "http://localhost:9000".to_string(),
                s3_bucket: "hirtir-archive".to_string(),
            },
        }
    }
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("JSON parse error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub struct SettingsManager {
    settings: Arc<RwLock<HirtirSettings>>,
    config_path: PathBuf,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            settings: Arc::new(RwLock::new(HirtirSettings::default())),
            config_path,
        }
    }

    pub async fn load(&self) -> Result<(), SettingsError> {
        if !self.config_path.exists() {
            let default_settings = HirtirSettings::default();
            let json = serde_json::to_string_pretty(&default_settings)?;
            if let Some(parent) = self.config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&self.config_path, json)?;
            info!("Created default config file at {:?}", self.config_path);
        }

        let content = std::fs::read_to_string(&self.config_path)?;
        let settings: HirtirSettings = serde_json::from_str(&content)?;
        
        self.validate(&settings)?;
        
        *self.settings.write().await = settings;
        info!("Settings loaded from {:?}", self.config_path);
        
        Ok(())
    }

    pub async fn get(&self) -> HirtirSettings {
        self.settings.read().await.clone()
    }

    pub async fn reload(&self) -> Result<(), SettingsError> {
        self.load().await
    }

    pub fn validate(&self, settings: &HirtirSettings) -> Result<(), SettingsError> {
        if settings.grpc_port == 0 {
            return Err(SettingsError::ValidationError("grpc_port cannot be 0".to_string()));
        }
        if settings.duneyrr.batch_size == 0 {
            return Err(SettingsError::ValidationError("batch_size cannot be 0".to_string()));
        }
        Ok(())
    }

    pub fn start_hot_reload(&self) -> Result<(), Box<dyn std::error::Error>> {
        let settings = Arc::clone(&self.settings);
        let config_path = self.config_path.clone();
        
        let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |result: Result<Event, notify::Error>| {
            match result {
                Ok(event) => {
                    if matches!(event.kind, EventKind::Modify(_)) {
                        let path = event.paths[0].clone();
                        if path == config_path {
                            info!("Config file changed, reloading...");
                            let rt = tokio::runtime::Runtime::new().unwrap();
                            if let Err(e) = rt.block_on(async {
                                let content = std::fs::read_to_string(&config_path)?;
                                let new_settings: HirtirSettings = serde_json::from_str(&content)?;
                                
                                let manager = SettingsManager::new(config_path.clone());
                                manager.validate(&new_settings)?;
                                
                                *settings.write().await = new_settings;
                                info!("Settings reloaded successfully");
                                Ok::<(), Box<dyn std::error::Error>>(())
                            }) {
                                error!("Failed to reload settings: {}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Watcher error: {}", e);
                }
            }
        })?;
        
        watcher.watch(&self.config_path.parent().unwrap_or(std::path::Path::new(".")), RecursiveMode::NonRecursive)?;
        info!("Hot reload watcher started for {:?}", self.config_path);
        
        Ok(())
    }
}
