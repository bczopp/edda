//! Tests for Audit Logging

use ragnarok::services::{AuditLogger, AuditEvent};

#[tokio::test]
async fn test_audit_logger_log_event() {
    let logger = AuditLogger::new();
    
    let event = AuditEvent {
        user_id: "test_user".to_string(),
        action: "chat".to_string(),
        resource: "odin".to_string(),
        timestamp: chrono::Utc::now(),
        details: Some("Sent chat message".to_string()),
    };
    
    logger.log(event).await;
    
    let events = logger.get_events_for_user("test_user").await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].action, "chat");
}

#[tokio::test]
async fn test_audit_logger_multiple_events() {
    let logger = AuditLogger::new();
    
    for i in 0..5 {
        let event = AuditEvent {
            user_id: "user1".to_string(),
            action: format!("action_{}", i),
            resource: "test".to_string(),
            timestamp: chrono::Utc::now(),
            details: None,
        };
        logger.log(event).await;
    }
    
    let events = logger.get_events_for_user("user1").await;
    assert_eq!(events.len(), 5);
}

#[tokio::test]
async fn test_audit_logger_filter_by_action() {
    let logger = AuditLogger::new();
    
    let event1 = AuditEvent {
        user_id: "user1".to_string(),
        action: "chat".to_string(),
        resource: "odin".to_string(),
        timestamp: chrono::Utc::now(),
        details: None,
    };
    
    let event2 = AuditEvent {
        user_id: "user1".to_string(),
        action: "action".to_string(),
        resource: "thor".to_string(),
        timestamp: chrono::Utc::now(),
        details: None,
    };
    
    logger.log(event1).await;
    logger.log(event2).await;
    
    let chat_events = logger.get_events_by_action("chat").await;
    assert_eq!(chat_events.len(), 1);
    assert_eq!(chat_events[0].resource, "odin");
}

#[tokio::test]
async fn test_audit_logger_get_all_events() {
    let logger = AuditLogger::new();
    
    let event1 = AuditEvent {
        user_id: "user1".to_string(),
        action: "chat".to_string(),
        resource: "odin".to_string(),
        timestamp: chrono::Utc::now(),
        details: None,
    };
    
    let event2 = AuditEvent {
        user_id: "user2".to_string(),
        action: "action".to_string(),
        resource: "thor".to_string(),
        timestamp: chrono::Utc::now(),
        details: None,
    };
    
    logger.log(event1).await;
    logger.log(event2).await;
    
    let all_events = logger.get_all_events().await;
    assert_eq!(all_events.len(), 2);
}
