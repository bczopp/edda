//! Service Resource Tracker for Byggvir

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub service_name: String,
    pub memory_bytes: u64,
    pub cpu_percent: f32,
    pub measured_at: chrono::DateTime<chrono::Utc>,
}

pub struct ServiceResourceTracker {
    services: Arc<RwLock<HashMap<String, ResourceUsage>>>,
    history: Arc<RwLock<HashMap<String, Vec<ResourceUsage>>>>,
    process_ids: Arc<RwLock<HashMap<String, u32>>>,
    history_capacity: usize,
}

impl ServiceResourceTracker {
    pub fn new() -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            history: Arc::new(RwLock::new(HashMap::new())),
            process_ids: Arc::new(RwLock::new(HashMap::new())),
            history_capacity: 10,
        }
    }

    /// Create a tracker with a custom maximum history length per service.
    pub fn with_history_capacity(history_capacity: usize) -> Self {
        Self {
            services: Arc::new(RwLock::new(HashMap::new())),
            history: Arc::new(RwLock::new(HashMap::new())),
            process_ids: Arc::new(RwLock::new(HashMap::new())),
            history_capacity: history_capacity.max(1),
        }
    }
    
    pub async fn register_service(&self, service_name: String, process_id: u32) {
        info!("Registering resource tracking for service: {} (PID: {})", service_name, process_id);
        
        let usage = ResourceUsage {
            service_name: service_name.clone(),
            memory_bytes: 0,
            cpu_percent: 0.0,
            measured_at: Utc::now(),
        };
        
        {
            let mut services = self.services.write().await;
            services.insert(service_name.clone(), usage);
        }
        
        {
            let mut pids = self.process_ids.write().await;
            pids.insert(service_name, process_id);
        }
    }
    
    pub async fn update_usage(
        &self,
        service_name: &str,
        memory_bytes: u64,
        cpu_percent: f32,
    ) {
        let mut services = self.services.write().await;
        let mut history = self.history.write().await;

        if let Some(usage) = services.get_mut(service_name) {
            usage.memory_bytes = memory_bytes;
            usage.cpu_percent = cpu_percent;
            usage.measured_at = Utc::now();

            let entry = usage.clone();
            let vec = history.entry(service_name.to_string()).or_default();
            vec.push(entry);
            if vec.len() > self.history_capacity {
                let overflow = vec.len() - self.history_capacity;
                vec.drain(0..overflow);
            }
        }
    }
    
    pub async fn get_usage(&self, service_name: &str) -> Option<ResourceUsage> {
        let services = self.services.read().await;
        services.get(service_name).cloned()
    }

    /// Return the recorded history for a service, optionally limited to the last `limit` entries.
    pub async fn get_history(
        &self,
        service_name: &str,
        limit: Option<usize>,
    ) -> Vec<ResourceUsage> {
        let history = self.history.read().await;
        let entries = match history.get(service_name) {
            Some(entries) => entries,
            None => return Vec::new(),
        };

        let len = entries.len();
        let limit = limit.unwrap_or(len).min(len);
        entries[len - limit..].to_vec()
    }

    async fn aggregate_internal<F>(&self, service_name: &str, f: F) -> Option<ResourceUsage>
    where
        F: Fn(&[ResourceUsage]) -> ResourceUsage,
    {
        let history = self.history.read().await;
        let entries = history.get(service_name)?;
        if entries.is_empty() {
            return None;
        }
        Some(f(entries))
    }

    pub async fn aggregate_average(&self, service_name: &str) -> Option<ResourceUsage> {
        self.aggregate_internal(service_name, |entries| {
            let len = entries.len() as f32;
            let mut avg = entries[0].clone();
            let total_mem: u64 = entries.iter().map(|e| e.memory_bytes).sum();
            let total_cpu: f32 = entries.iter().map(|e| e.cpu_percent).sum();
            avg.memory_bytes = (total_mem as f32 / len).round() as u64;
            avg.cpu_percent = total_cpu / len;
            avg
        })
        .await
    }

    pub async fn aggregate_min(&self, service_name: &str) -> Option<ResourceUsage> {
        self.aggregate_internal(service_name, |entries| {
            entries
                .iter()
                .cloned()
                .min_by_key(|e| e.memory_bytes)
                .unwrap_or_else(|| entries[0].clone())
        })
        .await
    }

    pub async fn aggregate_max(&self, service_name: &str) -> Option<ResourceUsage> {
        self.aggregate_internal(service_name, |entries| {
            entries
                .iter()
                .cloned()
                .max_by_key(|e| e.memory_bytes)
                .unwrap_or_else(|| entries[0].clone())
        })
        .await
    }
    
    pub async fn get_process_id(&self, service_name: &str) -> Option<u32> {
        let pids = self.process_ids.read().await;
        pids.get(service_name).copied()
    }
    
    pub async fn list_services(&self) -> Vec<String> {
        let services = self.services.read().await;
        services.keys().cloned().collect()
    }
    
    pub async fn unregister_service(&self, service_name: &str) {
        let mut services = self.services.write().await;
        services.remove(service_name);
        
        let mut history = self.history.write().await;
        history.remove(service_name);

        let mut pids = self.process_ids.write().await;
        pids.remove(service_name);
        
        info!("Unregistered resource tracking for service: {}", service_name);
    }
}

impl Default for ServiceResourceTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_service_resource_tracker() {
        let tracker = ServiceResourceTracker::new();
        
        tracker.register_service("test".to_string(), 12345).await;
        
        let usage = tracker.get_usage("test").await;
        assert!(usage.is_some());
        assert_eq!(usage.unwrap().service_name, "test");
    }
    
    #[tokio::test]
    async fn test_update_usage() {
        let tracker = ServiceResourceTracker::new();
        
        tracker.register_service("test".to_string(), 12345).await;
        tracker.update_usage("test", 1024 * 1024, 25.0).await;
        
        let usage = tracker.get_usage("test").await.unwrap();
        assert_eq!(usage.memory_bytes, 1024 * 1024);
        assert_eq!(usage.cpu_percent, 25.0);
    }
}
