//! Settings schema (Phase 1.3.1) – minimal JSON/TOML.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LokiConfig {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub ssid: String,
    pub password: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CapabilityConfiguration {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkResilienceSettings {
    pub max_retries: u32,
    pub backoff_ms: u32,
}

impl Default for NetworkResilienceSettings {
    fn default() -> Self {
        Self {
            max_retries: 3,
            backoff_ms: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_kb: u32,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_kb: 256,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OtaUpdateSettings {
    pub enabled: bool,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JotunheimSettings {
    pub loki: LokiConfig,
    pub network: NetworkConfig,
    #[serde(default)]
    pub capability_configuration: CapabilityConfiguration,
    #[serde(default)]
    pub network_resilience_settings: NetworkResilienceSettings,
    #[serde(default)]
    pub resource_limits: ResourceLimits,
    #[serde(default)]
    pub ota_update_settings: OtaUpdateSettings,
}

impl Default for JotunheimSettings {
    fn default() -> Self {
        Self {
            loki: LokiConfig {
                address: "127.0.0.1".to_string(),
                port: 50052,
            },
            network: NetworkConfig {
                ssid: "".to_string(),
                password: "".to_string(),
            },
            capability_configuration: CapabilityConfiguration::default(),
            network_resilience_settings: NetworkResilienceSettings::default(),
            resource_limits: ResourceLimits::default(),
            ota_update_settings: OtaUpdateSettings::default(),
        }
    }
}
