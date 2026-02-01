use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tracing::{info, error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("grpc_port must be non-zero")]
    InvalidPort,
    #[error("qdrant_url must be non-empty")]
    EmptyQdrantUrl,
    #[error("embedding_model must be non-empty")]
    EmptyEmbeddingModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrekiSettings {
    pub grpc_port: u16,
    pub qdrant_url: String,
    pub embedding_model: String,
}

impl FrekiSettings {
    pub fn validate(&self) -> Result<(), SettingsError> {
        if self.grpc_port == 0 {
            return Err(SettingsError::InvalidPort);
        }
        if self.qdrant_url.trim().is_empty() {
            return Err(SettingsError::EmptyQdrantUrl);
        }
        if self.embedding_model.trim().is_empty() {
            return Err(SettingsError::EmptyEmbeddingModel);
        }
        Ok(())
    }
}

impl Default for FrekiSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50053,
            qdrant_url: "http://localhost:6333".to_string(),
            embedding_model: "all-MiniLM-L6-v2".to_string(),
        }
    }
}

pub struct SettingsManager {
    config_path: PathBuf,
    settings: Arc<RwLock<FrekiSettings>>,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            config_path,
            settings: Arc::new(RwLock::new(FrekiSettings::default())),
        }
    }

    pub async fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config_path.exists() {
            let content = tokio::fs::read_to_string(&self.config_path).await?;
            let settings: FrekiSettings = serde_json::from_str(&content)?;
            settings.validate()?;
            *self.settings.write().await = settings;
            info!("Configuration loaded from {}", self.config_path.display());
        } else {
            let default_settings = FrekiSettings::default();
            let content = serde_json::to_string_pretty(&default_settings)?;
            tokio::fs::write(&self.config_path, content).await?;
            *self.settings.write().await = default_settings;
            info!("Default configuration created at {}", self.config_path.display());
        }
        Ok(())
    }

    pub async fn get(&self) -> FrekiSettings {
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
                                let new_settings: FrekiSettings = serde_json::from_str(&content)?;
                                new_settings.validate()?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_default_ok() {
        let s = FrekiSettings::default();
        assert!(s.validate().is_ok());
    }

    #[test]
    fn test_validate_invalid_port() {
        let mut s = FrekiSettings::default();
        s.grpc_port = 0;
        assert!(matches!(s.validate(), Err(SettingsError::InvalidPort)));
    }

    #[test]
    fn test_validate_empty_qdrant_url() {
        let mut s = FrekiSettings::default();
        s.qdrant_url = String::new();
        assert!(matches!(s.validate(), Err(SettingsError::EmptyQdrantUrl)));
    }

    #[test]
    fn test_validate_empty_embedding_model() {
        let mut s = FrekiSettings::default();
        s.embedding_model = "   ".to_string();
        assert!(matches!(s.validate(), Err(SettingsError::EmptyEmbeddingModel)));
    }
}
