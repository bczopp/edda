use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct ServiceInfo {
    pub service_name: String,
    pub status: String,
    pub health: String,
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
        self.services.write().await.insert(service.service_name.clone(), service);
    }

    pub async fn get(&self, service_name: &str) -> Option<ServiceInfo> {
        self.services.read().await.get(service_name).cloned()
    }

    pub async fn list(&self) -> Vec<ServiceInfo> {
        self.services.read().await.values().cloned().collect()
    }
}
