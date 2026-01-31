//! Audit logging for orchestration (Phase 9): security- and compliance-relevant events.
//!
//! Use [`AuditLogger`] with [`RequestProcessor::with_audit_logger`](crate::orchestration::RequestProcessor::with_audit_logger) to record [`AuditEvent`]s (e.g. `RequestReceived`).

use serde::Serialize;

/// Audit event for compliance (who/what/when).
#[derive(Debug, Clone, Serialize)]
pub enum AuditEvent {
    /// A user request was received.
    RequestReceived {
        request_id: String,
        user_id: String,
        device_id: String,
        input_type: String,
    },
}

/// Logger for audit events (injectable, e.g. for tests or file sink).
pub trait AuditLogger: Send + Sync {
    /// Record an audit event.
    fn log(&self, event: &AuditEvent);
}
