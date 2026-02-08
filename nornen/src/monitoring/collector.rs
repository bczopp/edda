use crate::monitoring::metrics::{RequestMetrics, ProviderMetrics, SystemMetrics, MonitoringMetrics};
use crate::cache::ProviderCache;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use tokio::sync::RwLock;

/// Metrics collector for monitoring
pub struct MetricsCollector {
    // Request metrics
    total_requests: Arc<AtomicU64>,
    successful_requests: Arc<AtomicU64>,
    failed_requests: Arc<AtomicU64>,
    total_response_time_ms: Arc<AtomicU64>,
    
    // Provider-specific metrics
    provider_metrics: Arc<RwLock<HashMap<String, ProviderMetrics>>>,
    
    // Timing
    start_time: SystemTime,
    last_request_time: Arc<RwLock<Option<SystemTime>>>,
    
    // Cache reference for cache metrics
    cache: Option<Arc<ProviderCache>>,
}

impl MetricsCollector {
    pub fn new(cache: Option<Arc<ProviderCache>>) -> Self {
        Self {
            total_requests: Arc::new(AtomicU64::new(0)),
            successful_requests: Arc::new(AtomicU64::new(0)),
            failed_requests: Arc::new(AtomicU64::new(0)),
            total_response_time_ms: Arc::new(AtomicU64::new(0)),
            provider_metrics: Arc::new(RwLock::new(HashMap::new())),
            start_time: SystemTime::now(),
            last_request_time: Arc::new(RwLock::new(None)),
            cache,
        }
    }

    /// Record a successful request
    pub async fn record_success(&self, provider_id: &str, response_time_ms: u64) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.successful_requests.fetch_add(1, Ordering::Relaxed);
        self.total_response_time_ms.fetch_add(response_time_ms, Ordering::Relaxed);
        
        // Update provider metrics
        let mut metrics = self.provider_metrics.write().await;
        let provider_metric = metrics.entry(provider_id.to_string()).or_insert_with(|| {
            ProviderMetrics {
                provider_id: provider_id.to_string(),
                ..Default::default()
            }
        });
        
        provider_metric.total_requests += 1;
        provider_metric.successful_requests += 1;
        provider_metric.last_used = Some(SystemTime::now());
        
        // Update average response time
        let total = provider_metric.total_requests;
        let current_avg = provider_metric.average_response_time_ms;
        provider_metric.average_response_time_ms = 
            (current_avg * (total - 1) as f64 + response_time_ms as f64) / total as f64;
        
        // Update last request time
        *self.last_request_time.write().await = Some(SystemTime::now());
    }

    /// Record a failed request
    pub async fn record_failure(&self, provider_id: &str, response_time_ms: u64) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.failed_requests.fetch_add(1, Ordering::Relaxed);
        self.total_response_time_ms.fetch_add(response_time_ms, Ordering::Relaxed);
        
        // Update provider metrics
        let mut metrics = self.provider_metrics.write().await;
        let provider_metric = metrics.entry(provider_id.to_string()).or_insert_with(|| {
            ProviderMetrics {
                provider_id: provider_id.to_string(),
                ..Default::default()
            }
        });
        
        provider_metric.total_requests += 1;
        provider_metric.failed_requests += 1;
        provider_metric.last_used = Some(SystemTime::now());
        
        // Update average response time
        let total = provider_metric.total_requests;
        let current_avg = provider_metric.average_response_time_ms;
        provider_metric.average_response_time_ms = 
            (current_avg * (total - 1) as f64 + response_time_ms as f64) / total as f64;
        
        // Update last request time
        *self.last_request_time.write().await = Some(SystemTime::now());
    }

    /// Get current metrics
    pub async fn get_metrics(&self, active_providers: usize) -> MonitoringMetrics {
        let total_reqs = self.total_requests.load(Ordering::Relaxed);
        let successful_reqs = self.successful_requests.load(Ordering::Relaxed);
        let failed_reqs = self.failed_requests.load(Ordering::Relaxed);
        let total_time_ms = self.total_response_time_ms.load(Ordering::Relaxed);
        
        // Calculate average response time
        let avg_response_time = if total_reqs > 0 {
            total_time_ms as f64 / total_reqs as f64
        } else {
            0.0
        };
        
        // Calculate requests per second
        let uptime_seconds = self.start_time
            .elapsed()
            .map(|d| d.as_secs())
            .unwrap_or(1);
        let requests_per_second = if uptime_seconds > 0 {
            total_reqs as f64 / uptime_seconds as f64
        } else {
            0.0
        };
        
        // Get cache metrics
        let (cache_hit_rate, cache_size, cache_max_size) = if let Some(cache) = &self.cache {
            let stats = cache.stats().await;
            let hit_rate = if total_reqs > 0 {
                // Estimate hit rate (this is simplified - in production, track actual hits/misses)
                // For now, use cache size as proxy
                (stats.size as f64 / stats.max_size as f64).min(1.0)
            } else {
                0.0
            };
            (hit_rate, stats.size, stats.max_size)
        } else {
            (0.0, 0, 0)
        };
        
        // Get provider metrics
        let provider_metrics_guard = self.provider_metrics.read().await;
        let providers: Vec<ProviderMetrics> = provider_metrics_guard.values().cloned().collect();
        
        MonitoringMetrics {
            system: SystemMetrics {
                uptime_seconds,
                total_requests: total_reqs,
                active_providers,
                cache_hit_rate,
                cache_size,
                cache_max_size,
            },
            requests: RequestMetrics {
                total_requests: total_reqs,
                successful_requests: successful_reqs,
                failed_requests: failed_reqs,
                average_response_time_ms: avg_response_time,
                requests_per_second,
            },
            providers,
            timestamp: SystemTime::now(),
        }
    }

    /// Reset metrics (useful for testing or periodic resets)
    pub async fn reset(&self) {
        self.total_requests.store(0, Ordering::Relaxed);
        self.successful_requests.store(0, Ordering::Relaxed);
        self.failed_requests.store(0, Ordering::Relaxed);
        self.total_response_time_ms.store(0, Ordering::Relaxed);
        self.provider_metrics.write().await.clear();
    }
}
