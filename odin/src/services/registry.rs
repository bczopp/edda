use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::protocols::einherjar::einherjar::CapabilityResponse;

#[derive(Clone)]
pub struct ServiceInfo {
    pub service_name: String,
    pub service_url: String,
    pub capabilities: Vec<String>,
}

pub struct ServiceRegistry {
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
}

impl ServiceRegistry {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, service: ServiceInfo) {
        let mut services = self.services.write().await;
        services.insert(service.service_name.clone(), service);
    }

    /// Phase 3 Einherjar-Protocol: register a service from an Einherjar CapabilityResponse.
    /// Uses god_name as service_name, responsibility_domains and responsibility_keywords as capabilities.
    pub async fn register_from_capability(&self, service_url: &str, response: CapabilityResponse) {
        let mut capabilities: Vec<String> = response.responsibility_domains.clone();
        capabilities.extend(response.responsibility_keywords.clone());
        let info = ServiceInfo {
            service_name: response.god_name.clone(),
            service_url: service_url.to_string(),
            capabilities,
        };
        self.register(info).await;
    }

    pub async fn get(&self, service_name: &str) -> Option<ServiceInfo> {
        let services = self.services.read().await;
        services.get(service_name).cloned()
    }

    pub async fn list(&self) -> Vec<ServiceInfo> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }
}

impl Default for ServiceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
