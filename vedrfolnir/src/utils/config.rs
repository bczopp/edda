use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use config::{Config, ConfigError, File, FileFormat};
use notify::{Watcher, RecommendedWatcher};
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VedrfolnirSettings {
    pub yggdrasil_ratatoskr_url: String,
    pub yggdrasil_grpc_url: String,
    pub heimdall_url: Option<String>,
    pub connection_timeout_secs: u64,
    pub retry_attempts: u32,
}

impl Default for VedrfolnirSettings {
    fn default() -> Self {
        Self {
            yggdrasil_ratatoskr_url: "wss://yggdrasil.example.com/ratatoskr".to_string(),
            yggdrasil_grpc_url: "http://yggdrasil.example.com:50000".to_string(),
            connection_timeout_secs: 30,
            retry_attempts: 3,
        }
    }
}

impl VedrfolnirSettings {
    pub fn load(path: Option<PathBuf>) -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .set_default("yggdrasil_ratatoskr_url", "wss://yggdrasil.example.com/ratatoskr")?
            .set_default("yggdrasil_grpc_url", "http://yggdrasil.example.com:50000")?
            .set_default("heimdall_url", "http://heimdall.example.com:50050")?
            .set_default("connection_timeout_secs", 30)?
            .set_default("retry_attempts", 3)?;

        if let Some(config_path) = path {
            if config_path.exists() {
                builder = builder.add_source(File::new(
                    config_path.to_str().unwrap(),
                    FileFormat::Json,
                ));
            }
        }

        builder.build()?.try_deserialize()
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.connection_timeout_secs == 0 {
            return Err("connection_timeout_secs cannot be 0".to_string());
        }
        if self.retry_attempts == 0 {
            return Err("retry_attempts cannot be 0".to_string());
        }
        Ok(())
    }
}

pub struct SettingsManager {
    settings: Arc<RwLock<VedrfolnirSettings>>,
}

impl SettingsManager {
    pub fn new(settings: VedrfolnirSettings) -> Result<Self, String> {
        settings.validate()?;
        Ok(Self {
            settings: Arc::new(RwLock::new(settings)),
        })
    }

    pub async fn get(&self) -> VedrfolnirSettings {
        self.settings.read().await.clone()
    }

    pub async fn reload(&self, new_settings: VedrfolnirSettings) -> Result<(), String> {
        new_settings.validate()?;
        *self.settings.write().await = new_settings;
        info!("Settings reloaded successfully");
        Ok(())
    }

    pub fn start_watcher(&self, config_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let settings = self.settings.clone();
        let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
            match res {
                Ok(event) => {
                    if event.kind.is_modify() {
                        let rt = tokio::runtime::Runtime::new().unwrap();
                        rt.block_on(async {
                            match VedrfolnirSettings::load(Some(config_path.clone())) {
                                Ok(new_settings) => {
                                    if let Err(e) = new_settings.validate() {
                                        error!("Failed to validate new settings: {}", e);
                                        return;
                                    }
                                    *settings.write().await = new_settings;
                                    info!("Settings reloaded successfully");
                                }
                                Err(e) => {
                                    error!("Failed to load settings: {}", e);
                                }
                            }
                        });
                    }
                }
                Err(e) => error!("Watch error: {}", e),
            }
        })?;

        watcher.watch(&config_path, notify::RecursiveMode::NonRecursive)?;
        Ok(())
    }
}
