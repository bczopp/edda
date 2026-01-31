use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, ED25519};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SignatureError {
    #[error("Signature generation failed: {0}")]
    GenerationFailed(String),
    #[error("Signature verification failed: {0}")]
    VerificationFailed(String),
    #[error("Invalid public key: {0}")]
    InvalidPublicKey(String),
}

pub struct SignatureManager;

impl SignatureManager {
    /// Sign message with private key
    pub fn sign(keypair: &Ed25519KeyPair, message: &[u8]) -> Result<Vec<u8>, SignatureError> {
        let signature = keypair.sign(message);
        Ok(signature.as_ref().to_vec())
    }

    /// Verify signature with public key
    pub fn verify(
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<(), SignatureError> {
        let public_key = UnparsedPublicKey::new(&ED25519, public_key);
        public_key
            .verify(message, signature)
            .map_err(|e| SignatureError::VerificationFailed(format!("{}", e)))
    }
}
