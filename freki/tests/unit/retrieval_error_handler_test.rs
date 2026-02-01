#[cfg(test)]
mod tests {
    use freki::retrieval::{RetrievalErrorCategory, RetrievalErrorHandler};
    use std::error::Error;
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
    fn test_categorize_timeout() {
        let h = RetrievalErrorHandler::default();
        let e = TestError("request timed out");
        assert_eq!(h.categorize(&e), RetrievalErrorCategory::Timeout);
    }

    #[test]
    fn test_categorize_vectordb() {
        let h = RetrievalErrorHandler::default();
        let e = TestError("qdrant search failed");
        assert_eq!(h.categorize(&e), RetrievalErrorCategory::VectorDb);
    }

    #[test]
    fn test_categorize_unknown() {
        let h = RetrievalErrorHandler::default();
        let e = TestError("unknown error");
        assert_eq!(h.categorize(&e), RetrievalErrorCategory::Unknown);
    }

    #[test]
    fn test_is_retriable() {
        let h = RetrievalErrorHandler::default();
        assert!(h.is_retriable(RetrievalErrorCategory::VectorDb));
        assert!(h.is_retriable(RetrievalErrorCategory::Timeout));
        assert!(!h.is_retriable(RetrievalErrorCategory::Unknown));
    }

    #[tokio::test]
    async fn test_execute_with_retry_success() {
        let h = RetrievalErrorHandler::new(2, Duration::from_millis(1));
        let r = h
            .execute_with_retry(|| async { Ok::<_, TestError>("ok") })
            .await;
        assert_eq!(r.unwrap(), "ok");
    }

    #[tokio::test]
    async fn test_execute_with_retry_fail_non_retriable() {
        let h = RetrievalErrorHandler::new(2, Duration::from_millis(1));
        let r = h
            .execute_with_retry(|| async { Err::<(), _>(TestError("unknown")) })
            .await;
        assert!(r.is_err());
    }
}
