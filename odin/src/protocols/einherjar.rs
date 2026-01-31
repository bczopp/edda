use tonic::transport::Channel;
use anyhow::Result;
use std::time::Duration;
use crate::clients::ServiceClientConfig;

pub mod einherjar {
    tonic::include_proto!("einherjar");
}

use einherjar::einherjar_protocol_client::EinherjarProtocolClient;
use einherjar::{CapabilityRequest, CapabilityResponse};

/// Client for Einherjar Protocol
/// Used to discover capabilities of services and plugins
pub struct EinherjarClient {
    client: EinherjarProtocolClient<Channel>,
}

impl EinherjarClient {
    /// Create a new Einherjar client
    pub async fn new(config: ServiceClientConfig) -> Result<Self> {
        let endpoint = tonic::transport::Endpoint::from_shared(config.url)?
            .timeout(Duration::from_secs(config.timeout_seconds))
            .connect_timeout(Duration::from_secs(5));
        
        let channel = endpoint.connect().await?;
        let client = EinherjarProtocolClient::new(channel);
        
        Ok(Self { client })
    }

    /// Get capabilities from a service/plugin
    pub async fn get_capabilities(&mut self) -> Result<CapabilityResponse> {
        let request = tonic::Request::new(CapabilityRequest {});
        let response = self.client.get_capabilities(request).await?;
        Ok(response.into_inner())
    }
}

/// Capability cache entry
#[derive(Debug, Clone)]
pub struct CachedCapability {
    pub service_name: String,
    pub service_url: String,
    pub capability: CapabilityResponse,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

/// Phase 3 Capability-Aggregation: aggregated view of all cached capabilities (by domain/keyword).
#[derive(Debug, Clone, Default)]
pub struct AggregatedCapabilities {
    /// For each responsibility_domain, service names that declare it.
    pub by_domain: std::collections::HashMap<String, Vec<String>>,
    /// For each responsibility_keyword, service names that declare it.
    pub by_keyword: std::collections::HashMap<String, Vec<String>>,
}

impl AggregatedCapabilities {
    /// Service names that handle the given domain.
    pub fn services_for_domain(&self, domain: &str) -> Option<&Vec<String>> {
        self.by_domain.get(domain)
    }

    /// Service names that declare the given keyword.
    pub fn services_for_keyword(&self, keyword: &str) -> Option<&Vec<String>> {
        self.by_keyword.get(keyword)
    }
}

/// Capability cache manager
pub struct CapabilityCache {
    cache: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, CachedCapability>>>,
}

impl CapabilityCache {
    pub fn new() -> Self {
        Self {
            cache: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// Get cached capability for a service
    pub async fn get(&self, service_name: &str) -> Option<CachedCapability> {
        let cache = self.cache.read().await;
        cache.get(service_name).cloned()
    }

    /// Update capability cache for a service
    pub async fn update(&self, service_name: String, service_url: String, capability: CapabilityResponse) {
        let mut cache = self.cache.write().await;
        cache.insert(service_name.clone(), CachedCapability {
            service_name,
            service_url,
            capability,
            last_updated: chrono::Utc::now(),
        });
    }

    /// Get all cached capabilities
    pub async fn get_all(&self) -> Vec<CachedCapability> {
        let cache = self.cache.read().await;
        cache.values().cloned().collect()
    }

    /// Phase 3 Capability-Aggregation: build aggregated view (by_domain, by_keyword) from all cached capabilities.
    pub async fn get_aggregated(&self) -> AggregatedCapabilities {
        let all = self.get_all().await;
        let mut by_domain: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        let mut by_keyword: std::collections::HashMap<String, Vec<String>> =
            std::collections::HashMap::new();
        for c in all {
            for d in &c.capability.responsibility_domains {
                by_domain
                    .entry(d.clone())
                    .or_default()
                    .push(c.service_name.clone());
            }
            for k in &c.capability.responsibility_keywords {
                by_keyword
                    .entry(k.clone())
                    .or_default()
                    .push(c.service_name.clone());
            }
        }
        AggregatedCapabilities {
            by_domain,
            by_keyword,
        }
    }

    /// Clear cache for a service
    pub async fn clear(&self, service_name: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(service_name);
    }

    /// Clear all cache
    pub async fn clear_all(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}

impl Default for CapabilityCache {
    fn default() -> Self {
        Self::new()
    }
}
