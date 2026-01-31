pub mod thor;
pub mod freki;
pub mod geri;
pub mod skuld;
pub mod huginn_muninn;
pub mod loki;
pub mod heimdall;
pub mod manager;

use anyhow::Result;

/// Service client configuration
#[derive(Debug, Clone)]
pub struct ServiceClientConfig {
    pub url: String,
    pub timeout_seconds: u64,
}

impl Default for ServiceClientConfig {
    fn default() -> Self {
        Self {
            url: String::new(),
            timeout_seconds: 30,
        }
    }
}
