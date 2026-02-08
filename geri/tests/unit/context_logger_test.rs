//! Tests für Context-Logger (Phase 18.2.1).

#[cfg(test)]
mod tests {
    use geri::logging::ContextLogger;

    #[test]
    fn new_with_trace_id() {
        let logger = ContextLogger::new("trace-123".to_string());
        assert_eq!(logger.trace_id(), "trace-123");
    }

    #[test]
    fn add_field_and_to_log_string() {
        let mut logger = ContextLogger::new("tid-1".to_string());
        logger.add_field("model_id", "gpt-4");
        logger.add_field("request_type", "prompt");
        let s = logger.to_log_string();
        assert!(s.contains("trace_id=tid-1"));
        assert!(s.contains("model_id=gpt-4"));
        assert!(s.contains("request_type=prompt"));
    }

    #[test]
    fn to_log_string_only_trace_id_when_no_fields() {
        let logger = ContextLogger::new("only".to_string());
        let s = logger.to_log_string();
        assert!(s.contains("trace_id=only"));
    }
}
