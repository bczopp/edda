use std::sync::Arc;

use thiserror::Error;

use crate::model::{ModelInfo, ModelRegistry};

/// Fehler für Model-Discovery-Operationen.
#[derive(Debug, Error, Clone)]
pub enum DiscoveryError {
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("invalid capability response: {0}")]
    InvalidResponse(String),
    #[error("network error: {0}")]
    NetworkError(String),
}

/// Trait für einen Einherjar-Capability-Client (abstrakte Schnittstelle).
///
/// Konkrete Implementierungen sprechen später z. B. gRPC.
pub trait EinherjarCapabilityClient: Send + Sync {
    /// Liefert die vom Remote-Service gemeldeten Modelle.
    fn discover_models(&self, service_url: &str) -> Result<Vec<ModelInfo>, DiscoveryError>;
}

/// Hilfsklasse für Model-Discovery und Registry-Update.
pub struct ModelDiscovery {
    registry: ModelRegistry,
    client: Arc<dyn EinherjarCapabilityClient>,
}

impl ModelDiscovery {
    /// Erstellt eine neue ModelDiscovery-Instanz.
    pub fn new(registry: ModelRegistry, client: Arc<dyn EinherjarCapabilityClient>) -> Self {
        Self { registry, client }
    }

    /// Führt eine Discovery gegen den angegebenen Service aus und registriert alle
    /// gefundenen Modelle im Registry. Gibt eine aktualisierte Registry zurück.
    pub fn discover_and_register(
        &self,
        service_url: &str,
    ) -> Result<ModelRegistry, DiscoveryError> {
        let discovered = self.client.discover_models(service_url)?;

        let mut registry = self.registry.clone();
        for model in discovered {
            registry = registry.register(model);
        }

        Ok(registry)
    }

    /// Liefert eine Referenz auf die aktuelle Registry.
    pub fn registry(&self) -> &ModelRegistry {
        &self.registry
    }
}

