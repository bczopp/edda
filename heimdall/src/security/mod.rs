// Security monitoring module
pub mod audit;
pub mod tls;
pub mod e2e;

pub use audit::{
    AuditLogger,
    AuditError,
    ThreatDetector,
    IncidentResponseManager,
    SecurityAnalyticsEngine,
    SecurityMetrics,
};
pub use tls::{
    TLSConfigManager,
    TLSConfig,
    TLSConfigError,
    ProtocolVersion,
};
pub use e2e::{
    E2EEncryptionManager,
    E2EEncryptionError,
    KeyPair,
};