use crate::urd::registry::ProviderRegistry;
use crate::verdandi::router::RequestRouter;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoordinationError {
    #[error("Routing error: {0}")]
    RoutingError(#[from] crate::verdandi::router::RequestRouterError),
    #[error("Registry error: {0}")]
    RegistryError(#[from] crate::urd::registry::ProviderRegistryError),
}

pub struct NornenCoordinator {
    registry: Arc<ProviderRegistry>,
    router: Arc<RequestRouter>,
}

impl NornenCoordinator {
    pub fn new(registry: Arc<ProviderRegistry>, router: Arc<RequestRouter>) -> Self {
        Self { registry, router }
    }

    pub async fn coordinate_request(
        &self,
        request_id: &str,
        request_type: &str,
        context: &std::collections::HashMap<String, String>,
    ) -> Result<CoordinationResult, CoordinationError> {
        // Extract required capabilities from context
        let required_capabilities: Vec<String> = context
            .get("required_capabilities")
            .map(|s| s.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        // Extract preferences from context
        let preferences: std::collections::HashMap<String, String> = context
            .iter()
            .filter(|(k, _)| k.starts_with("pref_"))
            .map(|(k, v)| (k.strip_prefix("pref_").unwrap_or(k).to_string(), v.clone()))
            .collect();

        // Route request
        let (provider_id, endpoint, score) = self.router
            .select_provider(&required_capabilities, &preferences)
            .await?;

        Ok(CoordinationResult {
            decision: "route".to_string(),
            provider_id,
            confidence: score,
            reasoning: format!("Selected provider {} for request type {}", provider_id, request_type),
        })
    }
}

pub struct CoordinationResult {
    pub decision: String,
    pub provider_id: String,
    pub confidence: f64,
    pub reasoning: String,
}
