//! Tests für Provider-Error-Handler (Phase 16.1.1).

#[cfg(test)]
mod tests {
    use geri::error_handling::{GrpcStatusCode, ProviderErrorHandler};
    use geri::llm::LLMError;

    #[test]
    fn handle_model_not_available_returns_unavailable() {
        let handler = ProviderErrorHandler::default();
        let err = LLMError::ModelNotAvailable("gpt-4".to_string());
        let (code, msg) = handler.handle_llm(&err);
        assert_eq!(code, GrpcStatusCode::Unavailable);
        assert!(msg.contains("gpt-4"));
    }

    #[test]
    fn handle_processing_failed_returns_internal() {
        let handler = ProviderErrorHandler::default();
        let err = LLMError::ProcessingFailed("timeout".to_string());
        let (code, msg) = handler.handle_llm(&err);
        assert_eq!(code, GrpcStatusCode::Internal);
        assert!(msg.contains("timeout"));
    }

    #[test]
    fn handle_generic_returns_internal() {
        let handler = ProviderErrorHandler::default();
        let (code, msg) = handler.handle_generic("something failed");
        assert_eq!(code, GrpcStatusCode::Internal);
        assert!(msg.contains("something failed"));
    }
}
