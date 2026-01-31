use crate::urd::registry::{ProviderRegistry, Provider};
use std::sync::Arc;
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
}

impl RequestRouter {
    pub fn new(registry: Arc<ProviderRegistry>) -> Self {
        Self { registry }
    }

    pub async fn route_request(
        &self,
        required_capabilities: &[String],
        preferences: &std::collections::HashMap<String, String>,
    ) -> Result<Provider, RequestRouterError> {
        // Query providers with required capabilities
        let providers = self.registry
            .query_providers(required_capabilities, Some("active"))
            .await?;

        if providers.is_empty() {
            return Err(RequestRouterError::NoProviderAvailable(
                format!("No active provider with capabilities: {:?}", required_capabilities)
            ));
        }

        // Simple selection: first provider that matches
        // TODO: Implement load balancing and preference-based selection
        Ok(providers[0].clone())
    }

    pub async fn select_provider(
        &self,
        required_capabilities: &[String],
        preferences: &std::collections::HashMap<String, String>,
    ) -> Result<(String, String, f64), RequestRouterError> {
        let provider = self.route_request(required_capabilities, preferences).await?;
        
        // Calculate score (simple implementation)
        let score = self.calculate_score(&provider, preferences);
        
        Ok((provider.provider_id, provider.endpoint, score))
    }

    fn calculate_score(&self, provider: &Provider, preferences: &std::collections::HashMap<String, String>) -> f64 {
        // Simple scoring: base score of 0.8, can be adjusted based on preferences
        let mut score = 0.8;
        
        // Adjust score based on preferences
        if let Some(preferred_status) = preferences.get("status") {
            if provider.status == preferred_status {
                score += 0.1;
            }
        }
        
        score.min(1.0)
    }
}
