use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tracing::{info, warn, error};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecuritySettings {
    pub encryption_algorithm: String,
    pub key_rotation_days: u32,
    pub enable_audit_logging: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionSettings {
    pub default_retention_days: u32,
    pub enable_auto_deletion: bool,
    pub anonymize_on_deletion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MimirSettings {
    pub grpc_port: u16,
    pub database: DatabaseConfig,
    pub security: SecuritySettings,
    pub data_retention: DataRetentionSettings,
    pub encryption_key_path: String,
}

impl Default for MimirSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50059,
            database: DatabaseConfig {
                url: "postgres://localhost/mimir".to_string(),
                max_connections: 10,
                min_connections: 2,
            },
            security: SecuritySettings {
                encryption_algorithm: "AES-256-GCM".to_string(),
                key_rotation_days: 90,
                enable_audit_logging: true,
            },
            data_retention: DataRetentionSettings {
                default_retention_days: 365,
                enable_auto_deletion: false,
                anonymize_on_deletion: true,
            },
            encryption_key_path: "keys/mimir.key".to_string(),
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
    settings: Arc<RwLock<MimirSettings>>,
    config_path: PathBuf,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            settings: Arc::new(RwLock::new(MimirSettings::default())),
            config_path,
        }
    }

    pub async fn load(&self) -> Result<(), SettingsError> {
        if !self.config_path.exists() {
            // Create default config if it doesn't exist
            let default_settings = MimirSettings::default();
            let json = serde_json::to_string_pretty(&default_settings)?;
            std::fs::write(&self.config_path, json)?;
            info!("Created default config file at {:?}", self.config_path);
        }

        let content = std::fs::read_to_string(&self.config_path)?;
        let settings: MimirSettings = serde_json::from_str(&content)?;
        
        // Validate settings
        self.validate(&settings)?;
        
        *self.settings.write().await = settings;
        info!("Settings loaded from {:?}", self.config_path);
        
        Ok(())
    }

    pub async fn get(&self) -> MimirSettings {
        self.settings.read().await.clone()
    }

    pub async fn reload(&self) -> Result<(), SettingsError> {
        self.load().await
    }

    pub fn validate(&self, settings: &MimirSettings) -> Result<(), SettingsError> {
        if settings.grpc_port == 0 {
            return Err(SettingsError::ValidationError("grpc_port cannot be 0".to_string()));
        }
        if settings.database.max_connections == 0 {
            return Err(SettingsError::ValidationError("max_connections cannot be 0".to_string()));
        }
        if settings.database.min_connections > settings.database.max_connections {
            return Err(SettingsError::ValidationError("min_connections cannot exceed max_connections".to_string()));
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
                                let new_settings: MimirSettings = serde_json::from_str(&content)?;
                                
                                // Validate before applying
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
