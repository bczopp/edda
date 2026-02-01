//! Security: TLS config, certificate handling, validation, rate limiting, threat detection.
//! See IMPLEMENTATION_PLAN Phases 3–4, 15.

pub mod anomaly;
pub mod challenge;
pub mod intrusion;
pub mod key_generator;
pub mod key_storage;
pub mod rate_limiter;
pub mod token;

pub use anomaly::{AnomalyAlert, AnomalyDetector, AnomalyEvent};
pub use challenge::{
    ChallengeProofError, ChallengeProofHandler, ChallengeProofValidationError,
    ChallengeProofValidator, ChallengeRequestError, ChallengeRequestHandler,
    ChallengeResponseError, ChallengeResponseGenerator,
};
pub use intrusion::{AlertKind, IntrusionDetector, IntrusionEvent, SecurityAlert};
pub use key_generator::{Ed25519KeyPair, KeyGenerationError, KeyGenerator};
pub use key_storage::{KeyStorage, KeyStorageError};
pub use rate_limiter::{RateLimitExceeded, RateLimiter};
pub use token::{
    SignedToken, TokenGenerationError, TokenGenerator, TokenRefreshError, TokenRefreshManager,
    TokenValidationError, TokenValidator, ValidatedToken,
};