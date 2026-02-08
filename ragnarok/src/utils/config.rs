use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tracing::{info, error};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdinConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThorConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeriConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrekiConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuginnConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuninnConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkuldConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GladsheimConfig {
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagnarokSettings {
    pub odin: OdinConfig,
    #[serde(default)]
    pub thor: Option<ThorConfig>,
    #[serde(default)]
    pub geri: Option<GeriConfig>,
    #[serde(default)]
    pub freki: Option<FrekiConfig>,
    #[serde(default)]
    pub huginn: Option<HuginnConfig>,
    #[serde(default)]
    pub muninn: Option<MuninnConfig>,
    #[serde(default)]
    pub skuld: Option<SkuldConfig>,
    pub gladsheim: GladsheimConfig,
}

impl Default for RagnarokSettings {
    fn default() -> Self {
        Self {
            odin: OdinConfig {
                port: 50051,
            },
            thor: None,
            geri: None,
            freki: None,
            huginn: None,
            muninn: None,
            gladsheim: GladsheimConfig {
                port: 50050,
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
    settings: Arc<RwLock<RagnarokSettings>>,
    config_path: PathBuf,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            settings: Arc::new(RwLock::new(RagnarokSettings::default())),
            config_path,
        }
    }

    pub async fn load(&self) -> Result<(), SettingsError> {
        if !self.config_path.exists() {
            let default_settings = RagnarokSettings::default();
            let json = serde_json::to_string_pretty(&default_settings)?;
            if let Some(parent) = self.config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&self.config_path, json)?;
            info!("Created default config file at {:?}", self.config_path);
        }

        let content = std::fs::read_to_string(&self.config_path)?;
        let settings: RagnarokSettings = serde_json::from_str(&content)?;
        
        self.validate(&settings)?;
        
        *self.settings.write().await = settings;
        info!("Settings loaded from {:?}", self.config_path);
        
        Ok(())
    }

    pub async fn get(&self) -> RagnarokSettings {
        self.settings.read().await.clone()
    }

    pub async fn reload(&self) -> Result<(), SettingsError> {
        self.load().await
    }

    pub fn validate(&self, settings: &RagnarokSettings) -> Result<(), SettingsError> {
        if settings.odin.port == 0 {
            return Err(SettingsError::ValidationError("odin.port cannot be 0".to_string()));
        }
        if settings.gladsheim.port == 0 {
            return Err(SettingsError::ValidationError("gladsheim.port cannot be 0".to_string()));
        }
        if let Some(ref thor) = settings.thor {
            if thor.port == 0 {
                return Err(SettingsError::ValidationError("thor.port cannot be 0".to_string()));
            }
        }
        if let Some(ref geri) = settings.geri {
            if geri.port == 0 {
                return Err(SettingsError::ValidationError("geri.port cannot be 0".to_string()));
            }
        }
        if let Some(ref freki) = settings.freki {
            if freki.port == 0 {
                return Err(SettingsError::ValidationError("freki.port cannot be 0".to_string()));
            }
        }
        if let Some(ref huginn) = settings.huginn {
            if huginn.port == 0 {
                return Err(SettingsError::ValidationError("huginn.port cannot be 0".to_string()));
            }
        }
        if let Some(ref muninn) = settings.muninn {
            if muninn.port == 0 {
                return Err(SettingsError::ValidationError("muninn.port cannot be 0".to_string()));
            }
        }
        if let Some(ref skuld) = settings.skuld {
            if skuld.port == 0 {
                return Err(SettingsError::ValidationError("skuld.port cannot be 0".to_string()));
            }
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
                                let new_settings: RagnarokSettings = serde_json::from_str(&content)?;
                                
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
