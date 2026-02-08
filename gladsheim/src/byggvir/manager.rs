//! ResourceManager - high level orchestrator for Byggvir components.
//!
//! Combines `SystemResourceMonitor`, `ServiceResourceTracker`,
//! `ResourceLimitChecker` and `ResourceEnforcer` to provide
//! continuous resource monitoring and enforcement.

use crate::byggvir::{
    limits::ResourceLimits,
    service_tracker::{ResourceUsage, ServiceResourceTracker},
    enforcer::{ResourceEnforcer, EnforcementAction},
    system_monitor::SystemResourceMonitor,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, watch};
use tokio::task::JoinHandle;
use std::time::Duration;
use sysinfo::Pid;

/// Handle to a running monitoring loop. Dropping this handle
/// signals the loop to stop.
pub struct MonitoringHandle {
    stop_tx: watch::Sender<bool>,
    _join_handle: JoinHandle<()>,
}

impl Drop for MonitoringHandle {
    fn drop(&mut self) {
        let _ = self.stop_tx.send(true);
    }
}

pub struct ResourceManager {
    system_monitor: Arc<Mutex<SystemResourceMonitor>>,
    service_tracker: Arc<ServiceResourceTracker>,
    enforcer: Arc<ResourceEnforcer>,
    limits: Arc<RwLock<HashMap<String, ResourceLimits>>>,
    skirnir: Option<crate::skirnir::Skirnir>,
}

impl ResourceManager {
    /// Create a new ResourceManager with default components.
    pub fn new() -> Self {
        Self {
            system_monitor: Arc::new(Mutex::new(SystemResourceMonitor::new())),
            service_tracker: Arc::new(ServiceResourceTracker::new()),
            enforcer: Arc::new(ResourceEnforcer::new()),
            limits: Arc::new(RwLock::new(HashMap::new())),
            skirnir: None,
        }
    }

    /// Set Skirnir registry for status updates.
    pub fn set_skirnir(&mut self, skirnir: crate::skirnir::Skirnir) {
        self.skirnir = Some(skirnir);
    }

    /// Register a service for resource monitoring with specific limits.
    pub async fn register_service(
        &self,
        service_name: String,
        process_id: u32,
        limits: ResourceLimits,
    ) {
        self.service_tracker
            .register_service(service_name.clone(), process_id)
            .await;

        let mut map = self.limits.write().await;
        map.insert(service_name.clone(), limits);

        // Update PID in Skirnir if available
        if let Some(ref skirnir) = self.skirnir {
            let _ = skirnir.update_pid(&service_name, process_id).await;
        }
    }

    /// Get current resource usage for a service, if known.
    pub async fn get_resource_usage(&self, service_name: &str) -> Option<ResourceUsage> {
        self.service_tracker.get_usage(service_name).await
    }

    /// Set/override resource limits for a service.
    pub async fn set_limits(&self, service_name: String, limits: ResourceLimits) {
        let mut map = self.limits.write().await;
        map.insert(service_name, limits);
    }

    /// Get resource limits for a service, if configured.
    pub async fn get_limits(&self, service_name: &str) -> Option<ResourceLimits> {
        let map = self.limits.read().await;
        map.get(service_name).cloned()
    }

    /// Unregister a service from tracking and limits.
    pub async fn unregister_service(&self, service_name: &str) {
        let mut map = self.limits.write().await;
        map.remove(service_name);
        self.service_tracker.unregister_service(service_name).await;
    }

    /// Update usage for a service and perform limit checks and enforcement.
    ///
    /// This is the core building block for the monitoring loop and is
    /// also used directly in unit tests with synthetic values.
    pub async fn update_and_enforce(
        &self,
        service_name: &str,
        memory_bytes: u64,
        cpu_percent: f32,
    ) -> Option<EnforcementAction> {
        // Update tracker
        self.service_tracker
            .update_usage(service_name, memory_bytes, cpu_percent)
            .await;

        // Update Skirnir
        if let Some(ref skirnir) = self.skirnir {
            let _ = skirnir.update_resources(service_name, memory_bytes, cpu_percent).await;
        }

        // Lookup limits
        let limits = {
            let map = self.limits.read().await;
            map.get(service_name).cloned()
        }?;

        // Execute enforcement
        let action = self
            .enforcer
            .check_and_enforce(memory_bytes, cpu_percent, &limits)
            .await;

        Some(action)
    }

    /// Start a background monitoring loop that periodically refreshes
    /// system metrics and updates all registered services.
    pub fn start_monitoring_loop(&self, interval: Duration) -> MonitoringHandle {
        let system_monitor = Arc::clone(&self.system_monitor);
        let service_tracker = Arc::clone(&self.service_tracker);
        let limits = Arc::clone(&self.limits);
        let enforcer = Arc::clone(&self.enforcer);
        let skirnir = self.skirnir.clone();

        let (tx, mut rx) = watch::channel(false);

        let handle = tokio::spawn(async move {
            loop {
                if *rx.borrow() {
                    break;
                }

                {
                    let mut monitor = system_monitor.lock().await;
                    monitor.refresh();
                }

                // Snapshot of services to monitor
                let services = service_tracker.list_services().await;

                for service_name in services {
                    if let Some(pid) = service_tracker.get_process_id(&service_name).await {
                        let memory_bytes;
                        let cpu_percent;
                        {
                            let monitor = system_monitor.lock().await;
                            let pid = Pid::from_u32(pid);
                            memory_bytes = monitor.process_memory(pid).unwrap_or(0);
                            cpu_percent = monitor.process_cpu(pid).unwrap_or(0.0);
                        }

                        // Update Skirnir
                        if let Some(ref skirnir) = skirnir {
                            let _ = skirnir.update_resources(&service_name, memory_bytes, cpu_percent).await;
                        }

                        // Lookup limits and enforce
                        let service_limits = {
                            let map = limits.read().await;
                            map.get(&service_name).cloned()
                        };

                        if let Some(limits) = service_limits {
                            // Update usage
                            service_tracker
                                .update_usage(&service_name, memory_bytes, cpu_percent)
                                .await;

                            let _ = enforcer
                                .check_and_enforce(memory_bytes, cpu_percent, &limits)
                                .await;
                        }
                    }
                }

                tokio::select! {
                    _ = tokio::time::sleep(interval) => {},
                    _ = rx.changed() => {},
                }
            }
        });

        MonitoringHandle {
            stop_tx: tx,
            _join_handle: handle,
        }
    }
}
