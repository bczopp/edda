use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::Utc;
use serde_json::Value;
use tracing::{info, warn, error};

/// Audit log entry
#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    pub timestamp: chrono::DateTime<Utc>,
    pub event_type: String,
    pub request_id: Option<String>,
    pub user_id: Option<String>,
    pub device_id: Option<String>,
    pub service_name: Option<String>,
    pub action: Option<String>,
    pub status: String,
    pub details: Option<Value>,
}

/// Audit logger for Odin
pub struct AuditLogger {
    logs: Arc<RwLock<Vec<AuditLogEntry>>>,
    max_logs: usize,
}

impl AuditLogger {
    pub fn new(max_logs: usize) -> Self {
        Self {
            logs: Arc::new(RwLock::new(Vec::new())),
            max_logs,
        }
    }

    pub async fn log(
        &self,
        event_type: String,
        request_id: Option<String>,
        user_id: Option<String>,
        device_id: Option<String>,
        service_name: Option<String>,
        action: Option<String>,
        status: String,
        details: Option<Value>,
    ) {
        let entry = AuditLogEntry {
            timestamp: Utc::now(),
            event_type,
            request_id,
            user_id,
            device_id,
            service_name,
            action,
            status,
            details,
        };

        let mut logs = self.logs.write().await;
        logs.push(entry.clone());
        
        // Trim if exceeds max
        if logs.len() > self.max_logs {
            logs.remove(0);
        }

        // Log to tracing
        match status.as_str() {
            "success" => {
                info!(
                    "Audit: {} - {} - {}",
                    entry.event_type,
                    entry.action.as_ref().unwrap_or(&"unknown".to_string()),
                    entry.status
                );
            }
            "failure" | "error" => {
                error!(
                    "Audit: {} - {} - {}",
                    entry.event_type,
                    entry.action.as_ref().unwrap_or(&"unknown".to_string()),
                    entry.status
                );
            }
            _ => {
                warn!(
                    "Audit: {} - {} - {}",
                    entry.event_type,
                    entry.action.as_ref().unwrap_or(&"unknown".to_string()),
                    entry.status
                );
            }
        }
    }

    pub async fn get_logs(&self, limit: Option<usize>) -> Vec<AuditLogEntry> {
        let logs = self.logs.read().await;
        let limit = limit.unwrap_or(logs.len());
        logs.iter().rev().take(limit).cloned().collect()
    }

    pub async fn clear(&self) {
        let mut logs = self.logs.write().await;
        logs.clear();
    }
}

/// Monitoring metrics
#[derive(Debug, Clone)]
pub struct MonitoringMetrics {
    pub active_requests: u64,
    pub queued_requests: u64,
    pub service_health: std::collections::HashMap<String, bool>,
    pub last_updated: chrono::DateTime<Utc>,
}

/// Monitoring service for Odin
pub struct MonitoringService {
    metrics: Arc<RwLock<MonitoringMetrics>>,
    audit_logger: Arc<AuditLogger>,
}

impl MonitoringService {
    pub fn new(audit_logger: Arc<AuditLogger>) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(MonitoringMetrics {
                active_requests: 0,
                queued_requests: 0,
                service_health: std::collections::HashMap::new(),
                last_updated: Utc::now(),
            })),
            audit_logger,
        }
    }

    pub async fn update_active_requests(&self, count: u64) {
        let mut metrics = self.metrics.write().await;
        metrics.active_requests = count;
        metrics.last_updated = Utc::now();
    }

    pub async fn update_queued_requests(&self, count: u64) {
        let mut metrics = self.metrics.write().await;
        metrics.queued_requests = count;
        metrics.last_updated = Utc::now();
    }

    pub async fn update_service_health(&self, service_name: String, healthy: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.service_health.insert(service_name, healthy);
        metrics.last_updated = Utc::now();
    }

    pub async fn get_metrics(&self) -> MonitoringMetrics {
        self.metrics.read().await.clone()
    }

    pub fn audit_logger(&self) -> Arc<AuditLogger> {
        self.audit_logger.clone()
    }
}
