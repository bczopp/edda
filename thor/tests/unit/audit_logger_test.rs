//! Unit tests for AuditLogger (Phase 10).

use thor::actions::ActionContext;
use thor::audit::{AuditLogger, TracingAuditLogger};
use std::sync::Arc;

#[tokio::test]
async fn test_tracing_audit_logger_new() {
    let logger = TracingAuditLogger::new();
    let _: Arc<dyn AuditLogger> = logger;
}

#[tokio::test]
async fn test_tracing_audit_logger_log_dispatch() {
    let logger = TracingAuditLogger::new();
    let context = ActionContext {
        device_id: "dev-1".to_string(),
        user_id: "user-1".to_string(),
        action_id: "act-1".to_string(),
    };
    logger.log_dispatch(&context, "FILE_OPERATION").await;
}

#[tokio::test]
async fn test_tracing_audit_logger_log_result_success() {
    let logger = TracingAuditLogger::new();
    let context = ActionContext {
        device_id: "dev-1".to_string(),
        user_id: "user-1".to_string(),
        action_id: "act-1".to_string(),
    };
    logger.log_result(&context, "FILE_OPERATION", true, None).await;
}

#[tokio::test]
async fn test_tracing_audit_logger_log_result_failure() {
    let logger = TracingAuditLogger::new();
    let context = ActionContext {
        device_id: "dev-1".to_string(),
        user_id: "user-1".to_string(),
        action_id: "act-1".to_string(),
    };
    logger
        .log_result(&context, "FILE_OPERATION", false, Some("permission denied"))
        .await;
}
