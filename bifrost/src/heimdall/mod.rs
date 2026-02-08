//! Heimdall integration (Phase 5). Connection validation client, stub, retry, request/response handlers, user-identity, cross-user blocking.

pub mod client;
pub mod cross_user_blocker;
pub mod user_identity;
pub mod validation_handler;
pub mod validation_response;

pub use client::{
    ConnectionValidationRequest, ConnectionValidationResponse, HeimdallClient,
    HeimdallConnectionValidator, HeimdallError, HeimdallStub,
};
pub use cross_user_blocker::CrossUserConnectionBlocker;
pub use user_identity::{
    UserIdentityProvider, UserIdentityVerifier, UserVerificationOutcome,
};
pub use validation_handler::{
    ConnectionValidationHandler, ConnectionValidationHandlerError,
};
pub use validation_response::{
    ValidationOutcome, ValidationResponseHandler, ValidationResponseHandlerError,
};
