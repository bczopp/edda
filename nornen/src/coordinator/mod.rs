use crate::urd::registry::ProviderRegistry;
use crate::verdandi::router::RequestRouter;
use crate::monitoring::collector::MetricsCollector;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
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
    // Service lifecycle tracking
    request_count: Arc<AtomicU64>,
    start_time: SystemTime,
    // Metrics collector for monitoring
    metrics_collector: Arc<MetricsCollector>,
}

impl NornenCoordinator {
    pub fn new(registry: Arc<ProviderRegistry>, router: Arc<RequestRouter>) -> Self {
        Self {
            registry: registry.clone(),
            router: router.clone(),
            request_count: Arc::new(AtomicU64::new(0)),
            start_time: SystemTime::now(),
            metrics_collector: Arc::new(MetricsCollector::new(None)),
        }
    }

    pub async fn coordinate_request(
        &self,
        request_id: &str,
        request_type: &str,
        context: &std::collections::HashMap<String, String>,
    ) -> Result<CoordinationResult, CoordinationError> {
        let start_time = std::time::Instant::now();
        
        // Increment request counter
        self.request_count.fetch_add(1, Ordering::Relaxed);

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
        let result = self.router
            .select_provider(&required_capabilities, &preferences)
            .await;
        
        let response_time_ms = start_time.elapsed().as_millis() as u64;
        
        match &result {
            Ok((provider_id, _, _)) => {
                // Record successful request
                self.metrics_collector.record_success(provider_id, response_time_ms).await;
            }
            Err(_) => {
                // Record failed request (no provider available)
                self.metrics_collector.record_failure("unknown", response_time_ms).await;
            }
        }
        
        let (provider_id, endpoint, score) = result?;

        Ok(CoordinationResult {
            decision: "route".to_string(),
            provider_id,
            confidence: score,
            reasoning: format!("Selected provider {} for request type {}", provider_id, request_type),
        })
    }

    /// Health check - returns service health status
    pub async fn health_check(&self) -> ServiceHealth {
        // Check registry health (try to query providers)
        let registry_healthy = self.registry
            .query_providers(&[], None)
            .await
            .is_ok();

        // Calculate uptime
        let uptime_seconds = self.start_time
            .elapsed()
            .map(|d| d.as_secs())
            .unwrap_or(0);

        ServiceHealth {
            status: if registry_healthy { "healthy" } else { "degraded" }.to_string(),
            uptime_seconds,
            request_count: self.request_count.load(Ordering::Relaxed),
            registry_healthy,
        }
    }

    /// Get service status and statistics
    pub async fn get_status(&self) -> ServiceStatus {
        let health = self.health_check().await;
        
        // Get provider statistics
        let provider_stats = match self.registry.list_providers(1000, 0).await {
            Ok(result) => ProviderStatistics {
                total_providers: result.total,
                active_providers: self.registry
                    .query_providers(&[], Some("active"))
                    .await
                    .map(|p| p.len() as i32)
                    .unwrap_or(0),
            },
            Err(_) => ProviderStatistics {
                total_providers: 0,
                active_providers: 0,
            },
        };

        ServiceStatus {
            health,
            provider_stats,
        }
    }

    /// Get monitoring metrics
    pub async fn get_metrics(&self) -> crate::monitoring::metrics::MonitoringMetrics {
        let active_providers = self.registry
            .query_providers(&[], Some("active"))
            .await
            .map(|p| p.len())
            .unwrap_or(0);
        
        self.metrics_collector.get_metrics(active_providers).await
    }
}

pub struct CoordinationResult {
    pub decision: String,
    pub provider_id: String,
    pub confidence: f64,
    pub reasoning: String,
}

pub struct ServiceHealth {
    pub status: String,
    pub uptime_seconds: u64,
    pub request_count: u64,
    pub registry_healthy: bool,
}

pub struct ServiceStatus {
    pub health: ServiceHealth,
    pub provider_stats: ProviderStatistics,
}

pub struct ProviderStatistics {
    pub total_providers: i32,
    pub active_providers: i32,
}
