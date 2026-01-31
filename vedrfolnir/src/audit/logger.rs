use chrono::{DateTime, Utc};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub timestamp: DateTime<Utc>,
    pub event_type: AuditEventType,
    pub device_id: Option<String>,
    pub user_id: Option<String>,
    pub details: String,
    pub success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuditEventType {
    ConnectionEstablished,
    ConnectionClosed,
    ConnectionFailed,
    MessageSent,
    MessageReceived,
    HeartbeatSent,
    ReconnectionAttempted,
    ReconnectionSucceeded,
    ReconnectionFailed,
    Error,
}

pub struct AuditLogger {
    logs: Arc<RwLock<Vec<AuditLogEntry>>>,
    max_logs: usize,
}

impl AuditLogger {
    pub fn new(max_logs: usize) -> Self {
        Self {
            logs: Arc::new(RwLock::new(Vec::with_capacity(max_logs))),
            max_logs,
        }
    }

    pub async fn log(
        &self,
        event_type: AuditEventType,
        device_id: Option<String>,
        user_id: Option<String>,
        details: String,
        success: bool,
    ) {
        let entry = AuditLogEntry {
            timestamp: Utc::now(),
            event_type,
            device_id,
            user_id,
            details,
            success,
        };

        // Log to tracing
        info!("Audit: {:?} - {} (success: {})", entry.event_type, entry.details, entry.success);

        // Store log entry
        let mut logs = self.logs.write().await;
        logs.push(entry);
        
        // Keep only recent logs
        if logs.len() > self.max_logs {
            logs.remove(0);
        }
    }

    pub async fn get_recent_logs(&self, limit: usize) -> Vec<AuditLogEntry> {
        let logs = self.logs.read().await;
        logs.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }
}
