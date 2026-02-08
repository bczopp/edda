//! Token Generator (Phase 4.2.1). Token Validator (Phase 4.2.2). Heimdall-style tokens.

use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
#[error("token generation failed")]
pub struct TokenGenerationError;

#[derive(Error, Debug)]
pub enum TokenValidationError {
    #[error("malformed token")]
    Malformed,
    #[error("invalid signature")]
    InvalidSignature,
    #[error("token expired")]
    Expired,
    #[error("token revoked")]
    Revoked,
}

#[derive(Error, Debug)]
pub enum TokenRefreshError {
    #[error("token validation failed: {0}")]
    Validation(#[from] TokenValidationError),
    #[error("not a refresh token")]
    NotRefreshToken,
    #[error("token generation failed: {0}")]
    Generation(#[from] TokenGenerationError),
}

/// Signed token (access or refresh) with expiration. Format: base64(payload_json).base64(signature).
#[derive(Debug, Clone)]
pub struct SignedToken {
    token_string: String,
    expires_at: i64,
}

impl SignedToken {
    pub fn token_string(&self) -> &str {
        &self.token_string
    }
    pub fn expires_at(&self) -> i64 {
        self.expires_at
    }
}

#[derive(Serialize, Deserialize)]
struct TokenPayload {
    sub: String,
    device_id: String,
    exp: i64,
    #[serde(rename = "type")]
    token_type: String,
    jti: String,
}

/// Generates access and refresh tokens (signed, with expiration) after successful authentication.
pub struct TokenGenerator {
    signing_key: SigningKey,
}

impl TokenGenerator {
    pub fn new(_public_key: [u8; 32], secret_key: [u8; 32]) -> Self {
        Self {
            signing_key: SigningKey::from_bytes(&secret_key),
        }
    }

    /// Generates an access token for subject (user_id) and device_id, valid for the given duration.
    pub fn generate_access_token(
        &self,
        subject: &str,
        device_id: &str,
        expires_in: Duration,
    ) -> Result<SignedToken, TokenGenerationError> {
        self.generate_token(subject, device_id, expires_in, "access")
    }

    /// Generates a refresh token for subject and device_id, valid for the given duration.
    pub fn generate_refresh_token(
        &self,
        subject: &str,
        device_id: &str,
        expires_in: Duration,
    ) -> Result<SignedToken, TokenGenerationError> {
        self.generate_token(subject, device_id, expires_in, "refresh")
    }

    fn generate_token(
        &self,
        subject: &str,
        device_id: &str,
        expires_in: Duration,
        token_type: &str,
    ) -> Result<SignedToken, TokenGenerationError> {
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let exp = now_secs.saturating_add(expires_in.as_secs() as i64);
        let payload = TokenPayload {
            sub: subject.to_string(),
            device_id: device_id.to_string(),
            exp,
            token_type: token_type.to_string(),
            jti: Uuid::new_v4().to_string(),
        };
        let payload_json =
            serde_json::to_string(&payload).map_err(|_| TokenGenerationError)?;
        let signature = self.signing_key.sign(payload_json.as_bytes());
        let payload_b64 =
            base64::engine::general_purpose::STANDARD.encode(payload_json.as_bytes());
        let sig_b64 =
            base64::engine::general_purpose::STANDARD.encode(signature.to_bytes().as_slice());
        let token_string = format!("{}.{}", payload_b64, sig_b64);
        Ok(SignedToken {
            token_string,
            expires_at: exp,
        })
    }
}

/// Result of successful token validation (sub, device_id, type, jti).
#[derive(Debug, Clone)]
pub struct ValidatedToken {
    sub: String,
    device_id: String,
    token_type: String,
    jti: String,
}

impl ValidatedToken {
    pub fn sub(&self) -> &str {
        &self.sub
    }
    pub fn device_id(&self) -> &str {
        &self.device_id
    }
    pub fn token_type(&self) -> &str {
        &self.token_type
    }
    pub fn jti(&self) -> &str {
        &self.jti
    }
}

/// Validates tokens: signature verification, expiration check, revocation check.
pub struct TokenValidator {
    verifying_key: VerifyingKey,
}

impl TokenValidator {
    pub fn new(public_key: [u8; 32]) -> Self {
        Self {
            verifying_key: VerifyingKey::from_bytes(&public_key).expect("valid key"),
        }
    }

    /// Validates token string: signature, expiration (now <= exp), and optional revocation set (jti).
    pub fn validate(
        &self,
        token_string: &str,
        revoked_jtis: Option<&HashSet<String>>,
    ) -> Result<ValidatedToken, TokenValidationError> {
        let parts: Vec<&str> = token_string.splitn(2, '.').collect();
        let (payload_b64, sig_b64) = match parts.as_slice() {
            [p, s] => (*p, *s),
            _ => return Err(TokenValidationError::Malformed),
        };
        let payload_bytes = base64::engine::general_purpose::STANDARD
            .decode(payload_b64)
            .map_err(|_| TokenValidationError::Malformed)?;
        let sig_bytes = base64::engine::general_purpose::STANDARD
            .decode(sig_b64)
            .map_err(|_| TokenValidationError::Malformed)?;
        let sig: [u8; 64] = sig_bytes
            .try_into()
            .map_err(|_| TokenValidationError::InvalidSignature)?;
        let signature = Signature::from_bytes(&sig);
        self.verifying_key
            .verify(&payload_bytes, &signature)
            .map_err(|_| TokenValidationError::InvalidSignature)?;
        let payload: TokenPayload =
            serde_json::from_slice(&payload_bytes).map_err(|_| TokenValidationError::Malformed)?;
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        if now_secs >= payload.exp {
            return Err(TokenValidationError::Expired);
        }
        if let Some(revoked) = revoked_jtis {
            if revoked.contains(&payload.jti) {
                return Err(TokenValidationError::Revoked);
            }
        }
        Ok(ValidatedToken {
            sub: payload.sub,
            device_id: payload.device_id,
            token_type: payload.token_type,
            jti: payload.jti,
        })
    }
}

/// Validates refresh token and issues new access + refresh tokens; supports proactive renewal check.
pub struct TokenRefreshManager {
    validator: TokenValidator,
    generator: TokenGenerator,
    access_token_ttl: Duration,
    refresh_token_ttl: Duration,
}

impl TokenRefreshManager {
    pub fn new(
        validator: TokenValidator,
        generator: TokenGenerator,
        access_token_ttl: Duration,
        refresh_token_ttl: Duration,
    ) -> Self {
        Self {
            validator,
            generator,
            access_token_ttl,
            refresh_token_ttl,
        }
    }

    /// Validates refresh token and generates new access + refresh tokens for the same sub/device_id.
    pub fn refresh(
        &self,
        refresh_token_string: &str,
        revoked_jtis: Option<&HashSet<String>>,
    ) -> Result<(SignedToken, SignedToken), TokenRefreshError> {
        let validated = self
            .validator
            .validate(refresh_token_string, revoked_jtis)?;
        if validated.token_type() != "refresh" {
            return Err(TokenRefreshError::NotRefreshToken);
        }
        let access = self.generator.generate_access_token(
            validated.sub(),
            validated.device_id(),
            self.access_token_ttl,
        )?;
        let refresh = self.generator.generate_refresh_token(
            validated.sub(),
            validated.device_id(),
            self.refresh_token_ttl,
        )?;
        Ok((access, refresh))
    }

    /// Returns true if the token with given expires_at should be renewed now (expires within threshold).
    pub fn should_renew_proactively(&self, expires_at: i64, threshold: Duration) -> bool {
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let threshold_secs = threshold.as_secs() as i64;
        now_secs.saturating_add(threshold_secs) >= expires_at
    }
}
