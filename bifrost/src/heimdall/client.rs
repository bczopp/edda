//! Heimdall Client (Phase 5.1.1). Connection validation interface, stub, retry.

use async_trait::async_trait;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;

#[derive(Error, Debug)]
pub enum HeimdallError {
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("connection error: {0}")]
    Connection(String),
}

/// Request sent to Heimdall for connection validation (optionally signed with device private key).
#[derive(Debug, Clone)]
pub struct ConnectionValidationRequest {
    pub user_id: String,
    pub device_id: String,
    /// Unix timestamp when request was built (for signing).
    pub timestamp: Option<i64>,
    /// Base64 Ed25519 signature over "user_id|device_id|timestamp".
    pub request_signature: Option<String>,
}

impl ConnectionValidationRequest {
    pub fn request_signature(&self) -> Option<&str> {
        self.request_signature.as_deref()
    }
}

/// Response from Heimdall (ALLOW/DENY + optional validation token).
#[derive(Debug, Clone)]
pub struct ConnectionValidationResponse {
    allowed: bool,
    validation_token: Option<String>,
}

impl ConnectionValidationResponse {
    pub fn allow(token: Option<String>) -> Self {
        Self {
            allowed: true,
            validation_token: token,
        }
    }
    pub fn deny() -> Self {
        Self {
            allowed: false,
            validation_token: None,
        }
    }
    pub fn allowed(&self) -> bool {
        self.allowed
    }
    pub fn validation_token(&self) -> Option<&str> {
        self.validation_token.as_deref()
    }
}

/// Provider for connection validation (real Heimdall gRPC or stub).
#[async_trait]
pub trait HeimdallConnectionValidator: Send + Sync {
    async fn validate(
        &self,
        request: &ConnectionValidationRequest,
    ) -> Result<ConnectionValidationResponse, HeimdallError>;
}

/// Client that validates connections via Heimdall (gRPC or stub); includes retry.
pub struct HeimdallClient {
    validator: Arc<dyn HeimdallConnectionValidator>,
    max_retries: u32,
}

impl HeimdallClient {
    pub fn new(validator: Arc<dyn HeimdallConnectionValidator>, max_retries: u32) -> Self {
        Self {
            validator,
            max_retries: max_retries.max(1),
        }
    }

    /// Validates connection with retry on transient errors (connection/validation errors).
    pub async fn validate_connection(
        &self,
        request: &ConnectionValidationRequest,
    ) -> Result<ConnectionValidationResponse, HeimdallError> {
        let mut last_error = None;
        for attempt in 0..self.max_retries {
            if attempt > 0 {
                sleep(Duration::from_millis(50 * (1 << attempt.min(4)))).await;
            }
            match self.validator.validate(request).await {
                Ok(resp) => return Ok(resp),
                Err(e) => last_error = Some(e),
            }
        }
        Err(last_error.unwrap_or_else(|| {
            HeimdallError::Connection("max retries exceeded".to_string())
        }))
    }
}

/// Stub for tests and until real Heimdall gRPC is available.
pub struct HeimdallStub {
    response: Option<ConnectionValidationResponse>,
    fail_count: std::sync::atomic::AtomicU32,
    fail_until: u32,
}

impl HeimdallStub {
    pub fn allow() -> Self {
        Self {
            response: Some(ConnectionValidationResponse::allow(Some("token-1".to_string()))),
            fail_count: std::sync::atomic::AtomicU32::new(0),
            fail_until: 0,
        }
    }
    pub fn deny() -> Self {
        Self {
            response: Some(ConnectionValidationResponse::deny()),
            fail_count: std::sync::atomic::AtomicU32::new(0),
            fail_until: 0,
        }
    }
    /// Fails the first `n` calls, then returns allow.
    pub fn fail_then_allow(n: u32) -> Self {
        Self {
            response: Some(ConnectionValidationResponse::allow(Some("token-1".to_string()))),
            fail_count: std::sync::atomic::AtomicU32::new(0),
            fail_until: n,
        }
    }
    pub fn always_fail() -> Self {
        Self {
            response: None,
            fail_count: std::sync::atomic::AtomicU32::new(0),
            fail_until: u32::MAX,
        }
    }
}

#[async_trait]
impl HeimdallConnectionValidator for HeimdallStub {
    async fn validate(
        &self,
        _request: &ConnectionValidationRequest,
    ) -> Result<ConnectionValidationResponse, HeimdallError> {
        let count = self
            .fail_count
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if count < self.fail_until {
            return Err(HeimdallError::Connection("stub transient failure".to_string()));
        }
        self.response
            .clone()
            .ok_or_else(|| HeimdallError::Validation("stub deny".to_string()))
    }
}
