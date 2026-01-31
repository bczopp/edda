use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;
use crate::utils::config::OdinSettings;
use crate::clients::ServiceClientConfig;
use super::einherjar::{EinherjarClient, CapabilityCache};
use super::responsibility::{ResponsibilityClient, responsibility};

/// Manages protocol clients (Einherjar, Responsibility)
pub struct ProtocolManager {
    settings: Arc<tokio::sync::RwLock<OdinSettings>>,
    capability_cache: Arc<CapabilityCache>,
    einherjar_clients: Arc<RwLock<std::collections::HashMap<String, EinherjarClient>>>,
    responsibility_clients: Arc<RwLock<std::collections::HashMap<String, ResponsibilityClient>>>,
}

impl ProtocolManager {
    pub fn new(settings: Arc<tokio::sync::RwLock<OdinSettings>>) -> Self {
        Self {
            settings,
            capability_cache: Arc::new(CapabilityCache::new()),
            einherjar_clients: Arc::new(RwLock::new(std::collections::HashMap::new())),
            responsibility_clients: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Get capability cache
    pub fn get_cache(&self) -> Arc<CapabilityCache> {
        self.capability_cache.clone()
    }

    /// Discover capabilities from all registered services and enabled plugins (Frigg, Valkyries).
    pub async fn discover_all_capabilities(&self) -> Result<()> {
        let settings = self.settings.read().await;
        
        let mut service_urls: Vec<(&str, Option<&String>)> = vec![
            ("thor", settings.service_urls.thor.as_ref()),
            ("freki", settings.service_urls.freki.as_ref()),
            ("geri", settings.service_urls.geri.as_ref()),
            ("loki", settings.service_urls.loki.as_ref()),
            ("heimdall", settings.service_urls.heimdall.as_ref()),
            ("skuld", settings.service_urls.skuld.as_ref()),
        ];
        if settings.plugins.frigg.enabled {
            service_urls.push(("frigg", settings.service_urls.frigg.as_ref()));
        }
        if settings.plugins.valkyries.enabled {
            service_urls.push(("valkyries", settings.service_urls.valkyries.as_ref()));
        }

        for (service_name, url_opt) in service_urls {
            if let Some(url) = url_opt {
                if let Err(e) = self.discover_service_capabilities(service_name, url).await {
                    tracing::warn!("Failed to discover capabilities for {}: {}", service_name, e);
                }
            }
        }

        Ok(())
    }

    /// Discover capabilities from a specific service
    pub async fn discover_service_capabilities(&self, service_name: &str, service_url: &str) -> Result<()> {
        let config = ServiceClientConfig {
            url: service_url.to_string(),
            timeout_seconds: 30,
        };

        // Create or get Einherjar client and get capabilities
        let capabilities = {
            let mut clients = self.einherjar_clients.write().await;
            let client = if let Some(client) = clients.get_mut(service_name) {
                // Reuse existing client
                client
            } else {
                // Create new client
                let client = EinherjarClient::new(config.clone()).await?;
                clients.insert(service_name.to_string(), client);
                clients.get_mut(service_name).unwrap()
            };
            
            // Get capabilities while holding the lock
            client.get_capabilities().await?
        };
        
        // Cache capabilities
        self.capability_cache.update(
            service_name.to_string(),
            service_url.to_string(),
            capabilities,
        ).await;

        tracing::info!("Discovered capabilities for service: {}", service_name);
        Ok(())
    }

    /// Take responsibility for a request (convenience method).
    /// Resolves service URL from settings first, then from capability cache (plugins).
    pub async fn take_responsibility(
        &self,
        service_name: &str,
        request: responsibility::TakeResponsibilityRequest,
    ) -> Result<responsibility::TakeResponsibilityResponse, String> {
        let service_url = self.resolve_service_url(service_name).await;

        if let Some(url) = service_url {
            let config = ServiceClientConfig {
                url: url.clone(),
                timeout_seconds: 30,
            };
            let mut clients = self.responsibility_clients.write().await;
            if !clients.contains_key(service_name) {
                match ResponsibilityClient::new(config).await {
                    Ok(client) => {
                        clients.insert(service_name.to_string(), client);
                    }
                    Err(e) => {
                        return Err(format!("Failed to create responsibility client: {}", e));
                    }
                }
            }
            if let Some(client) = clients.get_mut(service_name) {
                client.take_responsibility(request).await
                    .map_err(|e| format!("Failed to take responsibility: {}", e))
            } else {
                Err("Client not found".to_string())
            }
        } else {
            Err(format!("Service {} not configured or not in capability cache", service_name))
        }
    }

    /// Resolve service/plugin URL: settings first, then capability cache (e.g. plugins).
    async fn resolve_service_url(&self, service_name: &str) -> Option<String> {
        let settings = self.settings.read().await;
        let from_settings = match service_name {
            "thor" => settings.service_urls.thor.as_ref(),
            "freki" => settings.service_urls.freki.as_ref(),
            "geri" => settings.service_urls.geri.as_ref(),
            "loki" => settings.service_urls.loki.as_ref(),
            "heimdall" => settings.service_urls.heimdall.as_ref(),
            "skuld" => settings.service_urls.skuld.as_ref(),
            "frigg" => settings.service_urls.frigg.as_ref(),
            "valkyries" => settings.service_urls.valkyries.as_ref(),
            _ => None,
        };
        if let Some(u) = from_settings {
            return Some(u.clone());
        }
        drop(settings);
        self.capability_cache.get(service_name).await.map(|c| c.service_url)
    }

    /// Return responsibility (convenience method)
    pub async fn return_responsibility(
        &self,
        service_name: &str,
        request: responsibility::ReturnResponsibilityRequest,
    ) -> Result<responsibility::ReturnResponsibilityResponse, String> {
        let service_url = self.resolve_service_url(service_name).await;

        if let Some(url) = service_url {
            let config = ServiceClientConfig {
                url: url.clone(),
                timeout_seconds: 30,
            };
            
            let mut clients = self.responsibility_clients.write().await;
            if !clients.contains_key(service_name) {
                match ResponsibilityClient::new(config).await {
                    Ok(client) => {
                        clients.insert(service_name.to_string(), client);
                    }
                    Err(e) => {
                        return Err(format!("Failed to create responsibility client: {}", e));
                    }
                }
            }
            
            if let Some(client) = clients.get_mut(service_name) {
                client.return_responsibility(request).await
                    .map_err(|e| format!("Failed to return responsibility: {}", e))
            } else {
                Err("Client not found".to_string())
            }
        } else {
            Err(format!("Service {} not configured or not in capability cache", service_name))
        }
    }

    /// Reject responsibility (convenience method)
    pub async fn reject_responsibility(
        &self,
        service_name: &str,
        request: responsibility::RejectResponsibilityRequest,
    ) -> Result<responsibility::RejectResponsibilityResponse, String> {
        let service_url = self.resolve_service_url(service_name).await;

        if let Some(url) = service_url {
            let config = ServiceClientConfig {
                url: url.clone(),
                timeout_seconds: 30,
            };
            
            let mut clients = self.responsibility_clients.write().await;
            if !clients.contains_key(service_name) {
                match ResponsibilityClient::new(config).await {
                    Ok(client) => {
                        clients.insert(service_name.to_string(), client);
                    }
                    Err(e) => {
                        return Err(format!("Failed to create responsibility client: {}", e));
                    }
                }
            }
            
            if let Some(client) = clients.get_mut(service_name) {
                client.reject_responsibility(request).await
                    .map_err(|e| format!("Failed to reject responsibility: {}", e))
            } else {
                Err("Client not found".to_string())
            }
        } else {
            Err(format!("Service {} not configured or not in capability cache", service_name))
        }
    }
}

