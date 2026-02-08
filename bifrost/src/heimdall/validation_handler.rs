//! Connection Validation Request Handler (Phase 5.2.1). Build signed request, send to Heimdall, process response.

use crate::heimdall::{
    ConnectionValidationRequest, ConnectionValidationResponse, HeimdallClient, HeimdallError,
};
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConnectionValidationHandlerError {
    #[error("heimdall error: {0}")]
    Heimdall(#[from] HeimdallError),
    #[error("signing failed")]
    Signing,
}

/// Builds signed ConnectionValidationRequest, sends to Heimdall, returns response.
pub struct ConnectionValidationHandler {
    client: HeimdallClient,
}

impl ConnectionValidationHandler {
    pub fn new(client: HeimdallClient) -> Self {
        Self { client }
    }

    /// Builds a ConnectionValidationRequest signed with device private key (user_id|device_id|timestamp).
    pub fn build_signed_request(
        user_id: &str,
        device_id: &str,
        secret_key: &[u8; 32],
    ) -> Result<ConnectionValidationRequest, ConnectionValidationHandlerError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let to_sign = format!("{}|{}|{}", user_id, device_id, timestamp);
        let signing_key = SigningKey::from_bytes(secret_key);
        let signature = signing_key.sign(to_sign.as_bytes());
        let signature_b64 =
            base64::engine::general_purpose::STANDARD.encode(signature.to_bytes().as_slice());
        Ok(ConnectionValidationRequest {
            user_id: user_id.to_string(),
            device_id: device_id.to_string(),
            timestamp: Some(timestamp),
            request_signature: Some(signature_b64),
        })
    }

    /// Sends signed ConnectionValidationRequest to Heimdall and returns the response.
    pub async fn validate_connection(
        &self,
        user_id: &str,
        device_id: &str,
        secret_key: &[u8; 32],
    ) -> Result<ConnectionValidationResponse, ConnectionValidationHandlerError> {
        let request = Self::build_signed_request(user_id, device_id, secret_key)?;
        let response = self.client.validate_connection(&request).await?;
        Ok(response)
    }
}
