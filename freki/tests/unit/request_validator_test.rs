#[cfg(test)]
mod tests {
    use freki::grpc::{RequestValidator, ValidationError};

    #[test]
    fn test_validate_index_document_ok() {
        assert!(RequestValidator::validate_index_document("doc-1", 100, 384).is_ok());
    }

    #[test]
    fn test_validate_index_document_empty_id() {
        let r = RequestValidator::validate_index_document("", 100, 384);
        assert!(matches!(r, Err(ValidationError::EmptyDocumentId)));
        let r = RequestValidator::validate_index_document("  ", 100, 384);
        assert!(matches!(r, Err(ValidationError::EmptyDocumentId)));
    }

    #[test]
    fn test_validate_index_document_content_too_large() {
        let r = RequestValidator::validate_index_document(
            "doc-1",
            11 * 1024 * 1024, // > 10 MB
            384,
        );
        assert!(matches!(r, Err(ValidationError::ContentTooLarge)));
    }

    #[test]
    fn test_validate_index_document_empty_embedding() {
        let r = RequestValidator::validate_index_document("doc-1", 100, 0);
        assert!(matches!(r, Err(ValidationError::EmptyEmbedding)));
    }

    #[test]
    fn test_validate_retrieve_context_ok() {
        assert!(RequestValidator::validate_retrieve_context(384, 10).is_ok());
    }

    #[test]
    fn test_validate_retrieve_context_empty_embedding() {
        let r = RequestValidator::validate_retrieve_context(0, 10);
        assert!(matches!(r, Err(ValidationError::EmptyQueryEmbedding)));
    }

    #[test]
    fn test_validate_retrieve_context_limit_zero() {
        let r = RequestValidator::validate_retrieve_context(384, 0);
        assert!(matches!(r, Err(ValidationError::InvalidLimit)));
    }

    #[test]
    fn test_validate_retrieve_context_limit_too_high() {
        let r = RequestValidator::validate_retrieve_context(384, 1001);
        assert!(matches!(r, Err(ValidationError::InvalidLimit)));
    }
}
