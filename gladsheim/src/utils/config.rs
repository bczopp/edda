use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tracing::{info, error};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GladsheimSettings {
    pub grpc_port: u16,
    pub service_directory: String,
    pub auto_start: bool,
    pub max_memory_mb: u64,
    pub max_cpu_percent: f64,
    pub health_check_interval_secs: u64,
}

impl Default for GladsheimSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50066,
            service_directory: "./services".to_string(),
            auto_start: false,
            max_memory_mb: 1024,
            max_cpu_percent: 50.0,
            health_check_interval_secs: 60,
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
    settings: Arc<RwLock<GladsheimSettings>>,
    config_path: PathBuf,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            settings: Arc::new(RwLock::new(GladsheimSettings::default())),
            config_path,
        }
    }

    pub async fn load(&self) -> Result<(), SettingsError> {
        if !self.config_path.exists() {
            let default_settings = GladsheimSettings::default();
            let json = serde_json::to_string_pretty(&default_settings)?;
            if let Some(parent) = self.config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(&self.config_path, json)?;
            info!("Created default config file at {:?}", self.config_path);
        }

        let content = std::fs::read_to_string(&self.config_path)?;
        let settings: GladsheimSettings = serde_json::from_str(&content)?;
        
        self.validate(&settings)?;
        
        *self.settings.write().await = settings;
        info!("Settings loaded from {:?}", self.config_path);
        
        Ok(())
    }

    pub async fn get(&self) -> GladsheimSettings {
        self.settings.read().await.clone()
    }

    pub async fn reload(&self) -> Result<(), SettingsError> {
        self.load().await
    }

    pub fn validate(&self, settings: &GladsheimSettings) -> Result<(), SettingsError> {
        if settings.grpc_port == 0 {
            return Err(SettingsError::ValidationError("grpc_port cannot be 0".to_string()));
        }
        if settings.max_memory_mb == 0 {
            return Err(SettingsError::ValidationError("max_memory_mb cannot be 0".to_string()));
        }
        if settings.max_cpu_percent < 0.0 || settings.max_cpu_percent > 100.0 {
            return Err(SettingsError::ValidationError("max_cpu_percent must be between 0.0 and 100.0".to_string()));
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
                                let new_settings: GladsheimSettings = serde_json::from_str(&content)?;
                                
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
