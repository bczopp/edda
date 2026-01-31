use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tracing::{info, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicySettings {
    pub fail_safe: bool,
    pub default_deny: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfiguration {
    pub heimdall_token_expiration_hours: u64,
    pub session_token_expiration_hours: u64,
    pub refresh_token_expiration_days: u64,
    pub proactive_renewal_minutes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSystemSettings {
    pub enable_rbac: bool,
    pub enable_conditional_permissions: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionManagementSettings {
    pub session_timeout_hours: u64,
    pub enable_hijacking_detection: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub name: String,
    pub client_id: String,
    pub client_secret: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthSettings {
    pub providers: Vec<OAuthProvider>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeimdallSettings {
    pub security_policy: SecurityPolicySettings,
    pub token_configuration: TokenConfiguration,
    pub permission_system: PermissionSystemSettings,
    pub session_management: SessionManagementSettings,
    #[serde(default)]
    pub oauth: Option<OAuthSettings>,
    pub grpc_port: u16,
    pub database_url: String,
}

impl Default for HeimdallSettings {
    fn default() -> Self {
        Self {
            security_policy: SecurityPolicySettings {
                fail_safe: true,
                default_deny: true,
            },
            token_configuration: TokenConfiguration {
                heimdall_token_expiration_hours: 24,
                session_token_expiration_hours: 1,
                refresh_token_expiration_days: 30,
                proactive_renewal_minutes: 5,
            },
            permission_system: PermissionSystemSettings {
                enable_rbac: true,
                enable_conditional_permissions: true,
            },
            session_management: SessionManagementSettings {
                session_timeout_hours: 1,
                enable_hijacking_detection: true,
            },
            oauth: None,
            grpc_port: 50051,
            database_url: "postgres://localhost/heimdall".to_string(),
        }
    }
}

pub struct SettingsManager {
    settings: Arc<RwLock<HeimdallSettings>>,
    config_path: PathBuf,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            settings: Arc::new(RwLock::new(HeimdallSettings::default())),
            config_path,
        }
    }

    pub async fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config_path.exists() {
            // Create default config if it doesn't exist
            let default_settings = HeimdallSettings::default();
            let json = serde_json::to_string_pretty(&default_settings)?;
            std::fs::write(&self.config_path, json)?;
            info!("Created default config file at {:?}", self.config_path);
        }

        let content = std::fs::read_to_string(&self.config_path)?;
        let settings: HeimdallSettings = serde_json::from_str(&content)?;
        validate_settings(&settings)?;
        *self.settings.write().await = settings;
        info!("Settings loaded from {:?}", self.config_path);
        
        Ok(())
    }

    pub async fn get(&self) -> HeimdallSettings {
        self.settings.read().await.clone()
    }

    pub async fn reload(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.load().await
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
                                let new_settings: HeimdallSettings = serde_json::from_str(&content)?;
                                validate_settings(&new_settings)?;
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
                    error!("Error watching config file: {}", e);
                }
            }
        })?;

        watcher.watch(&self.config_path.parent().unwrap(), RecursiveMode::NonRecursive)?;
        info!("Hot-reload watcher started for {:?}", self.config_path);
        
        Ok(())
    }

}

/// Validates settings (token expirations, port, database_url). Used by load() and hot-reload.
pub fn validate_settings(settings: &HeimdallSettings) -> Result<(), Box<dyn std::error::Error>> {
    if settings.token_configuration.heimdall_token_expiration_hours == 0 {
        return Err("heimdall_token_expiration_hours must be > 0".into());
    }
    if settings.token_configuration.session_token_expiration_hours == 0 {
        return Err("session_token_expiration_hours must be > 0".into());
    }
    if settings.token_configuration.refresh_token_expiration_days == 0 {
        return Err("refresh_token_expiration_days must be > 0".into());
    }
    if settings.grpc_port == 0 {
        return Err("grpc_port must be > 0".into());
    }
    if settings.database_url.is_empty() {
        return Err("database_url cannot be empty".into());
    }
    Ok(())
}
