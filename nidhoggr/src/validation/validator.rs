use ratatoskr::protocol::MessageValidator as RatatoskrValidator;
use ratatoskr::messages::RatatoskrRequest;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Ratatoskr validation error: {0}")]
    RatatoskrError(#[from] ratatoskr::protocol::ValidationError),
    #[error("Device identity validation failed: {0}")]
    DeviceIdentityError(String),
    #[error("Authentication token validation failed: {0}")]
    AuthenticationError(String),
}

/// Nidhöggr-specific message validator
/// Wraps Ratatoskr validation and adds Nidhöggr-specific checks
pub struct NidhoggrValidator {
    ratatoskr_validator: RatatoskrValidator,
}

impl NidhoggrValidator {
    pub fn new() -> Self {
        Self {
            ratatoskr_validator: RatatoskrValidator::new(),
        }
    }

    /// Validate a Ratatoskr request
    /// This includes Ratatoskr protocol validation plus Nidhöggr-specific checks
    pub fn validate_request(&self, request: &RatatoskrRequest) -> Result<(), ValidationError> {
        // First, validate using Ratatoskr validator
        self.ratatoskr_validator.validate_request(request)?;

        // Additional Nidhöggr-specific validation can be added here
        // For example:
        // - Device identity validation (check with Heimdall)
        // - Authentication token validation (check with Heimdall)
        // - Rate limiting (handled separately)
        // - Connection limits (handled separately)

        Ok(())
    }
}

impl Default for NidhoggrValidator {
    fn default() -> Self {
        Self::new()
    }
}
