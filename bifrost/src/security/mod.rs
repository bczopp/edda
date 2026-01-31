//! Security: TLS config, certificate handling, validation, rate limiting, threat detection.
//! See IMPLEMENTATION_PLAN Phases 3–4, 15.

pub mod anomaly;
pub mod intrusion;
pub mod key_generator;
pub mod rate_limiter;

pub use anomaly::{AnomalyAlert, AnomalyDetector, AnomalyEvent};
pub use intrusion::{AlertKind, IntrusionDetector, IntrusionEvent, SecurityAlert};
pub use key_generator::{Ed25519KeyPair, KeyGenerationError, KeyGenerator};
pub use rate_limiter::{RateLimitExceeded, RateLimiter};
