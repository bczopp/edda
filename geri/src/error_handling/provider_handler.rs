//! Provider-Error-Handler (Phase 16.1.1): Provider-spezifische Fehler → gRPC-Status-Codes.

use crate::llm::LLMError;

/// gRPC-äquivalente Status-Codes (für Mapping zu tonic::Code).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrpcStatusCode {
    /// Interner Fehler (z. B. ProcessingFailed).
    Internal,
    /// Ressource nicht verfügbar (z. B. ModelNotAvailable).
    Unavailable,
    /// Ungültige Argumente (für zukünftige Nutzung).
    InvalidArgument,
}

/// Mappt Provider-Fehler auf gRPC-Status-Code und Nutzer-/Log-Nachricht.
#[derive(Debug, Clone, Copy, Default)]
pub struct ProviderErrorHandler;

impl ProviderErrorHandler {
    /// Behandelt einen LLM-Provider-Fehler; liefert (Code, Nachricht) für gRPC/Logging.
    pub fn handle_llm(&self, err: &LLMError) -> (GrpcStatusCode, String) {
        let (code, msg) = match err {
            LLMError::ModelNotAvailable(m) => (GrpcStatusCode::Unavailable, m.clone()),
            LLMError::ProcessingFailed(m) => (GrpcStatusCode::Internal, m.clone()),
        };
        (code, msg)
    }

    /// Behandelt einen generischen Fehler (z. B. unbekannter Provider); liefert Internal.
    pub fn handle_generic(&self, message: &str) -> (GrpcStatusCode, String) {
        (GrpcStatusCode::Internal, message.to_string())
    }
}
