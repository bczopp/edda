//! Security Test Suite (Phase 19.3.1): Input-Validation, Access-Control.

#[cfg(test)]
mod tests {
    use freki::grpc::{RequestValidator, ValidationError};

    // --- Input-Validation-Tests ---

    #[test]
    fn security_input_validation_rejects_empty_document_id() {
        let r = RequestValidator::validate_index_document("", 100, 384);
        assert!(matches!(r, Err(ValidationError::EmptyDocumentId)));
        let r = RequestValidator::validate_index_document("  ", 100, 384);
        assert!(matches!(r, Err(ValidationError::EmptyDocumentId)));
    }

    #[test]
    fn security_input_validation_rejects_content_too_large() {
        let r = RequestValidator::validate_index_document(
            "doc-1",
            11 * 1024 * 1024, // > 10 MB
            384,
        );
        assert!(matches!(r, Err(ValidationError::ContentTooLarge)));
    }

    #[test]
    fn security_input_validation_rejects_empty_embedding() {
        let r = RequestValidator::validate_index_document("doc-1", 100, 0);
        assert!(matches!(r, Err(ValidationError::EmptyEmbedding)));
    }

    #[test]
    fn security_input_validation_rejects_empty_query_embedding() {
        let r = RequestValidator::validate_retrieve_context(0, 10);
        assert!(matches!(r, Err(ValidationError::EmptyQueryEmbedding)));
    }

    #[test]
    fn security_input_validation_rejects_invalid_limit() {
        let r = RequestValidator::validate_retrieve_context(384, 0);
        assert!(matches!(r, Err(ValidationError::InvalidLimit)));
        let r = RequestValidator::validate_retrieve_context(384, 1001);
        assert!(matches!(r, Err(ValidationError::InvalidLimit)));
    }

    // --- Access-Control (Validation as Gate) ---

    #[test]
    fn security_access_control_valid_index_request_accepted() {
        assert!(RequestValidator::validate_index_document("doc-1", 1000, 384).is_ok());
    }

    #[test]
    fn security_access_control_valid_retrieve_request_accepted() {
        assert!(RequestValidator::validate_retrieve_context(384, 10).is_ok());
        assert!(RequestValidator::validate_retrieve_context(384, 1).is_ok());
        assert!(RequestValidator::validate_retrieve_context(384, 1000).is_ok());
    }
}
