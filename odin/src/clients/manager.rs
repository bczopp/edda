use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use crate::utils::config::OdinSettings;
use super::{
    ServiceClientConfig,
    thor::ThorClient,
    freki::FrekiClient,
    geri::GeriClient,
    skuld::SkuldClient,
    huginn_muninn::HuginnMuninnClient,
    loki::LokiClient,
    heimdall::HeimdallClient,
};

/// Manages all service clients
pub struct ClientManager {
    settings: Arc<tokio::sync::RwLock<OdinSettings>>,
    thor_client: Arc<RwLock<Option<ThorClient>>>,
    freki_client: Arc<RwLock<Option<FrekiClient>>>,
    geri_client: Arc<RwLock<Option<GeriClient>>>,
    skuld_client: Arc<RwLock<Option<SkuldClient>>>,
    huginn_muninn_client: Arc<RwLock<Option<HuginnMuninnClient>>>,
    loki_client: Arc<RwLock<Option<LokiClient>>>,
    heimdall_client: Arc<RwLock<Option<HeimdallClient>>>,
}

impl ClientManager {
    pub fn new(settings: Arc<tokio::sync::RwLock<OdinSettings>>) -> Self {
        Self {
            settings,
            thor_client: Arc::new(RwLock::new(None)),
            freki_client: Arc::new(RwLock::new(None)),
            geri_client: Arc::new(RwLock::new(None)),
            skuld_client: Arc::new(RwLock::new(None)),
            huginn_muninn_client: Arc::new(RwLock::new(None)),
            loki_client: Arc::new(RwLock::new(None)),
            heimdall_client: Arc::new(RwLock::new(None)),
        }
    }

    /// Initialize all clients from settings
    pub async fn initialize(&self) -> Result<()> {
        let settings = self.settings.read().await;
        
        // Initialize Thor client
        if let Some(url) = &settings.service_urls.thor {
            let config = ServiceClientConfig {
                url: url.clone(),
                timeout_seconds: 30,
            };
            match ThorClient::new(config).await {
                Ok(client) => {
                    *self.thor_client.write().await = Some(client);
                    tracing::info!("Thor client initialized");
                }
                Err(e) => {
                    tracing::warn!("Failed to initialize Thor client: {}", e);
                }
            }
        }

        // Initialize Freki client
        if let Some(url) = &settings.service_urls.freki {
            let config = ServiceClientConfig {
                url: url.clone(),
                timeout_seconds: 30,
            };
            match FrekiClient::new(config).await {
                Ok(client) => {
                    *self.freki_client.write().await = Some(client);
                    tracing::info!("Freki client initialized");
                }
                Err(e) => {
                    tracing::warn!("Failed to initialize Freki client: {}", e);
                }
            }
        }

        // Initialize Geri client
        if let Some(url) = &settings.service_urls.geri {
            let config = ServiceClientConfig {
                url: url.clone(),
                timeout_seconds: 30,
            };
            match GeriClient::new(config).await {
                Ok(client) => {
                    *self.geri_client.write().await = Some(client);
                    tracing::info!("Geri client initialized");
                }
                Err(e) => {
                    tracing::warn!("Failed to initialize Geri client: {}", e);
                }
            }
        }

        // Initialize Skuld client
        if let Some(url) = &settings.service_urls.skuld {
            let config = ServiceClientConfig {
                url: url.clone(),
                timeout_seconds: 30,
            };
            match SkuldClient::new(config).await {
                Ok(client) => {
                    *self.skuld_client.write().await = Some(client);
                    tracing::info!("Skuld client initialized");
                }
                Err(e) => {
                    tracing::warn!("Failed to initialize Skuld client: {}", e);
                }
            }
        }

        // Initialize Huginn-Muninn client
        if let Some(url) = &settings.service_urls.huginn_muninn {
            let config = ServiceClientConfig {
                url: url.clone(),
                timeout_seconds: 30,
            };
            match HuginnMuninnClient::new(config).await {
                Ok(client) => {
                    *self.huginn_muninn_client.write().await = Some(client);
                    tracing::info!("Huginn-Muninn client initialized");
                }
                Err(e) => {
                    tracing::warn!("Failed to initialize Huginn-Muninn client: {}", e);
                }
            }
        }

        // Initialize Loki client
        if let Some(url) = &settings.service_urls.loki {
            let config = ServiceClientConfig {
                url: url.clone(),
                timeout_seconds: 30,
            };
            match LokiClient::new(config).await {
                Ok(client) => {
                    *self.loki_client.write().await = Some(client);
                    tracing::info!("Loki client initialized");
                }
                Err(e) => {
                    tracing::warn!("Failed to initialize Loki client: {}", e);
                }
            }
        }

        // Initialize Heimdall client
        if let Some(url) = &settings.service_urls.heimdall {
            let config = ServiceClientConfig {
                url: url.clone(),
                timeout_seconds: 30,
            };
            match HeimdallClient::new(config).await {
                Ok(client) => {
                    *self.heimdall_client.write().await = Some(client);
                    tracing::info!("Heimdall client initialized");
                }
                Err(e) => {
                    tracing::warn!("Failed to initialize Heimdall client: {}", e);
                }
            }
        }

        Ok(())
    }

    /// Execute action via Thor (convenience method)
    pub async fn execute_thor_action(&self, action: crate::clients::thor::thor::ThorAction) -> Result<crate::clients::thor::thor::ThorResult, String> {
        let mut client_guard = self.thor_client.write().await;
        if let Some(ref mut client) = *client_guard {
            client.execute_action(action).await
                .map_err(|e| format!("Failed to execute action: {}", e))
        } else {
            Err("Thor client not initialized".to_string())
        }
    }

    /// Retrieve context from Freki (convenience method)
    pub async fn retrieve_freki_context(&self, request: crate::clients::freki::freki::RetrieveContextRequest) -> Result<crate::clients::freki::freki::RetrieveContextResponse, String> {
        let mut client_guard = self.freki_client.write().await;
        if let Some(ref mut client) = *client_guard {
            client.retrieve_context(request).await
                .map_err(|e| format!("Failed to retrieve context: {}", e))
        } else {
            Err("Freki client not initialized".to_string())
        }
    }

    /// Process prompt via Geri (convenience method)
    pub async fn process_geri_prompt(&self, request: crate::clients::geri::geri::ProcessPromptRequest) -> Result<crate::clients::geri::geri::ProcessPromptResponse, String> {
        let mut client_guard = self.geri_client.write().await;
        if let Some(ref mut client) = *client_guard {
            client.process_prompt(request).await
                .map_err(|e| format!("Failed to process prompt: {}", e))
        } else {
            Err("Geri client not initialized".to_string())
        }
    }

    /// Select model via Skuld (convenience method)
    pub async fn select_skuld_model(&self, request: crate::clients::skuld::skuld::SelectModelRequest) -> Result<crate::clients::skuld::skuld::SelectModelResponse, String> {
        let mut client_guard = self.skuld_client.write().await;
        if let Some(ref mut client) = *client_guard {
            client.select_model(request).await
                .map_err(|e| format!("Failed to select model: {}", e))
        } else {
            Err("Skuld client not initialized".to_string())
        }
    }
}
