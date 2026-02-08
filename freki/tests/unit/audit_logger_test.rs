//! Unit tests for AuditLogger (Phase 15.1.2).

#[cfg(test)]
mod tests {
    use freki::utils::audit::{AuditLogger, AuditSink};
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    struct RecordingSink {
        events: Mutex<Vec<String>>,
    }

    impl AuditSink for RecordingSink {
        fn log_document_indexed(&self, document_id: &str) {
            self.events.lock().unwrap().push(format!("indexed:{}", document_id));
        }
        fn log_document_accessed(&self, document_id: &str) {
            self.events.lock().unwrap().push(format!("accessed:{}", document_id));
        }
        fn log_query(&self, request_id: &str, limit: u32) {
            self.events.lock().unwrap().push(format!("query:{}:{}", request_id, limit));
        }
    }

    fn events(sink: &RecordingSink) -> Vec<String> {
        sink.events.lock().unwrap().clone()
    }

    #[test]
    fn test_log_document_indexed() {
        let sink = Arc::new(RecordingSink::default());
        let logger = AuditLogger::new(sink.clone());
        logger.log_document_indexed("doc-1");
        assert_eq!(events(&sink), vec!["indexed:doc-1"]);
    }

    #[test]
    fn test_log_document_accessed() {
        let sink = Arc::new(RecordingSink::default());
        let logger = AuditLogger::new(sink.clone());
        logger.log_document_accessed("doc-2");
        assert_eq!(events(&sink), vec!["accessed:doc-2"]);
    }

    #[test]
    fn test_log_query() {
        let sink = Arc::new(RecordingSink::default());
        let logger = AuditLogger::new(sink.clone());
        logger.log_query("req-1", 10);
        assert_eq!(events(&sink), vec!["query:req-1:10"]);
    }

    #[test]
    fn test_multiple_events() {
        let sink = Arc::new(RecordingSink::default());
        let logger = AuditLogger::new(sink.clone());
        logger.log_document_indexed("a");
        logger.log_query("r", 5);
        logger.log_document_accessed("b");
        assert_eq!(
            events(&sink),
            vec!["indexed:a", "query:r:5", "accessed:b"]
        );
    }
}
