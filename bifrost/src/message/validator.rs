//! Message Validator (Phase 15.1.1). Format validation, signature (stub), sanitization (injection prevention).

use crate::message::BifrostMessage;
use thiserror::Error;

const MAX_ID_LEN: usize = 512;

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
    #[error("payload too large")]
    PayloadTooLarge,
    #[error("signature invalid")]
    SignatureInvalid,
}

/// Validates message format, optional signature; sanitizes string fields for injection prevention.
pub struct MessageValidator {
    max_payload_bytes: usize,
    check_signature: bool,
}

impl MessageValidator {
    pub fn new(max_payload_bytes: usize, check_signature: bool) -> Self {
        Self {
            max_payload_bytes: max_payload_bytes.max(1),
            check_signature,
        }
    }

    /// Validates format (non-empty ids, payload size) and optionally signature (stub: always ok if disabled).
    pub fn validate(&self, msg: &BifrostMessage) -> Result<(), ValidationError> {
        if msg.message_id.trim().is_empty() {
            return Err(ValidationError::InvalidFormat("message_id must not be empty".into()));
        }
        if msg.source_device_id.trim().is_empty() {
            return Err(ValidationError::InvalidFormat(
                "source_device_id must not be empty".into(),
            ));
        }
        if msg.target_device_id.trim().is_empty() {
            return Err(ValidationError::InvalidFormat(
                "target_device_id must not be empty".into(),
            ));
        }
        let payload_len = serde_json::to_vec(&msg.payload).map(|v| v.len()).unwrap_or(0);
        if payload_len > self.max_payload_bytes {
            return Err(ValidationError::PayloadTooLarge);
        }
        if self.check_signature {
            // Stub: no real signature yet; treat as valid.
        }
        Ok(())
    }

    /// Sanitizes string fields: strip control chars, truncate overlong ids.
    pub fn sanitize(&self, mut msg: BifrostMessage) -> BifrostMessage {
        msg.message_id = sanitize_string(&msg.message_id, MAX_ID_LEN);
        msg.source_device_id = sanitize_string(&msg.source_device_id, MAX_ID_LEN);
        msg.target_device_id = sanitize_string(&msg.target_device_id, MAX_ID_LEN);
        msg
    }
}

fn sanitize_string(s: &str, max_len: usize) -> String {
    let stripped: String = s
        .chars()
        .filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t')
        .collect();
    let trimmed = stripped.trim();
    trimmed.chars().take(max_len).collect()
}
