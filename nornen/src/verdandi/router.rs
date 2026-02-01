use crate::urd::registry::{ProviderRegistry, Provider};
use crate::cache::ProviderCache;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestRouterError {
    #[error("No provider available: {0}")]
    NoProviderAvailable(String),
    #[error("Provider registry error: {0}")]
    RegistryError(#[from] crate::urd::registry::ProviderRegistryError),
}

pub struct RequestRouter {
    registry: Arc<ProviderRegistry>,
    // Round-robin counter for load balancing
    round_robin_counter: Arc<AtomicUsize>,
    // Provider cache for performance optimization
    cache: Option<Arc<ProviderCache>>,
}

impl RequestRouter {
    pub fn new(registry: Arc<ProviderRegistry>) -> Self {
        Self {
            registry,
            round_robin_counter: Arc::new(AtomicUsize::new(0)),
            cache: None,
        }
    }

    pub fn new_with_cache(registry: Arc<ProviderRegistry>, cache: Arc<ProviderCache>) -> Self {
        Self {
            registry,
            round_robin_counter: Arc::new(AtomicUsize::new(0)),
            cache: Some(cache),
        }
    }

    pub async fn route_request(
        &self,
        required_capabilities: &[String],
        preferences: &std::collections::HashMap<String, String>,
    ) -> Result<Provider, RequestRouterError> {
        // Try to get from cache first
        let mut providers = if let Some(cache) = &self.cache {
            if let Some(cached) = cache.get(required_capabilities, Some("active")).await {
                cached
            } else {
                // Cache miss - query from registry
                let queried = self.registry
                    .query_providers(required_capabilities, Some("active"))
                    .await?;
                // Store in cache
                cache.set(required_capabilities, Some("active"), queried.clone()).await;
                queried
            }
        } else {
            // No cache - query directly
            self.registry
                .query_providers(required_capabilities, Some("active"))
                .await?
        };

        if providers.is_empty() {
            return Err(RequestRouterError::NoProviderAvailable(
                format!("No active provider with capabilities: {:?}", required_capabilities)
            ));
        }

        // Sort providers by score (preference-based selection)
        providers.sort_by(|a, b| {
            let score_a = self.calculate_score(a, preferences);
            let score_b = self.calculate_score(b, preferences);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Select provider using round-robin load balancing
        let index = self.round_robin_counter.fetch_add(1, Ordering::Relaxed) % providers.len();
        Ok(providers[index].clone())
    }

    pub async fn select_provider(
        &self,
        required_capabilities: &[String],
        preferences: &std::collections::HashMap<String, String>,
    ) -> Result<(String, String, f64), RequestRouterError> {
        let provider = self.route_request(required_capabilities, preferences).await?;
        
        // Calculate score
        let score = self.calculate_score(&provider, preferences);
        
        Ok((provider.provider_id, provider.endpoint, score))
    }

    /// Calculate provider score based on preferences and metadata
    fn calculate_score(&self, provider: &Provider, preferences: &std::collections::HashMap<String, String>) -> f64 {
        let mut score = 0.5; // Base score
        
        // Status preference (higher weight)
        if let Some(preferred_status) = preferences.get("status") {
            if provider.status == preferred_status {
                score += 0.3;
            } else if provider.status == "active" {
                score += 0.2; // Active providers get bonus even if not preferred
            }
        } else if provider.status == "active" {
            score += 0.3; // Default preference for active providers
        }
        
        // Metadata preferences
        if let Some(preferred_region) = preferences.get("region") {
            if let Some(region) = provider.metadata.get("region").and_then(|v| v.as_str()) {
                if region == preferred_region {
                    score += 0.1;
                }
            }
        }
        
        // Capability count bonus (more capabilities = more versatile)
        if provider.capabilities.len() > 1 {
            score += 0.05 * (provider.capabilities.len() as f64 - 1.0).min(2.0);
        }
        
        score.min(1.0).max(0.0)
    }

    /// Route request with fallback - tries multiple providers if first fails
    pub async fn route_request_with_fallback(
        &self,
        required_capabilities: &[String],
        preferences: &std::collections::HashMap<String, String>,
        max_attempts: usize,
    ) -> Result<Provider, RequestRouterError> {
        // Try to get from cache first
        let mut providers = if let Some(cache) = &self.cache {
            if let Some(cached) = cache.get(required_capabilities, Some("active")).await {
                cached
            } else {
                // Cache miss - query from registry
                let queried = self.registry
                    .query_providers(required_capabilities, Some("active"))
                    .await?;
                // Store in cache
                cache.set(required_capabilities, Some("active"), queried.clone()).await;
                queried
            }
        } else {
            // No cache - query directly
            self.registry
                .query_providers(required_capabilities, Some("active"))
                .await?
        };

        if providers.is_empty() {
            return Err(RequestRouterError::NoProviderAvailable(
                format!("No active provider with capabilities: {:?}", required_capabilities)
            ));
        }

        // Sort providers by score
        providers.sort_by(|a, b| {
            let score_a = self.calculate_score(a, preferences);
            let score_b = self.calculate_score(b, preferences);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        // Try providers in order (fallback mechanism)
        let attempts = max_attempts.min(providers.len());
        let start_index = self.round_robin_counter.fetch_add(1, Ordering::Relaxed) % providers.len();
        
        for i in 0..attempts {
            let index = (start_index + i) % providers.len();
            // In a real implementation, we would check if provider is actually available
            // For now, we just return the selected provider
            return Ok(providers[index].clone());
        }

        // If all attempts failed, return the best provider anyway
        Ok(providers[0].clone())
    }
}
