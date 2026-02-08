//! Roskva - Health Monitor

use crate::utils::Result;
use crate::roskva::monitoring::{HealthMonitor, ServiceHealthTracker};
use crate::roskva::restart_policy::{RestartAttemptTracker, RestartPolicy};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;
use tokio::time::interval;
use tracing::{info, warn};

pub struct Roskva {
    health_monitor: HealthMonitor,
    health_tracker: ServiceHealthTracker,
    skirnir: Option<crate::skirnir::Skirnir>,
}

/// Handle to the monitoring loop; drop to stop the loop.
pub struct MonitoringLoopHandle {
    _shutdown: watch::Sender<()>,
    _join: tokio::task::JoinHandle<()>,
}

impl Roskva {
    /// Start a background task that periodically checks health of all registered services.
    /// If `attempt_tracker` is provided, it is reset for a service when that service becomes healthy.
    /// Returns a handle; dropping it stops the loop.
    pub async fn start_monitoring_loop(
        self: Arc<Self>,
        check_interval: Duration,
        attempt_tracker: Option<Arc<RestartAttemptTracker>>,
    ) -> MonitoringLoopHandle {
        let (tx, mut rx) = watch::channel(());
        let mut ticker = interval(check_interval);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let this = Arc::clone(&self);
        let skirnir = this.skirnir.clone();

        let join = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = ticker.tick() => {
                        let names = this.health_tracker.list_services().await;
                        for name in names {
                            match this.check_service_health(&name).await {
                                Ok(healthy) => {
                                    this.health_tracker.update_health(&name, healthy, None).await;
                                    // Update Skirnir
                                    if let Some(ref skirnir) = skirnir {
                                        let _ = skirnir.update_health(&name, healthy, None).await;
                                    }
                                    if healthy {
                                        if let Some(ref t) = attempt_tracker {
                                            t.reset(&name).await;
                                        }
                                    }
                                }
                                Err(e) => {
                                    this.health_tracker.update_health(&name, false, Some(e.to_string())).await;
                                    // Update Skirnir
                                    if let Some(ref skirnir) = skirnir {
                                        let _ = skirnir.update_health(&name, false, Some(e.to_string())).await;
                                    }
                                    warn!("Health check failed for {}: {}", name, e);
                                }
                            }
                        }
                    }
                    res = rx.changed() => {
                        if res.is_err() {
                            break;
                        }
                    }
                }
            }
            info!("Monitoring loop stopped");
        });

        MonitoringLoopHandle { _shutdown: tx, _join: join }
    }

    /// Returns `Some(backoff_duration)` if the service should be restarted now (caller must call
    /// Thjalfi.restart_service and then attempt_tracker.increment).
    pub async fn evaluate_restart(
        &self,
        service_name: &str,
        policy: &RestartPolicy,
        attempt_tracker: &RestartAttemptTracker,
    ) -> Option<Duration> {
        let should_restart = self.health_tracker.should_restart(service_name).await?;
        if !should_restart {
            return None;
        }
        let attempts = attempt_tracker.get(service_name).await;
        if !policy.should_allow_restart(service_name, attempts) {
            return None;
        }
        Some(policy.backoff_duration(attempts))
    }
}

impl Roskva {
    pub fn new() -> Result<Self> {
        info!("Initializing Roskva (Health Monitor)");
        
        Ok(Self {
            health_monitor: HealthMonitor::default(),
            health_tracker: ServiceHealthTracker::new(Duration::from_secs(5)),
            skirnir: None,
        })
    }

    /// Set Skirnir registry for health status updates.
    pub fn set_skirnir(&mut self, skirnir: crate::skirnir::Skirnir) {
        self.skirnir = Some(skirnir);
    }
    
    pub fn health_monitor(&self) -> &HealthMonitor {
        &self.health_monitor
    }
    
    pub fn health_tracker(&self) -> &ServiceHealthTracker {
        &self.health_tracker
    }
    
    pub async fn check_service_health(&self, service_name: &str) -> Result<bool> {
        // Get health check strategy for service
        let strategy = self.health_tracker.get_strategy(service_name).await;
        
        if let Some(strategy) = strategy {
            match strategy {
                crate::roskva::monitoring::HealthCheckStrategy::Http { url } => {
                    self.health_monitor.check_http_health(&url).await
                }
                crate::roskva::monitoring::HealthCheckStrategy::Grpc { service } => {
                    self.health_monitor.check_grpc_health(&service).await
                }
                crate::roskva::monitoring::HealthCheckStrategy::Process => {
                    // Process-based health check (process is running)
                    Ok(true) // TODO: Check if process is actually running
                }
            }
        } else {
            // No health check strategy registered, assume healthy
            Ok(true)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::roskva::{HealthCheckStrategy, RestartAttemptTracker, RestartPolicy};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_evaluate_restart_returns_backoff_when_should_restart_and_policy_allows() {
        let roskva = Roskva::new().unwrap();
        let policy = RestartPolicy::new(true, 5);
        let tracker = RestartAttemptTracker::new();

        roskva.health_tracker().register_service(
            "svc".to_string(),
            HealthCheckStrategy::Http { url: "http://127.0.0.1:1/health".to_string() },
        ).await;
        roskva.health_tracker().set_max_failures("svc", 2).await;
        roskva.health_tracker().update_health("svc", false, None).await;
        roskva.health_tracker().update_health("svc", false, None).await;

        let backoff = roskva.evaluate_restart("svc", &policy, &tracker).await;
        assert!(backoff.is_some());
    }

    #[tokio::test]
    async fn test_evaluate_restart_none_when_policy_disabled() {
        let roskva = Roskva::new().unwrap();
        let policy = RestartPolicy::new(false, 5);
        let tracker = RestartAttemptTracker::new();

        roskva.health_tracker().register_service(
            "svc".to_string(),
            HealthCheckStrategy::Http { url: "http://127.0.0.1:1/health".to_string() },
        ).await;
        roskva.health_tracker().set_max_failures("svc", 1).await;
        roskva.health_tracker().update_health("svc", false, None).await;

        let backoff = roskva.evaluate_restart("svc", &policy, &tracker).await;
        assert!(backoff.is_none());
    }

    #[tokio::test]
    async fn test_monitoring_loop_accepts_attempt_tracker() {
        let roskva = Arc::new(Roskva::new().unwrap());
        let tracker = Arc::new(RestartAttemptTracker::new());
        roskva.health_tracker().register_service(
            "svc".to_string(),
            HealthCheckStrategy::Http { url: "http://127.0.0.1:59997/health".to_string() },
        ).await;
        let handle = roskva.clone().start_monitoring_loop(Duration::from_millis(50), Some(tracker)).await;
        tokio::time::sleep(Duration::from_millis(80)).await;
        drop(handle);
        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    #[tokio::test]
    async fn test_roskva_creation() {
        let roskva = Roskva::new();
        assert!(roskva.is_ok());
    }

    #[tokio::test]
    async fn test_check_service_health() {
        let roskva = Roskva::new().unwrap();

        // Service without strategy should return healthy
        let result = roskva.check_service_health("nonexistent").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[tokio::test]
    async fn test_monitoring_loop_updates_health_after_tick() {
        let roskva = Arc::new(Roskva::new().unwrap());
        roskva.health_tracker().register_service(
            "loop-test".to_string(),
            HealthCheckStrategy::Http { url: "http://127.0.0.1:59999/health".to_string() },
        ).await;

        let handle = roskva.clone().start_monitoring_loop(Duration::from_millis(50), None).await;
        tokio::time::sleep(Duration::from_millis(120)).await;
        drop(handle);
        tokio::time::sleep(Duration::from_millis(20)).await;

        let health = roskva.health_tracker().get_health("loop-test").await;
        assert!(health.is_some());
        let h = health.unwrap();
        assert!(!h.is_healthy, "unreachable URL should mark service unhealthy");
        assert!(h.consecutive_failures >= 1);
    }

    #[tokio::test]
    async fn test_monitoring_loop_stops_when_handle_dropped() {
        let roskva = Arc::new(Roskva::new().unwrap());
        roskva.health_tracker().register_service(
            "stop-test".to_string(),
            HealthCheckStrategy::Http { url: "http://127.0.0.1:59998/health".to_string() },
        ).await;

        let handle = roskva.start_monitoring_loop(Duration::from_secs(3600), None).await;
        drop(handle);
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
