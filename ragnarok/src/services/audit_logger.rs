//! Audit Logger for tracking user actions

use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    pub user_id: String,
    pub action: String,
    pub resource: String,
    pub timestamp: DateTime<Utc>,
    pub details: Option<String>,
}

pub struct AuditLogger {
    events: Arc<RwLock<Vec<AuditEvent>>>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Log an audit event
    pub async fn log(&self, event: AuditEvent) {
        let mut events = self.events.write().await;
        events.push(event);
        
        // In production, you'd also write to a persistent store or log file
        tracing::info!(
            "Audit: user={}, action={}, resource={}",
            events.last().unwrap().user_id,
            events.last().unwrap().action,
            events.last().unwrap().resource
        );
    }

    /// Get all events for a specific user
    pub async fn get_events_for_user(&self, user_id: &str) -> Vec<AuditEvent> {
        let events = self.events.read().await;
        events
            .iter()
            .filter(|e| e.user_id == user_id)
            .cloned()
            .collect()
    }

    /// Get events by action type
    pub async fn get_events_by_action(&self, action: &str) -> Vec<AuditEvent> {
        let events = self.events.read().await;
        events
            .iter()
            .filter(|e| e.action == action)
            .cloned()
            .collect()
    }

    /// Get all events
    pub async fn get_all_events(&self) -> Vec<AuditEvent> {
        let events = self.events.read().await;
        events.clone()
    }

    /// Clear all events (useful for testing)
    pub async fn clear(&self) {
        let mut events = self.events.write().await;
        events.clear();
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new()
    }
}
