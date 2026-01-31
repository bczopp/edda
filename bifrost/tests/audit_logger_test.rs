//! Tests for Phase 14.1.2: AuditLogger (security, connection, auth events).

use bifrost::utils::audit::{AuditEvent, AuditLogger, InMemoryAuditSink};
use std::sync::Arc;

#[test]
fn log_security_event_recorded() {
    let sink = Arc::new(InMemoryAuditSink::new());
    let logger = AuditLogger::new(Some(sink.clone()));
    logger.log_security_event("RATE_LIMIT_EXCEEDED", "key=192.168.1.1");
    let events = sink.events();
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], AuditEvent::Security { ref kind, .. } if kind == "RATE_LIMIT_EXCEEDED"));
}

#[test]
fn log_connection_event_recorded() {
    let sink = Arc::new(InMemoryAuditSink::new());
    let logger = AuditLogger::new(Some(sink.clone()));
    logger.log_connection_event("CONNECTED", "conn-1", "device-1");
    let events = sink.events();
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], AuditEvent::Connection { ref kind, ref connection_id, .. } if kind == "CONNECTED" && connection_id == "conn-1"));
}

#[test]
fn log_auth_event_recorded() {
    let sink = Arc::new(InMemoryAuditSink::new());
    let logger = AuditLogger::new(Some(sink.clone()));
    logger.log_auth_event("CHALLENGE_SENT", "conn-1", "device-1");
    let events = sink.events();
    assert_eq!(events.len(), 1);
    assert!(matches!(events[0], AuditEvent::Authentication { ref kind, .. } if kind == "CHALLENGE_SENT"));
}

#[test]
fn multiple_events_recorded_in_order() {
    let sink = Arc::new(InMemoryAuditSink::new());
    let logger = AuditLogger::new(Some(sink.clone()));
    logger.log_security_event("S1", "d1");
    logger.log_connection_event("C1", "c1", "d1");
    logger.log_auth_event("A1", "c1", "d1");
    let events = sink.events();
    assert_eq!(events.len(), 3);
    assert!(matches!(events[0], AuditEvent::Security { .. }));
    assert!(matches!(events[1], AuditEvent::Connection { .. }));
    assert!(matches!(events[2], AuditEvent::Authentication { .. }));
}
