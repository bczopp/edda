//! Connection-Management (Phase 6.2, 5.4).
//!
//! Enthält Connection-Handler, -Manager, -Cache, Status-Tracker (ACTIVE/IDLE/SUSPICIOUS/BLOCKED),
//! Validation-Cache, Error-Handler und ConnectionBlocker (Token-Revocation, Security-Alert, Audit).

pub mod blocker;
pub mod cache;
pub mod error_handler;
pub mod handler;
pub mod manager;
pub mod status_tracker;
pub mod validation_cache;

pub use blocker::{
    AuditLogger, ConnectionBlocker, SecurityAlertSender, TokenRevoker,
};
pub use cache::{ConnectionCacheManager, ConnectionInfo};
pub use error_handler::{ConnectionErrorAction, ConnectionErrorCategory, ConnectionErrorHandler};
pub use handler::{ConnectionHandler, ConnectionState};
pub use manager::*;
pub use status_tracker::{ConnectionStatus, ConnectionStatusTracker};
pub use validation_cache::{ValidationCacheManager, ValidationResult};
