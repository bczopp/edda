#[cfg(test)]
mod tests {
    use nornen::audit::{AuditLogger, AuditEvent, PostgresAuditLogger, MimirAuditLogger, CompositeAuditLogger};
    use crate::common::TestDatabase;
    use nornen::mimir_client::MimirClient;
    use serde_json::json;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_postgres_audit_logger() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        
        let logger = PostgresAuditLogger::new(Arc::new(test_db.pool.clone()));
        
        let event = AuditEvent::new(
            "test_event",
            "provider",
            "provider1",
            json!({"test": "data"}),
        );
        
        // Should not fail
        logger.log(event).await.unwrap();
        
        // Verify event was logged
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM audit_logs WHERE event_type = 'test_event' AND provider_id = 'provider1'"
        )
        .fetch_one(&test_db.pool)
        .await
        .unwrap();
        
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn test_audit_event_with_user_id() {
        let event = AuditEvent::new(
            "test_event",
            "provider",
            "provider1",
            json!({"test": "data"}),
        )
        .with_user_id("user123");
        
        assert_eq!(event.event_type, "test_event");
        assert_eq!(event.entity_type, "provider");
        assert_eq!(event.entity_id, "provider1");
        assert_eq!(event.user_id, Some("user123".to_string()));
    }

    #[tokio::test]
    async fn test_composite_audit_logger() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        
        let mut composite = CompositeAuditLogger::new();
        let postgres_logger = Arc::new(PostgresAuditLogger::new(Arc::new(test_db.pool.clone())));
        composite.add_logger(postgres_logger);
        
        let event = AuditEvent::new(
            "composite_test",
            "provider",
            "provider1",
            json!({"test": "composite"}),
        );
        
        // Should not fail
        composite.log(event).await.unwrap();
        
        // Verify event was logged
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM audit_logs WHERE event_type = 'composite_test'"
        )
        .fetch_one(&test_db.pool)
        .await
        .unwrap();
        
        assert_eq!(count, 1);
    }
}
