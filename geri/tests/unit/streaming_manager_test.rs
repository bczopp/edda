//! Tests für Streaming-Manager (Phase 12.1.1).

#[cfg(test)]
mod tests {
    use geri::streaming::{StreamingError, StreamingManager};

    #[test]
    fn collect_chunks_concatenates_success_chunks() {
        let mgr = StreamingManager::default();
        let chunks: Vec<Result<String, geri::streaming::StreamingError>> =
            vec![Ok("Hello ".to_string()), Ok("world".to_string())];
        let result = mgr.collect_chunks(chunks);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello world");
    }

    #[test]
    fn collect_chunks_returns_error_on_first_error() {
        let mgr = StreamingManager::default();
        let chunks = vec![
            Ok("a".to_string()),
            Err(StreamingError::StreamError("fail".to_string())),
            Ok("b".to_string()),
        ];
        let result = mgr.collect_chunks(chunks);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), StreamingError::StreamError(_)));
    }

    #[test]
    fn collect_chunks_empty_returns_empty_string() {
        let mgr = StreamingManager::default();
        let chunks: Vec<Result<String, StreamingError>> = vec![];
        let result = mgr.collect_chunks(chunks);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn collect_chunks_single_chunk() {
        let mgr = StreamingManager::default();
        let chunks: Vec<Result<String, geri::streaming::StreamingError>> = vec![Ok("only".to_string())];
        let result = mgr.collect_chunks(chunks);
        assert_eq!(result.unwrap(), "only");
    }

    #[test]
    fn streaming_error_user_message() {
        let err = StreamingError::StreamError("net failed".to_string());
        let msg = err.user_message();
        assert!(msg.contains("Streaming fehlgeschlagen"));
        assert!(msg.contains("net failed"));
    }
}
