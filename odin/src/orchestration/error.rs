//! Structured error types for orchestration (Phase 9 Error-Handling).

use thiserror::Error;

/// Errors produced by orchestration flows (responsibility, routing, actions).
#[derive(Error, Debug)]
pub enum OrchestrationError {
    /// No capable service found for the request (empty or irrelevant capabilities).
    #[error("no suitable service found for request")]
    NoServiceFound,

    /// No fallback service available after primary rejected.
    #[error("no fallback service available")]
    NoFallbackService,

    /// Service declined responsibility or failed to accept.
    #[error("service rejected: {0}")]
    ServiceRejected(String),

    /// Action execution or planning failed (e.g. Geri/Thor error).
    #[error("action failed: {0}")]
    ActionFailed(String),

    /// Service is not implemented for direct routing (e.g. Freki standalone).
    #[error("service not implemented for direct routing: {0}")]
    ServiceNotImplemented(String),

    /// Other orchestration error.
    #[error("{0}")]
    Other(String),
}
