use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
use tracing::{warn, error};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct SecurityEvent {
    pub event_type: SecurityEventType,
    pub device_id: String,
    pub user_id: String,
    pub timestamp: DateTime<Utc>,
    pub details: String,
    pub severity: SecuritySeverity,
}

#[derive(Debug, Clone)]
pub enum SecurityEventType {
    RateLimitExceeded,
    InvalidSignature,
    InvalidNonce,
    ConnectionAttempt,
    SuspiciousActivity,
    AuthenticationFailure,
    AuthorizationFailure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct SecurityMonitor {
    events: Arc<RwLock<Vec<SecurityEvent>>>,
    device_activity: Arc<RwLock<HashMap<String, Vec<DateTime<Utc>>>>>,
    max_events: usize,
}

impl SecurityMonitor {
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::with_capacity(max_events))),
            device_activity: Arc::new(RwLock::new(HashMap::new())),
            max_events,
        }
    }

    pub async fn record_event(
        &self,
        event_type: SecurityEventType,
        device_id: String,
        user_id: String,
        details: String,
        severity: SecuritySeverity,
    ) {
        let event = SecurityEvent {
            event_type,
            device_id: device_id.clone(),
            user_id,
            timestamp: Utc::now(),
            details,
            severity,
        };

        // Log based on severity
        match event.severity {
            SecuritySeverity::Critical => error!("Security event: {:?} - {}", event.event_type, event.details),
            SecuritySeverity::High => error!("Security event: {:?} - {}", event.event_type, event.details),
            SecuritySeverity::Medium => warn!("Security event: {:?} - {}", event.event_type, event.details),
            SecuritySeverity::Low => warn!("Security event: {:?} - {}", event.event_type, event.details),
        }

        // Store event
        let mut events = self.events.write().await;
        events.push(event);
        
        // Keep only recent events
        if events.len() > self.max_events {
            events.remove(0);
        }

        // Track device activity
        let mut activity = self.device_activity.write().await;
        let device_events = activity.entry(device_id).or_insert_with(Vec::new);
        device_events.push(Utc::now());
        
        // Keep only recent activity (last hour)
        device_events.retain(|&ts| (Utc::now() - ts).num_seconds() < 3600);
    }

    pub async fn get_recent_events(&self, limit: usize) -> Vec<SecurityEvent> {
        let events = self.events.read().await;
        events.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    pub async fn get_events_by_severity(&self, severity: SecuritySeverity) -> Vec<SecurityEvent> {
        let events = self.events.read().await;
        events.iter()
            .filter(|e| e.severity >= severity)
            .cloned()
            .collect()
    }

    pub async fn check_suspicious_activity(&self, device_id: &str, threshold: usize) -> bool {
        let activity = self.device_activity.read().await;
        if let Some(events) = activity.get(device_id) {
            events.len() >= threshold
        } else {
            false
        }
    }
}
