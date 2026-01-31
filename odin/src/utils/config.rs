use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use notify::{Watcher, RecommendedWatcher, RecursiveMode, Event, EventKind};
use tracing::{info, warn, error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub quality_level: String, // "low" | "medium" | "high" | "custom"
    #[serde(default)]
    pub max_cost: Option<f64>,
    #[serde(default)]
    pub max_latency_ms: Option<u64>,
    #[serde(default)]
    pub model_preferences: Vec<String>,
    #[serde(default)]
    pub provider_preferences: Vec<String>,
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            quality_level: "medium".to_string(),
            max_cost: None,
            max_latency_ms: None,
            model_preferences: vec![],
            provider_preferences: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSelection {
    pub auto_select: bool,
    #[serde(default)]
    pub min_quality: Option<f64>,
    pub fair_distribution: bool,
}

impl Default for ProviderSelection {
    fn default() -> Self {
        Self {
            auto_select: true,
            min_quality: None,
            fair_distribution: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValkyriesPluginConfig {
    pub enabled: bool,
    pub parallel_agents: u32,
    #[serde(default)]
    pub llm_config: serde_json::Value,
}

impl Default for ValkyriesPluginConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            parallel_agents: 3,
            llm_config: serde_json::json!({}),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriggPluginConfig {
    pub enabled: bool,
    pub chat_direct: bool,
}

impl Default for FriggPluginConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            chat_direct: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginsConfig {
    #[serde(default)]
    pub valkyries: ValkyriesPluginConfig,
    #[serde(default)]
    pub frigg: FriggPluginConfig,
}

impl Default for PluginsConfig {
    fn default() -> Self {
        Self {
            valkyries: ValkyriesPluginConfig::default(),
            frigg: FriggPluginConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub auto_connect: bool,
    pub yggdrasil_enabled: bool,
    pub asgard_relay: bool,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            auto_connect: true,
            yggdrasil_enabled: false,
            asgard_relay: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSyncConfig {
    pub enabled: bool,
    #[serde(default)]
    pub sync_interval_ms: Option<u64>,
    pub selective_propagation: bool,
}

impl Default for StateSyncConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sync_interval_ms: Some(1000),
            selective_propagation: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFlags {
    pub frigg_direct: bool,
    pub valkyries_direct: bool,
}

impl Default for ChatFlags {
    fn default() -> Self {
        Self {
            frigg_direct: false,
            valkyries_direct: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OdinSettings {
    #[serde(default)]
    pub user_preferences: UserPreferences,
    #[serde(default)]
    pub provider_selection: ProviderSelection,
    #[serde(default)]
    pub plugins: PluginsConfig,
    #[serde(default)]
    pub network: NetworkConfig,
    #[serde(default)]
    pub state_sync: StateSyncConfig,
    #[serde(default)]
    pub chat_flags: ChatFlags,
    pub grpc_port: u16,
    #[serde(default)]
    pub service_urls: ServiceUrls,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceUrls {
    #[serde(default)]
    pub thor: Option<String>,
    #[serde(default)]
    pub freki: Option<String>,
    #[serde(default)]
    pub geri: Option<String>,
    #[serde(default)]
    pub huginn: Option<String>,
    #[serde(default)]
    pub muninn: Option<String>,
    #[serde(default)]
    pub loki: Option<String>,
    #[serde(default)]
    pub heimdall: Option<String>,
    #[serde(default)]
    pub skuld: Option<String>,
    #[serde(default)]
    pub bifrost: Option<String>,
    /// Frigg plugin (healthcare); used when plugins.frigg.enabled.
    #[serde(default)]
    pub frigg: Option<String>,
    /// Valkyries plugin (coding); used when plugins.valkyries.enabled.
    #[serde(default)]
    pub valkyries: Option<String>,
}

impl Default for ServiceUrls {
    fn default() -> Self {
        Self {
            thor: Some("http://localhost:50052".to_string()),
            freki: Some("http://localhost:50053".to_string()),
            geri: Some("http://localhost:50054".to_string()),
            huginn: Some("http://localhost:50055".to_string()),
            muninn: Some("http://localhost:50056".to_string()),
            loki: Some("http://localhost:50057".to_string()),
            heimdall: Some("http://localhost:50051".to_string()),
            skuld: Some("http://localhost:50058".to_string()),
            bifrost: Some("http://localhost:50059".to_string()),
            frigg: None,
            valkyries: None,
        }
    }
}

impl Default for OdinSettings {
    fn default() -> Self {
        Self {
            user_preferences: UserPreferences::default(),
            provider_selection: ProviderSelection::default(),
            plugins: PluginsConfig::default(),
            network: NetworkConfig::default(),
            state_sync: StateSyncConfig::default(),
            chat_flags: ChatFlags::default(),
            grpc_port: 50050,
            service_urls: ServiceUrls::default(),
        }
    }
}

pub struct SettingsManager {
    settings: Arc<RwLock<OdinSettings>>,
    config_path: PathBuf,
}

impl SettingsManager {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            settings: Arc::new(RwLock::new(OdinSettings::default())),
            config_path,
        }
    }

    pub async fn load(&self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.config_path.exists() {
            // Create default config if it doesn't exist
            let default_settings = OdinSettings::default();
            let json = serde_json::to_string_pretty(&default_settings)?;
            
            // Create parent directory if it doesn't exist
            if let Some(parent) = self.config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            std::fs::write(&self.config_path, json)?;
            info!("Created default config file at {:?}", self.config_path);
        }

        let content = std::fs::read_to_string(&self.config_path)?;
        let settings: OdinSettings = serde_json::from_str(&content)?;
        
        // Validate settings
        self.validate(&settings)?;
        
        *self.settings.write().await = settings;
        info!("Settings loaded from {:?}", self.config_path);
        
        Ok(())
    }

    pub async fn get(&self) -> OdinSettings {
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
                                let new_settings: OdinSettings = serde_json::from_str(&content)?;
                                
                                // Validate before applying
                                // TODO: Add validation
                                
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

        if let Some(parent) = self.config_path.parent() {
            watcher.watch(parent, RecursiveMode::NonRecursive)?;
        }
        info!("Hot-reload watcher started for {:?}", self.config_path);
        
        Ok(())
    }

    fn validate(&self, settings: &OdinSettings) -> Result<(), Box<dyn std::error::Error>> {
        // Validate quality_level
        let valid_quality_levels = vec!["low", "medium", "high", "custom"];
        if !valid_quality_levels.contains(&settings.user_preferences.quality_level.as_str()) {
            return Err(format!("quality_level must be one of: {:?}", valid_quality_levels).into());
        }
        
        // Validate max_cost
        if let Some(max_cost) = settings.user_preferences.max_cost {
            if max_cost < 0.0 {
                return Err("max_cost must be >= 0".into());
            }
        }
        
        // Validate max_latency_ms
        if let Some(max_latency) = settings.user_preferences.max_latency_ms {
            if max_latency == 0 {
                return Err("max_latency_ms must be > 0".into());
            }
        }
        
        // Validate min_quality
        if let Some(min_quality) = settings.provider_selection.min_quality {
            if min_quality < 0.0 || min_quality > 1.0 {
                return Err("min_quality must be between 0.0 and 1.0".into());
            }
        }
        
        // Validate parallel_agents
        if settings.plugins.valkyries.parallel_agents == 0 {
            return Err("parallel_agents must be > 0".into());
        }
        
        // Validate sync_interval_ms
        if let Some(interval) = settings.state_sync.sync_interval_ms {
            if interval == 0 {
                return Err("sync_interval_ms must be > 0".into());
            }
        }
        
        // Validate port
        if settings.grpc_port == 0 {
            return Err("grpc_port must be > 0".into());
        }
        
        Ok(())
    }
}
