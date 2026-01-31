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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JotunheimSettings {
    pub loki: LokiConfig,
    pub network: NetworkConfig,
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
        }
    }
}
