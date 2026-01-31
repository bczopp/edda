use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tracing::{info, error};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdinConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioConfig {
    pub input_device: Option<String>,
    pub output_device: Option<String>,
    pub sample_rate: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidgardSettings {
    pub odin: OdinConfig,
    pub audio: AudioConfig,
}

impl Default for MidgardSettings {
    fn default() -> Self {
        Self {
            odin: OdinConfig {
                address: "127.0.0.1".to_string(),
                port: 50051,
            },
            audio: AudioConfig {
                input_device: None,
                output_device: None,
                sample_rate: 44100,
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
    settings: Arc<RwLock<MidgardSettings>>,
    config_path: PathBuf,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            settings: Arc::new(RwLock::new(MidgardSettings::default())),
            config_path,
        }
    }

    pub async fn load(&self) -> Result<(), SettingsError> {
        if !self.config_path.exists() {
            let default_settings = MidgardSettings::default();
            let json = serde_json::to_string_pretty(&default_settings)?;
            if let Some(parent) = self.config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&self.config_path, json)?;
            info!("Created default config file at {:?}", self.config_path);
        }

        let content = std::fs::read_to_string(&self.config_path)?;
        let settings: MidgardSettings = serde_json::from_str(&content)?;
        
        self.validate(&settings)?;
        
        *self.settings.write().await = settings;
        info!("Settings loaded from {:?}", self.config_path);
        
        Ok(())
    }

    pub async fn get(&self) -> MidgardSettings {
        self.settings.read().await.clone()
    }

    pub async fn reload(&self) -> Result<(), SettingsError> {
        self.load().await
    }

    pub fn validate(&self, settings: &MidgardSettings) -> Result<(), SettingsError> {
        if settings.odin.port == 0 {
            return Err(SettingsError::ValidationError("odin.port cannot be 0".to_string()));
        }
        if settings.audio.sample_rate == 0 {
            return Err(SettingsError::ValidationError("audio.sample_rate cannot be 0".to_string()));
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
                                let new_settings: MidgardSettings = serde_json::from_str(&content)?;
                                
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
