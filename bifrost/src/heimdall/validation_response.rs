//! Validation Response Handler (Phase 5.2.2). Process ConnectionValidationResponse, evaluate ALLOW/DENY, extract token.

use crate::heimdall::ConnectionValidationResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationResponseHandlerError {
    #[error("invalid response: {0}")]
    InvalidResponse(String),
}

/// Result of processing a ConnectionValidationResponse.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationOutcome {
    /// Connection allowed; optional validation token for session.
    Allowed {
        validation_token: Option<String>,
    },
    /// Connection denied.
    Denied,
}

/// Processes ConnectionValidationResponse: evaluates status (ALLOW/DENY) and extracts validation token.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidationResponseHandler;

impl ValidationResponseHandler {
    /// Processes the response and returns an outcome (Allowed with optional token, or Denied).
    pub fn handle_response(
        &self,
        response: &ConnectionValidationResponse,
    ) -> Result<ValidationOutcome, ValidationResponseHandlerError> {
        if response.allowed() {
            Ok(ValidationOutcome::Allowed {
                validation_token: response.validation_token().map(String::from),
            })
        } else {
            Ok(ValidationOutcome::Denied)
        }
    }
}
