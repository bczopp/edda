//! Tests for Phase 5.4.2: ConnectionBlocker (block on threat, token revocation, security alert, audit log).

use bifrost::connection::{
    ConnectionBlocker, ConnectionStatus, ConnectionStatusTracker,
};
use std::sync::{Arc, Mutex};

#[test]
fn block_connection_sets_status_blocked() {
    let tracker = Arc::new(ConnectionStatusTracker::new(None));
    let blocker = ConnectionBlocker::new(
        Arc::clone(&tracker),
        None,
        None,
        None,
    );
    tracker.update_status("conn-1", ConnectionStatus::Active);
    blocker.block_connection("conn-1", "threat-detected");
    assert_eq!(tracker.get_status("conn-1"), Some(ConnectionStatus::Blocked));
}

#[test]
fn block_connection_invokes_token_revoker() {
    let tracker = Arc::new(ConnectionStatusTracker::new(None));
    let revoked = Arc::new(Mutex::new(Vec::<String>::new()));
    let revoker = StubTokenRevoker(Arc::clone(&revoked));
    let blocker = ConnectionBlocker::new(
        tracker,
        Some(Arc::new(revoker)),
        None,
        None,
    );
    blocker.block_connection("conn-1", "threat");
    assert_eq!(revoked.lock().unwrap().as_slice(), ["conn-1"]);
}

#[test]
fn block_connection_invokes_alert_sender() {
    let tracker = Arc::new(ConnectionStatusTracker::new(None));
    let alerts = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
    let sender = StubAlertSender(Arc::clone(&alerts));
    let blocker = ConnectionBlocker::new(
        tracker,
        None,
        Some(Arc::new(sender)),
        None,
    );
    blocker.block_connection("conn-1", "suspicious-activity");
    assert_eq!(
        alerts.lock().unwrap().as_slice(),
        [("conn-1".to_string(), "suspicious-activity".to_string())]
    );
}

#[test]
fn block_connection_invokes_audit_logger() {
    let tracker = Arc::new(ConnectionStatusTracker::new(None));
    let logs = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
    let logger = StubAuditLogger(Arc::clone(&logs));
    let blocker = ConnectionBlocker::new(
        tracker,
        None,
        None,
        Some(Arc::new(logger)),
    );
    blocker.block_connection("conn-1", "intrusion");
    assert_eq!(
        logs.lock().unwrap().as_slice(),
        [("conn-1".to_string(), "intrusion".to_string())]
    );
}

#[test]
fn block_connection_invokes_all_when_present() {
    let tracker = Arc::new(ConnectionStatusTracker::new(None));
    let revoked = Arc::new(Mutex::new(Vec::<String>::new()));
    let alerts = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
    let logs = Arc::new(Mutex::new(Vec::<(String, String)>::new()));
    let blocker = ConnectionBlocker::new(
        tracker,
        Some(Arc::new(StubTokenRevoker(Arc::clone(&revoked)))),
        Some(Arc::new(StubAlertSender(Arc::clone(&alerts)))),
        Some(Arc::new(StubAuditLogger(Arc::clone(&logs)))),
    );
    blocker.block_connection("conn-1", "threat");
    assert_eq!(revoked.lock().unwrap().as_slice(), ["conn-1"]);
    assert_eq!(alerts.lock().unwrap().len(), 1);
    assert_eq!(logs.lock().unwrap().len(), 1);
}

struct StubTokenRevoker(Arc<Mutex<Vec<String>>>);
impl bifrost::connection::TokenRevoker for StubTokenRevoker {
    fn revoke_connection(&self, connection_id: &str) {
        self.0.lock().unwrap().push(connection_id.to_string());
    }
}

struct StubAlertSender(Arc<Mutex<Vec<(String, String)>>>);
impl bifrost::connection::SecurityAlertSender for StubAlertSender {
    fn send_alert(&self, connection_id: &str, reason: &str) {
        self.0
            .lock()
            .unwrap()
            .push((connection_id.to_string(), reason.to_string()));
    }
}

struct StubAuditLogger(Arc<Mutex<Vec<(String, String)>>>);
impl bifrost::connection::AuditLogger for StubAuditLogger {
    fn log_connection_blocked(&self, connection_id: &str, reason: &str) {
        self.0
            .lock()
            .unwrap()
            .push((connection_id.to_string(), reason.to_string()));
    }
}
