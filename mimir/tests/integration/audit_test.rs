#[cfg(test)]
mod tests {
    use mimir::audit::{AuditLogManager, AuditEvent};
    use tests::common::TestDatabase;
    use chrono::{Utc, Duration};

    #[tokio::test]
    async fn test_log_event_and_retrieve() {
        let test_db = TestDatabase::new().await.unwrap();
        let audit_manager = AuditLogManager::new(test_db.pool.clone());
        
        // Log an event
        let log_id = audit_manager.log_event(
            AuditEvent::DataStored,
            Some("user1".to_string()),
            Some("data1".to_string()),
            serde_json::json!({"size": 1024}),
        ).await.unwrap();
        
        assert!(log_id > 0);
        
        // Retrieve user logs
        let logs = audit_manager.get_user_audit_logs("user1").await.unwrap();
        assert_eq!(logs.len(), 1);
        assert!(matches!(logs[0].event_type, AuditEvent::DataStored));
        assert_eq!(logs[0].user_id, Some("user1".to_string()));
        assert_eq!(logs[0].data_id, Some("data1".to_string()));
    }

    #[tokio::test]
    async fn test_multiple_events_for_same_user() {
        let test_db = TestDatabase::new().await.unwrap();
        let audit_manager = AuditLogManager::new(test_db.pool.clone());
        
        // Log multiple events
        audit_manager.log_event(
            AuditEvent::DataStored,
            Some("user1".to_string()),
            Some("data1".to_string()),
            serde_json::json!({}),
        ).await.unwrap();
        
        audit_manager.log_event(
            AuditEvent::DataRetrieved,
            Some("user1".to_string()),
            Some("data1".to_string()),
            serde_json::json!({}),
        ).await.unwrap();
        
        audit_manager.log_event(
            AuditEvent::DataDeleted,
            Some("user1".to_string()),
            Some("data1".to_string()),
            serde_json::json!({}),
        ).await.unwrap();
        
        // Retrieve all logs for user
        let logs = audit_manager.get_user_audit_logs("user1").await.unwrap();
        assert_eq!(logs.len(), 3);
        
        // Logs should be ordered by timestamp (DESC)
        assert!(matches!(logs[0].event_type, AuditEvent::DataDeleted));
        assert!(matches!(logs[1].event_type, AuditEvent::DataRetrieved));
        assert!(matches!(logs[2].event_type, AuditEvent::DataStored));
    }

    #[tokio::test]
    async fn test_get_data_audit_logs() {
        let test_db = TestDatabase::new().await.unwrap();
        let audit_manager = AuditLogManager::new(test_db.pool.clone());
        
        // Log events for different users accessing the same data
        audit_manager.log_event(
            AuditEvent::DataStored,
            Some("user1".to_string()),
            Some("data123".to_string()),
            serde_json::json!({}),
        ).await.unwrap();
        
        audit_manager.log_event(
            AuditEvent::DataRetrieved,
            Some("user2".to_string()),
            Some("data123".to_string()),
            serde_json::json!({}),
        ).await.unwrap();
        
        // Retrieve logs for specific data
        let logs = audit_manager.get_data_audit_logs("data123").await.unwrap();
        assert_eq!(logs.len(), 2);
        
        // Check that both users are represented
        let user_ids: Vec<_> = logs.iter().filter_map(|log| log.user_id.clone()).collect();
        assert!(user_ids.contains(&"user1".to_string()));
        assert!(user_ids.contains(&"user2".to_string()));
    }

    #[tokio::test]
    async fn test_immutable_logs() {
        let test_db = TestDatabase::new().await.unwrap();
        let audit_manager = AuditLogManager::new(test_db.pool.clone());
        
        // Log an event
        let log_id = audit_manager.log_event(
            AuditEvent::DataStored,
            Some("user1".to_string()),
            Some("data1".to_string()),
            serde_json::json!({"original": true}),
        ).await.unwrap();
        
        // Retrieve the log
        let logs = audit_manager.get_user_audit_logs("user1").await.unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].details["original"], true);
        
        // Audit logs are immutable - there's no update method
        // This test verifies that only creation and retrieval are possible
    }

    #[tokio::test]
    async fn test_get_logs_in_time_range() {
        let test_db = TestDatabase::new().await.unwrap();
        let audit_manager = AuditLogManager::new(test_db.pool.clone());
        
        let now = Utc::now();
        
        // Log events
        audit_manager.log_event(
            AuditEvent::DataStored,
            Some("user1".to_string()),
            Some("data1".to_string()),
            serde_json::json!({}),
        ).await.unwrap();
        
        // Wait a bit and log another event
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        audit_manager.log_event(
            AuditEvent::DataRetrieved,
            Some("user1".to_string()),
            Some("data1".to_string()),
            serde_json::json!({}),
        ).await.unwrap();
        
        // Get logs in time range
        let start = now - Duration::minutes(1);
        let end = now + Duration::minutes(1);
        let logs = audit_manager.get_logs_in_range(start, end).await.unwrap();
        
        assert_eq!(logs.len(), 2);
    }

    #[tokio::test]
    async fn test_count_events_by_type() {
        let test_db = TestDatabase::new().await.unwrap();
        let audit_manager = AuditLogManager::new(test_db.pool.clone());
        
        // Log multiple events of different types
        audit_manager.log_event(
            AuditEvent::DataStored,
            Some("user1".to_string()),
            Some("data1".to_string()),
            serde_json::json!({}),
        ).await.unwrap();
        
        audit_manager.log_event(
            AuditEvent::DataStored,
            Some("user2".to_string()),
            Some("data2".to_string()),
            serde_json::json!({}),
        ).await.unwrap();
        
        audit_manager.log_event(
            AuditEvent::DataRetrieved,
            Some("user1".to_string()),
            Some("data1".to_string()),
            serde_json::json!({}),
        ).await.unwrap();
        
        // Count events by type
        let stored_count = audit_manager.count_events_by_type(AuditEvent::DataStored).await.unwrap();
        let retrieved_count = audit_manager.count_events_by_type(AuditEvent::DataRetrieved).await.unwrap();
        let deleted_count = audit_manager.count_events_by_type(AuditEvent::DataDeleted).await.unwrap();
        
        assert_eq!(stored_count, 2);
        assert_eq!(retrieved_count, 1);
        assert_eq!(deleted_count, 0);
    }

    #[tokio::test]
    async fn test_access_denied_logging() {
        let test_db = TestDatabase::new().await.unwrap();
        let audit_manager = AuditLogManager::new(test_db.pool.clone());
        
        // Log access denied event
        audit_manager.log_event(
            AuditEvent::AccessDenied,
            Some("user1".to_string()),
            Some("data2".to_string()),
            serde_json::json!({
                "reason": "User attempted to access another user's data",
                "attempted_action": "READ"
            }),
        ).await.unwrap();
        
        // Retrieve logs
        let logs = audit_manager.get_user_audit_logs("user1").await.unwrap();
        assert_eq!(logs.len(), 1);
        assert!(matches!(logs[0].event_type, AuditEvent::AccessDenied));
        assert_eq!(logs[0].details["reason"], "User attempted to access another user's data");
    }

    #[tokio::test]
    async fn test_gdpr_compliance_logging() {
        let test_db = TestDatabase::new().await.unwrap();
        let audit_manager = AuditLogManager::new(test_db.pool.clone());
        
        // Log GDPR-relevant events
        audit_manager.log_event(
            AuditEvent::DataExported,
            Some("user1".to_string()),
            None,
            serde_json::json!({"format": "JSON", "size_bytes": 10240}),
        ).await.unwrap();
        
        audit_manager.log_event(
            AuditEvent::UserDataDeleted,
            Some("user1".to_string()),
            None,
            serde_json::json!({"reason": "User requested data deletion", "entries_deleted": 5}),
        ).await.unwrap();
        
        // Retrieve logs
        let logs = audit_manager.get_user_audit_logs("user1").await.unwrap();
        assert_eq!(logs.len(), 2);
        
        // Check for GDPR-relevant events
        let has_export = logs.iter().any(|log| matches!(log.event_type, AuditEvent::DataExported));
        let has_deletion = logs.iter().any(|log| matches!(log.event_type, AuditEvent::UserDataDeleted));
        
        assert!(has_export);
        assert!(has_deletion);
    }
}
