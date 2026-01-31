use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use config::{Config, ConfigError, File, FileFormat};
use notify::{Watcher, RecommendedWatcher};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoints {
    pub nornen: String,
    pub heidrun: String,
    pub mimir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NidhoggrSettings {
    pub grpc_port: u16,
    pub websocket_port: u16,
    pub tls_cert_path: Option<String>,
    pub tls_key_path: Option<String>,
    pub rate_limit_per_minute: u32,
    pub rate_limit_per_hour: u32,
    pub max_connections: u32,
    pub service_endpoints: ServiceEndpoints,
}

impl Default for ServiceEndpoints {
    fn default() -> Self {
        Self {
            nornen: "http://localhost:50055".to_string(),
            heidrun: "http://localhost:50057".to_string(),
            mimir: "http://localhost:50059".to_string(),
        }
    }
}

impl Default for NidhoggrSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50061,
            websocket_port: 50062,
            tls_cert_path: None,
            tls_key_path: None,
            rate_limit_per_minute: 60,
            rate_limit_per_hour: 1000,
            max_connections: 1000,
            service_endpoints: ServiceEndpoints::default(),
        }
    }
}

impl NidhoggrSettings {
    pub fn load(path: Option<PathBuf>) -> Result<Self, ConfigError> {
        let mut builder = Config::builder()
            .set_default("grpc_port", 50061)?
            .set_default("websocket_port", 50062)?
            .set_default("rate_limit_per_minute", 60)?
            .set_default("rate_limit_per_hour", 1000)?
            .set_default("max_connections", 1000)?
            .set_default("service_endpoints.nornen", "http://localhost:50055")?
            .set_default("service_endpoints.heidrun", "http://localhost:50057")?
            .set_default("service_endpoints.mimir", "http://localhost:50059")?;

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
        if self.grpc_port == 0 {
            return Err("grpc_port cannot be 0".to_string());
        }
        if self.websocket_port == 0 {
            return Err("websocket_port cannot be 0".to_string());
        }
        if self.websocket_port == self.grpc_port {
            return Err("websocket_port and grpc_port must be different".to_string());
        }
        if self.rate_limit_per_minute == 0 {
            return Err("rate_limit_per_minute cannot be 0".to_string());
        }
        if self.rate_limit_per_hour == 0 {
            return Err("rate_limit_per_hour cannot be 0".to_string());
        }
        if self.max_connections == 0 {
            return Err("max_connections cannot be 0".to_string());
        }
        Ok(())
    }
}

pub struct SettingsManager {
    settings: Arc<RwLock<NidhoggrSettings>>,
}

impl SettingsManager {
    pub fn new(settings: NidhoggrSettings) -> Result<Self, String> {
        settings.validate()?;
        Ok(Self {
            settings: Arc::new(RwLock::new(settings)),
        })
    }

    pub async fn get(&self) -> NidhoggrSettings {
        self.settings.read().await.clone()
    }

    pub async fn reload(&self, new_settings: NidhoggrSettings) -> Result<(), String> {
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
                            match NidhoggrSettings::load(Some(config_path.clone())) {
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

