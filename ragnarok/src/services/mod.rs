//! Service integration: Odin, Thor, Freki/Geri (Phase 4).

mod odin_integration;
pub mod heimdall_integration;
pub mod audit_logger;

pub use odin_integration::OdinServiceIntegration;
pub use heimdall_integration::{HeimdallIntegration, HeimdallError};
pub use audit_logger::{AuditLogger, AuditEvent};