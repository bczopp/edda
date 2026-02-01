//! Request Validator (Phase 16.1.1): WolfRequest-Validation, Input-Sanitization.

use thiserror::Error;

/// Maximale Content-Länge (Bytes) für IndexDocument.
const MAX_CONTENT_BYTES: usize = 10 * 1024 * 1024; // 10 MB
/// Maximales Limit für RetrieveContext.
const MAX_RETRIEVE_LIMIT: u64 = 1000;

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("document_id must not be empty")]
    EmptyDocumentId,
    #[error("content too large (max {} bytes)", MAX_CONTENT_BYTES)]
    ContentTooLarge,
    #[error("embedding must not be empty")]
    EmptyEmbedding,
    #[error("query_embedding must not be empty")]
    EmptyQueryEmbedding,
    #[error("limit must be between 1 and {}", MAX_RETRIEVE_LIMIT)]
    InvalidLimit,
}

/// Validiert gRPC-Requests (IndexDocument, RetrieveContext).
pub struct RequestValidator;

impl RequestValidator {
    pub fn new() -> Self {
        Self
    }

    /// IndexDocumentRequest validieren.
    pub fn validate_index_document(
        document_id: &str,
        content_len: usize,
        embedding_len: usize,
    ) -> Result<(), ValidationError> {
        if document_id.trim().is_empty() {
            return Err(ValidationError::EmptyDocumentId);
        }
        if content_len > MAX_CONTENT_BYTES {
            return Err(ValidationError::ContentTooLarge);
        }
        if embedding_len == 0 {
            return Err(ValidationError::EmptyEmbedding);
        }
        Ok(())
    }

    /// RetrieveContextRequest validieren.
    pub fn validate_retrieve_context(query_embedding_len: usize, limit: u64) -> Result<(), ValidationError> {
        if query_embedding_len == 0 {
            return Err(ValidationError::EmptyQueryEmbedding);
        }
        if limit == 0 || limit > MAX_RETRIEVE_LIMIT {
            return Err(ValidationError::InvalidLimit);
        }
        Ok(())
    }
}

impl Default for RequestValidator {
    fn default() -> Self {
        Self::new()
    }
}
