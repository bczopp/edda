//! Audit Logger (Phase 14.1.2). Security-, connection-, and authentication-events for audit trail.

use std::sync::Arc;
use std::sync::RwLock;
use tracing;

/// Single audit event for security, connection, or authentication.
#[derive(Debug, Clone, PartialEq)]
pub enum AuditEvent {
    Security {
        kind: String,
        details: String,
    },
    Connection {
        kind: String,
        connection_id: String,
        device_id: String,
    },
    Authentication {
        kind: String,
        connection_id: String,
        device_id: String,
    },
}

/// Sink for audit events (in-memory for tests, or tracing by default).
pub trait AuditSink: Send + Sync {
    fn log(&self, event: &AuditEvent);
}

/// In-memory sink for tests.
pub struct InMemoryAuditSink {
    events: RwLock<Vec<AuditEvent>>,
}

impl InMemoryAuditSink {
    pub fn new() -> Self {
        Self {
            events: RwLock::new(Vec::new()),
        }
    }

    pub fn events(&self) -> Vec<AuditEvent> {
        self.events.read().unwrap().clone()
    }
}

impl Default for InMemoryAuditSink {
    fn default() -> Self {
        Self::new()
    }
}

impl AuditSink for InMemoryAuditSink {
    fn log(&self, event: &AuditEvent) {
        self.events.write().unwrap().push(event.clone());
    }
}

/// Logs security-relevant, connection, and authentication events (to sink or tracing).
pub struct AuditLogger {
    sink: Option<Arc<dyn AuditSink>>,
}

impl AuditLogger {
    pub fn new(sink: Option<Arc<dyn AuditSink>>) -> Self {
        Self { sink }
    }

    pub fn log_security_event(&self, kind: &str, details: &str) {
        let event = AuditEvent::Security {
            kind: kind.to_string(),
            details: details.to_string(),
        };
        if let Some(ref s) = self.sink {
            s.log(&event);
        } else {
            tracing::info!(
                target: "bifrost.audit",
                event = "security",
                kind = %kind,
                details = %details
            );
        }
    }

    pub fn log_connection_event(&self, kind: &str, connection_id: &str, device_id: &str) {
        let event = AuditEvent::Connection {
            kind: kind.to_string(),
            connection_id: connection_id.to_string(),
            device_id: device_id.to_string(),
        };
        if let Some(ref s) = self.sink {
            s.log(&event);
        } else {
            tracing::info!(
                target: "bifrost.audit",
                event = "connection",
                kind = %kind,
                connection_id = %connection_id,
                device_id = %device_id
            );
        }
    }

    pub fn log_auth_event(&self, kind: &str, connection_id: &str, device_id: &str) {
        let event = AuditEvent::Authentication {
            kind: kind.to_string(),
            connection_id: connection_id.to_string(),
            device_id: device_id.to_string(),
        };
        if let Some(ref s) = self.sink {
            s.log(&event);
        } else {
            tracing::info!(
                target: "bifrost.audit",
                event = "authentication",
                kind = %kind,
                connection_id = %connection_id,
                device_id = %device_id
            );
        }
    }
}
