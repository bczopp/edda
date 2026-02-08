//! Skirnir - Service Registry

use crate::utils::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[derive(Debug, Clone)]
#[derive(Debug, Clone, Default)]
pub struct ResourceUsage {
    pub memory_bytes: u64,
    pub cpu_percent: f32,
}

#[derive(Debug, Clone, Default)]
pub struct HealthStatus {
    pub is_healthy: bool,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub status: ServiceStatus,
    pub process_id: Option<u32>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub resource_usage: ResourceUsage,
    pub health: HealthStatus,
}

pub struct Skirnir {
    services: Arc<RwLock<HashMap<String, ServiceInfo>>>,
}

impl Skirnir {
    pub fn new() -> Result<Self> {
        info!("Initializing Skirnir (Service Registry)");
        
        Ok(Self {
            services: Arc::new(RwLock::new(HashMap::new())),
        })
    }
    
    pub async fn register_service(&self, name: String) -> Result<()> {
        let mut services = self.services.write().await;
        
        let service_info = ServiceInfo {
            name: name.clone(),
            status: ServiceStatus::Starting,
            process_id: None,
            start_time: Some(chrono::Utc::now()),
            resource_usage: ResourceUsage::default(),
            health: HealthStatus {
                is_healthy: true,
                error_message: None,
            },
        };
        
        services.insert(name, service_info);
        info!("Service registered");
        
        Ok(())
    }
    
    pub async fn update_status(&self, name: &str, status: ServiceStatus) -> Result<()> {
        let mut services = self.services.write().await;
        
        if let Some(service) = services.get_mut(name) {
            service.status = status;
            info!("Service '{}' status updated to {:?}", name, service.status);
        }
        
        Ok(())
    }

    pub async fn update_pid(&self, name: &str, pid: u32) -> Result<()> {
        let mut services = self.services.write().await;
        if let Some(service) = services.get_mut(name) {
            service.process_id = Some(pid);
        }
        Ok(())
    }

    pub async fn update_resources(&self, name: &str, memory_bytes: u64, cpu_percent: f32) -> Result<()> {
        let mut services = self.services.write().await;
        if let Some(service) = services.get_mut(name) {
            service.resource_usage = ResourceUsage {
                memory_bytes,
                cpu_percent,
            };
        }
        Ok(())
    }

    pub async fn update_health(&self, name: &str, is_healthy: bool, error_message: Option<String>) -> Result<()> {
        let mut services = self.services.write().await;
        if let Some(service) = services.get_mut(name) {
            service.health = HealthStatus {
                is_healthy,
                error_message,
            };
        }
        Ok(())
    }
    
    pub async fn get_service(&self, name: &str) -> Option<ServiceInfo> {
        let services = self.services.read().await;
        services.get(name).cloned()
    }
    
    pub async fn list_services(&self) -> Vec<ServiceInfo> {
        let services = self.services.read().await;
        services.values().cloned().collect()
    }

    /// Remove a service from the registry (e.g. after stop).
    pub async fn unregister_service(&self, name: &str) -> Result<()> {
        let mut services = self.services.write().await;
        services.remove(name);
        info!("Service '{}' unregistered", name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_skirnir_creation() {
        let skirnir = Skirnir::new();
        assert!(skirnir.is_ok());
    }
    
    #[tokio::test]
    async fn test_register_service() {
        let skirnir = Skirnir::new().unwrap();
        skirnir.register_service("test-service".to_string()).await.unwrap();
        
        let service = skirnir.get_service("test-service").await;
        assert!(service.is_some());
        assert_eq!(service.unwrap().name, "test-service");
    }
    
    #[tokio::test]
    async fn test_update_status() {
        let skirnir = Skirnir::new().unwrap();
        skirnir.register_service("test".to_string()).await.unwrap();
        skirnir.update_status("test", ServiceStatus::Running).await.unwrap();
        
        let service = skirnir.get_service("test").await.unwrap();
        matches!(service.status, ServiceStatus::Running);
    }
}
