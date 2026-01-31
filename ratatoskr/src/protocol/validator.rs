use crate::messages::*;
use chrono::Utc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Schema validation failed: {0}")]
    SchemaError(String),
    #[error("Nonce validation failed: {0}")]
    NonceError(String),
    #[error("Signature validation failed: {0}")]
    SignatureError(String),
    #[error("Timestamp validation failed: {0}")]
    TimestampError(String),
}

/// MessageValidator validates Ratatoskr messages
#[derive(Clone)]
pub struct MessageValidator {
    max_timestamp_age_seconds: i64,
    min_nonce_length: usize,
    signature_length: usize,
}

impl MessageValidator {
    /// Create a new MessageValidator with default settings
    pub fn new() -> Self {
        Self {
            max_timestamp_age_seconds: 300, // 5 minutes
            min_nonce_length: 8,
            signature_length: 64, // Ed25519 signature length
        }
    }

    /// Create a new MessageValidator with custom settings
    pub fn with_settings(
        max_timestamp_age_seconds: i64,
        min_nonce_length: usize,
        signature_length: usize,
    ) -> Self {
        Self {
            max_timestamp_age_seconds,
            min_nonce_length,
            signature_length,
        }
    }

    /// Validate request schema
    pub fn validate_request_schema(&self, request: &RatatoskrRequest) -> Result<(), ValidationError> {
        if request.request_id.is_empty() {
            return Err(ValidationError::SchemaError("request_id is required".to_string()));
        }
        if request.device_id.is_empty() {
            return Err(ValidationError::SchemaError("device_id is required".to_string()));
        }
        if request.user_id.is_empty() {
            return Err(ValidationError::SchemaError("user_id is required".to_string()));
        }
        if request.message_type == 0 {
            return Err(ValidationError::SchemaError("message_type cannot be UNKNOWN".to_string()));
        }
        Ok(())
    }

    /// Validate response schema
    pub fn validate_response_schema(&self, response: &RatatoskrResponse) -> Result<(), ValidationError> {
        if response.request_id.is_empty() {
            return Err(ValidationError::SchemaError("request_id is required".to_string()));
        }
        if response.message_type == 0 {
            return Err(ValidationError::SchemaError("message_type cannot be UNKNOWN".to_string()));
        }
        Ok(())
    }

    /// Validate nonce
    pub fn validate_nonce(&self, nonce: &[u8]) -> Result<(), ValidationError> {
        if nonce.is_empty() {
            return Err(ValidationError::NonceError("nonce cannot be empty".to_string()));
        }
        if nonce.len() < self.min_nonce_length {
            return Err(ValidationError::NonceError(
                format!("nonce must be at least {} bytes", self.min_nonce_length)
            ));
        }
        Ok(())
    }

    /// Validate signature length
    pub fn validate_signature_length(&self, signature: &[u8]) -> Result<(), ValidationError> {
        if signature.is_empty() {
            return Err(ValidationError::SignatureError("signature cannot be empty".to_string()));
        }
        if signature.len() != self.signature_length {
            return Err(ValidationError::SignatureError(
                format!("signature must be exactly {} bytes", self.signature_length)
            ));
        }
        Ok(())
    }

    /// Validate timestamp
    pub fn validate_timestamp(&self, timestamp: i64) -> Result<(), ValidationError> {
        let now = Utc::now().timestamp();
        let age = now - timestamp;

        if age < 0 {
            return Err(ValidationError::TimestampError("timestamp is in the future".to_string()));
        }
        if age > self.max_timestamp_age_seconds {
            return Err(ValidationError::TimestampError(
                format!("timestamp is too old (age: {} seconds)", age)
            ));
        }
        Ok(())
    }

    /// Validate full request (schema, nonce, signature, timestamp)
    pub fn validate_request(&self, request: &RatatoskrRequest) -> Result<(), ValidationError> {
        self.validate_request_schema(request)?;
        self.validate_nonce(&request.nonce)?;
        self.validate_signature_length(&request.signature)?;
        self.validate_timestamp(request.timestamp)?;
        Ok(())
    }

    /// Validate full response (schema only, as responses don't have nonce/signature)
    pub fn validate_response(&self, response: &RatatoskrResponse) -> Result<(), ValidationError> {
        self.validate_response_schema(response)?;
        Ok(())
    }
}

impl Default for MessageValidator {
    fn default() -> Self {
        Self::new()
    }
}
