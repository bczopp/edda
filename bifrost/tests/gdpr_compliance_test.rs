//! GDPR Compliance Tests (Phase 20.3.2).
//! Covers: Data-Minimization, Data-Encryption (no sensitive data in logs), Access-Control,
//! Audit-Logging, Right-to-Erasure.
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::connection::{ConnectionBlocker, ConnectionStatus, ConnectionStatusTracker};
use bifrost::guest::{GuestMeshCleanupManager, GuestMeshManager};
use bifrost::heimdall::{CrossUserConnectionBlocker, UserIdentityVerifier};
use bifrost::message::{BifrostMessage, MessageType, MessageValidator, ValidationError};
use bifrost::utils::audit::{AuditEvent, AuditLogger, InMemoryAuditSink};
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

// --- Data-Minimization (only necessary data; payload size limit; sanitization) ---

#[test]
fn gdpr_data_minimization_validator_rejects_oversized_payload() {
    let validator = MessageValidator::new(64, false);
    let msg = BifrostMessage {
        message_id: "m1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "tgt".to_string(),
        payload: serde_json::json!({"data": "a".repeat(100)}),
        timestamp: 0,
        protocol_version: None,
    };
    let res = validator.validate(&msg);
    assert!(res.is_err());
    assert!(matches!(res.unwrap_err(), ValidationError::PayloadTooLarge));
}

#[test]
fn gdpr_data_minimization_sanitize_truncates_long_ids() {
    let validator = MessageValidator::new(1024, false);
    let mut msg = BifrostMessage {
        message_id: "a".repeat(600),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "tgt".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    };
    msg = validator.sanitize(msg);
    assert!(msg.message_id.len() <= 512);
}

// --- Data-Encryption / No sensitive data in audit (GDPR: don't log full payloads) ---

#[test]
fn gdpr_audit_events_do_not_contain_message_payload() {
    let sink = Arc::new(InMemoryAuditSink::new());
    let logger = AuditLogger::new(Some(sink.clone()));
    logger.log_security_event("BLOCKED", "connection_id=conn-1");
    logger.log_connection_event("CONNECTED", "conn-1", "dev-1");
    logger.log_auth_event("CHALLENGE_SENT", "conn-1", "dev-1");
    let events = sink.events();
    for e in &events {
        match e {
            AuditEvent::Security { kind, details } => {
                assert!(!details.contains("payload"));
                assert!(!details.contains("message_body"));
                let _ = kind;
            }
            AuditEvent::Connection { kind, connection_id, device_id } => {
                let _ = (kind, connection_id, device_id);
            }
            AuditEvent::Authentication { kind, connection_id, device_id } => {
                let _ = (kind, connection_id, device_id);
            }
        }
    }
    assert_eq!(events.len(), 3);
}

// --- Access-Control (cross-user blocking; connection blocking) ---

#[test]
fn gdpr_access_control_cross_user_blocked() {
    let verifier = UserIdentityVerifier::new(None);
    let blocker = CrossUserConnectionBlocker::new(verifier);
    assert!(!blocker.allow_direct_connection("user-a", "user-b"));
    assert!(blocker.requires_relay("user-a", "user-b"));
}

#[test]
fn gdpr_access_control_block_connection_updates_status_and_audit() {
    let tracker = Arc::new(ConnectionStatusTracker::new(None));
    let logs: Arc<Mutex<Vec<(String, String)>>> = Arc::new(Mutex::new(Vec::new()));
    let logger = StubAuditLogger(Arc::clone(&logs));
    let blocker = ConnectionBlocker::new(
        Arc::clone(&tracker),
        None,
        None,
        Some(Arc::new(logger)),
    );
    blocker.block_connection("conn-gdpr", "threat-detected");
    let status = tracker.get_status("conn-gdpr").unwrap();
    assert_eq!(status, ConnectionStatus::Blocked);
    let entries = logs.lock().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0].0, "conn-gdpr");
    assert!(entries[0].1.contains("threat"));
}

// --- Audit-Logging (security-relevant events recorded) ---

#[test]
fn gdpr_audit_logging_security_and_connection_events_recorded() {
    let sink = Arc::new(InMemoryAuditSink::new());
    let logger = AuditLogger::new(Some(sink.clone()));
    logger.log_security_event("RATE_LIMIT", "key=1.2.3.4");
    logger.log_connection_event("DISCONNECTED", "c1", "d1");
    let events = sink.events();
    assert_eq!(events.len(), 2);
    assert!(matches!(events[0], AuditEvent::Security { ref kind, .. } if kind == "RATE_LIMIT"));
    assert!(matches!(events[1], AuditEvent::Connection { ref kind, .. } if kind == "DISCONNECTED"));
}

// --- Right-to-Erasure (guest mesh cleaned up; connection remove erases data) ---

#[tokio::test]
async fn gdpr_right_to_erasure_guest_cleanup_after_idle_removes_mesh() {
    let guest_manager = Arc::new(GuestMeshManager::new());
    let mesh_id = guest_manager.create_guest_mesh();
    let mesh_str = mesh_id.as_str();
    let cleanup = GuestMeshCleanupManager::new(Arc::clone(&guest_manager), Duration::from_millis(5));
    cleanup.register_connection(mesh_str);
    cleanup.on_connection_closed(mesh_str);
    tokio::time::sleep(Duration::from_millis(15)).await;
    cleanup.cleanup_idle();
    assert!(!guest_manager.is_guest_mesh(mesh_str), "guest mesh must be removed (right to erasure)");
}

#[test]
fn gdpr_right_to_erasure_guest_cleanup_removes_mesh_data() {
    let guest_manager = Arc::new(GuestMeshManager::new());
    let mesh_id = guest_manager.create_guest_mesh();
    let mesh_str = mesh_id.as_str();
    assert!(guest_manager.is_guest_mesh(mesh_str));
    guest_manager.remove_guest_mesh(mesh_str);
    assert!(!guest_manager.is_guest_mesh(mesh_str));
}

struct StubAuditLogger(Arc<Mutex<Vec<(String, String)>>>);
impl bifrost::connection::AuditLogger for StubAuditLogger {
    fn log_connection_blocked(&self, connection_id: &str, reason: &str) {
        self.0.lock().unwrap().push((connection_id.to_string(), reason.to_string()));
    }
}
