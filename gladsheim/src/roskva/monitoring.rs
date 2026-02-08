//! Health Monitoring for Roskva

use crate::utils::{GladsheimError, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{info, warn};
use chrono::Utc;

#[derive(Debug, Clone)]
pub enum HealthCheckStrategy {
    Http { url: String },
    Grpc { service: String },
    Process,
}

#[derive(Debug, Clone)]
pub struct ServiceHealth {
    pub service_name: String,
    pub is_healthy: bool,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub consecutive_failures: u32,
    pub error_message: Option<String>,
    pub check_strategy: HealthCheckStrategy,
}

#[derive(Clone)]
pub struct HealthMonitor {
    check_interval: Duration,
    http_client: reqwest::Client,
}

impl HealthMonitor {
    pub fn new(check_interval: Duration) -> Self {
        Self {
            check_interval,
            http_client: reqwest::Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap_or_else(|_| reqwest::Client::new()),
        }
    }
    
    pub fn default() -> Self {
        Self::new(Duration::from_secs(5))
    }
    
    pub fn check_interval(&self) -> Duration {
        self.check_interval
    }
    
    pub async fn check_http_health(&self, url: &str) -> Result<bool> {
        info!("Checking HTTP health: {}", url);
        
        match self.http_client
            .get(url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => {
                let is_healthy = response.status().is_success();
                info!("HTTP health check for {}: {}", url, is_healthy);
                Ok(is_healthy)
            }
            Err(e) => {
                warn!("HTTP health check failed for {}: {}", url, e);
                Err(GladsheimError::HealthCheckError(format!("HTTP check failed: {}", e)))
            }
        }
    }
    
    pub async fn check_grpc_health(&self, service: &str) -> Result<bool> {
        info!("Checking gRPC health for: {}", service);
        
        // TODO: Use gRPC health check protocol (grpc.health.v1.Health)
        // For now, return mock result
        warn!("gRPC health check not yet implemented for {}", service);
        Ok(true)
    }
}

#[derive(Clone)]
pub struct ServiceHealthTracker {
    services: Arc<RwLock<HashMap<String, ServiceHealth>>>,
    #[allow(dead_code)]
    check_interval: Duration,
    max_failures: Arc<RwLock<HashMap<String, u32>>>,
}

impl ServiceHealthTracker {
    pub fn new(check_interval: Duration) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            check_interval,
            max_failures: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn register_service(
        &self,
        service_name: String,
        strategy: HealthCheckStrategy,
    ) {
        let health = ServiceHealth {
            service_name: service_name.clone(),
            is_healthy: true,
            last_check: Utc::now(),
            consecutive_failures: 0,
            error_message: None,
            check_strategy: strategy,
        };
        
        let mut services = self.services.write().await;
        services.insert(service_name, health);
        info!("Registered health check for service");
    }
    
    pub async fn unregister_service(&self, service_name: &str) {
        let mut services = self.services.write().await;
        services.remove(service_name);
        
        let mut max_failures = self.max_failures.write().await;
        max_failures.remove(service_name);
        
        info!("Unregistered health check for service '{}'", service_name);
    }
    
    pub async fn update_health(
        &self,
        service_name: &str,
        is_healthy: bool,
        error_message: Option<String>,
    ) {
        let mut services = self.services.write().await;
        
        if let Some(health) = services.get_mut(service_name) {
            health.is_healthy = is_healthy;
            health.last_check = Utc::now();
            
            if is_healthy {
                health.consecutive_failures = 0;
                health.error_message = None;
            } else {
                health.consecutive_failures += 1;
                health.error_message = error_message;
            }
        }
    }
    
    pub async fn get_health(&self, service_name: &str) -> Option<ServiceHealth> {
        let services = self.services.read().await;
        services.get(service_name).cloned()
    }
    
    pub async fn get_strategy(&self, service_name: &str) -> Option<HealthCheckStrategy> {
        let services = self.services.read().await;
        services.get(service_name).map(|h| h.check_strategy.clone())
    }
    
    pub async fn set_max_failures(&self, service_name: &str, max: u32) {
        let mut max_failures = self.max_failures.write().await;
        max_failures.insert(service_name.to_string(), max);
    }
    
    pub async fn should_restart(&self, service_name: &str) -> Option<bool> {
        let health = self.get_health(service_name).await?;
        let max_failures = {
            let max = self.max_failures.read().await;
            max.get(service_name).copied().unwrap_or(3) // Default: 3 failures
        };
        
        Some(health.consecutive_failures >= max_failures)
    }
    
    pub async fn list_services(&self) -> Vec<String> {
        let services = self.services.read().await;
        services.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_health_monitor_creation() {
        let monitor = HealthMonitor::default();
        assert_eq!(monitor.check_interval(), Duration::from_secs(5));
    }
    
    #[tokio::test]
    async fn test_http_health_check_unreachable() {
        let monitor = HealthMonitor::default();
        // Test with unreachable URL
        let result = monitor.check_http_health("http://127.0.0.1:59999/health").await;
        // Should return error for unreachable
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_service_health_tracker() {
        let tracker = ServiceHealthTracker::new(Duration::from_secs(5));
        
        tracker.register_service("test".to_string(), HealthCheckStrategy::Http {
            url: "http://localhost:8080/health".to_string(),
        }).await;
        
        let health = tracker.get_health("test").await;
        assert!(health.is_some());
        assert_eq!(health.unwrap().is_healthy, true);
    }
}
