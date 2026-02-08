#[cfg(test)]
mod tests {
    use freki::indexing::{IndexingErrorCategory, IndexingErrorHandler};
    use std::error::Error;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::Duration;

    #[derive(Debug)]
    struct TestError(&'static str);
    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl Error for TestError {}

    #[test]
    fn test_categorize_parse() {
        let h = IndexingErrorHandler::default();
        let e = TestError("parse failed: unsupported file type");
        assert_eq!(h.categorize(&e), IndexingErrorCategory::Parse);
    }

    #[test]
    fn test_categorize_embedding() {
        let h = IndexingErrorHandler::default();
        let e = TestError("embedding model not loaded");
        assert_eq!(h.categorize(&e), IndexingErrorCategory::Embedding);
    }

    #[test]
    fn test_categorize_vectordb() {
        let h = IndexingErrorHandler::default();
        let e = TestError("qdrant connection error");
        assert_eq!(h.categorize(&e), IndexingErrorCategory::VectorDb);
    }

    #[test]
    fn test_categorize_unknown() {
        let h = IndexingErrorHandler::default();
        let e = TestError("something went wrong");
        assert_eq!(h.categorize(&e), IndexingErrorCategory::Unknown);
    }

    #[test]
    fn test_is_retriable() {
        let h = IndexingErrorHandler::default();
        assert!(h.is_retriable(IndexingErrorCategory::VectorDb));
        assert!(h.is_retriable(IndexingErrorCategory::Embedding));
        assert!(!h.is_retriable(IndexingErrorCategory::Parse));
        assert!(!h.is_retriable(IndexingErrorCategory::Unknown));
    }

    #[test]
    fn test_user_message() {
        let h = IndexingErrorHandler::default();
        assert!(!h.user_message(IndexingErrorCategory::Parse).is_empty());
        assert!(!h.user_message(IndexingErrorCategory::VectorDb).is_empty());
    }

    #[tokio::test]
    async fn test_execute_with_retry_success_first_try() {
        let h = IndexingErrorHandler::new(2, Duration::from_millis(1));
        let count = std::sync::Arc::new(AtomicU32::new(0));
        let count_clone = std::sync::Arc::clone(&count);
        let r = h
            .execute_with_retry(move || {
                let c = std::sync::Arc::clone(&count_clone);
                async move {
                    c.fetch_add(1, Ordering::SeqCst);
                    Ok::<_, TestError>(42)
                }
            })
            .await;
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), 42);
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_execute_with_retry_fail_after_max() {
        let h = IndexingErrorHandler::new(2, Duration::from_millis(1));
        let r = h
            .execute_with_retry(|| async { Err::<i32, _>(TestError("parse error")) })
            .await;
        assert!(r.is_err());
    }
}
