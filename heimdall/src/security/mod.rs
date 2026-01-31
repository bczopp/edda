// Security monitoring module
pub mod audit;
pub mod tls;

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
