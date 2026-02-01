//! Connection Blocking Mechanism (Phase 5.4.2). Block on threat, token revocation, security alert, audit log.

use std::sync::Arc;

use super::{ConnectionStatus, ConnectionStatusTracker};

/// Revokes tokens for a blocked connection.
pub trait TokenRevoker: Send + Sync {
    fn revoke_connection(&self, connection_id: &str);
}

/// Sends security alerts when a connection is blocked.
pub trait SecurityAlertSender: Send + Sync {
    fn send_alert(&self, connection_id: &str, reason: &str);
}

/// Writes audit log entries when a connection is blocked.
pub trait AuditLogger: Send + Sync {
    fn log_connection_blocked(&self, connection_id: &str, reason: &str);
}

/// Blocks a connection immediately on threat; updates status, revokes token, sends alert, writes audit log.
pub struct ConnectionBlocker {
    tracker: Arc<ConnectionStatusTracker>,
    token_revoker: Option<Arc<dyn TokenRevoker>>,
    alert_sender: Option<Arc<dyn SecurityAlertSender>>,
    audit_logger: Option<Arc<dyn AuditLogger>>,
}

impl ConnectionBlocker {
    pub fn new(
        tracker: Arc<ConnectionStatusTracker>,
        token_revoker: Option<Arc<dyn TokenRevoker>>,
        alert_sender: Option<Arc<dyn SecurityAlertSender>>,
        audit_logger: Option<Arc<dyn AuditLogger>>,
    ) -> Self {
        Self {
            tracker,
            token_revoker,
            alert_sender,
            audit_logger,
        }
    }

    /// Blocks the connection immediately (sets BLOCKED), revokes token, sends security alert, writes audit log.
    pub fn block_connection(&self, connection_id: &str, reason: &str) {
        self.tracker.update_status(connection_id, ConnectionStatus::Blocked);
        if let Some(ref r) = self.token_revoker {
            r.revoke_connection(connection_id);
        }
        if let Some(ref s) = self.alert_sender {
            s.send_alert(connection_id, reason);
        }
        if let Some(ref l) = self.audit_logger {
            l.log_connection_blocked(connection_id, reason);
        }
    }
}
