use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub connection_latency_ms: Vec<f64>,
    pub message_routing_latency_ms: Vec<f64>,
    pub messages_per_second: f64,
    pub active_connections: usize,
    pub total_messages: u64,
    pub last_updated: DateTime<Utc>,
}

pub struct PerformanceMonitor {
    metrics: Arc<RwLock<PerformanceMetrics>>,
    connection_times: Arc<RwLock<HashMap<String, Instant>>>,
    routing_times: Arc<RwLock<HashMap<String, Instant>>>,
    message_count: Arc<RwLock<u64>>,
    window_start: Arc<RwLock<Instant>>,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(PerformanceMetrics {
                connection_latency_ms: Vec::new(),
                message_routing_latency_ms: Vec::new(),
                messages_per_second: 0.0,
                active_connections: 0,
                total_messages: 0,
                last_updated: Utc::now(),
            })),
            connection_times: Arc::new(RwLock::new(HashMap::new())),
            routing_times: Arc::new(RwLock::new(HashMap::new())),
            message_count: Arc::new(RwLock::new(0)),
            window_start: Arc::new(RwLock::new(Instant::now())),
        }
    }

    pub async fn record_connection_start(&self, connection_id: String) {
        let mut times = self.connection_times.write().await;
        times.insert(connection_id, Instant::now());
    }

    pub async fn record_connection_end(&self, connection_id: String) {
        let mut times = self.connection_times.write().await;
        if let Some(start_time) = times.remove(&connection_id) {
            let latency = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to ms
            
            let mut metrics = self.metrics.write().await;
            metrics.connection_latency_ms.push(latency);
            
            // Keep only last 1000 measurements
            if metrics.connection_latency_ms.len() > 1000 {
                metrics.connection_latency_ms.remove(0);
            }
            
            metrics.last_updated = Utc::now();
        }
    }

    pub async fn record_routing_start(&self, request_id: String) {
        let mut times = self.routing_times.write().await;
        times.insert(request_id, Instant::now());
    }

    pub async fn record_routing_end(&self, request_id: String) {
        let mut times = self.routing_times.write().await;
        if let Some(start_time) = times.remove(&request_id) {
            let latency = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to ms
            
            let mut metrics = self.metrics.write().await;
            metrics.message_routing_latency_ms.push(latency);
            
            // Keep only last 1000 measurements
            if metrics.message_routing_latency_ms.len() > 1000 {
                metrics.message_routing_latency_ms.remove(0);
            }
            
            metrics.last_updated = Utc::now();
        }
    }

    pub async fn record_message(&self) {
        let mut count = self.message_count.write().await;
        *count += 1;
        
        let mut metrics = self.metrics.write().await;
        metrics.total_messages += 1;
        
        // Calculate messages per second over last 10 seconds
        let window_start = *self.window_start.read().await;
        let elapsed = window_start.elapsed();
        
        if elapsed.as_secs() >= 10 {
            let mut window = self.window_start.write().await;
            *window = Instant::now();
            metrics.messages_per_second = *count as f64 / 10.0;
            *count = 0;
        }
        
        metrics.last_updated = Utc::now();
    }

    pub async fn update_active_connections(&self, count: usize) {
        let mut metrics = self.metrics.write().await;
        metrics.active_connections = count;
        metrics.last_updated = Utc::now();
    }

    pub async fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.read().await.clone()
    }

    pub async fn get_avg_connection_latency(&self) -> f64 {
        let metrics = self.metrics.read().await;
        if metrics.connection_latency_ms.is_empty() {
            return 0.0;
        }
        metrics.connection_latency_ms.iter().sum::<f64>() / metrics.connection_latency_ms.len() as f64
    }

    pub async fn get_avg_routing_latency(&self) -> f64 {
        let metrics = self.metrics.read().await;
        if metrics.message_routing_latency_ms.is_empty() {
            return 0.0;
        }
        metrics.message_routing_latency_ms.iter().sum::<f64>() / metrics.message_routing_latency_ms.len() as f64
    }
}
