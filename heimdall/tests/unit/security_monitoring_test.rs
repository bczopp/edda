#[cfg(test)]
mod tests {
    use heimdall::security::{AuditLogger, ThreatDetector, IncidentResponseManager, SecurityAnalyticsEngine};
    use uuid::Uuid;
    use sqlx::Row;
    use crate::common::{TestDatabase, create_test_device};

    #[tokio::test]
    async fn test_audit_logging() {
        let test_db = TestDatabase::new().await.unwrap();
        let logger = AuditLogger::new(test_db.pool.clone());
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        logger.log_event(
            "authentication",
            Some(device_id),
            Some(user_id),
            Some("session"),
            Some("login"),
            "success",
            None,
        ).await.unwrap();
        
        // Verify log was created
        let row = sqlx::query("SELECT COUNT(*) as count FROM audit_logs WHERE event_type = 'authentication'")
            .fetch_one(&test_db.pool)
            .await
            .unwrap();
        let count: i64 = row.get("count");
        assert!(count > 0);
    }

    #[tokio::test]
    async fn test_threat_detection_brute_force() {
        let test_db = TestDatabase::new().await.unwrap();
        let logger = AuditLogger::new(test_db.pool.clone());
        let detector = ThreatDetector::new(test_db.pool.clone(), logger.clone(), 5); // 5 failed attempts threshold
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        // Simulate multiple failed authentication attempts
        for i in 0..6 {
            logger.log_event(
                "authentication",
                Some(device_id),
                Some(user_id),
                Some("session"),
                Some("login"),
                "failure",
                None,
            ).await.unwrap();
            
            if i < 5 {
                assert!(!detector.detect_threats(device_id, user_id).await.unwrap());
            } else {
                // After 5 failures, should detect threat
                assert!(detector.detect_threats(device_id, user_id).await.unwrap());
            }
        }
    }

    #[tokio::test]
    async fn test_incident_response_automatic_blocking() {
        let test_db = TestDatabase::new().await.unwrap();
        let logger = AuditLogger::new(test_db.pool.clone());
        let detector = ThreatDetector::new(test_db.pool.clone(), logger.clone(), 5);
        let response_manager = IncidentResponseManager::new(test_db.pool.clone(), logger.clone(), detector);
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;
        for _ in 0..6 {
            logger.log_event(
                "authentication",
                Some(device_id),
                Some(user_id),
                Some("session"),
                Some("login"),
                "failure",
                None,
            ).await.unwrap();
        }
        
        // Trigger incident response
        response_manager.handle_incident(device_id, user_id, "brute_force_attack").await.unwrap();
        
        // Verify device was blocked
        let row = sqlx::query("SELECT COUNT(*) as count FROM blocked_devices WHERE device_id = $1")
            .bind(device_id)
            .fetch_one(&test_db.pool)
            .await
            .unwrap();
        let count: i64 = row.get("count");
        assert!(count > 0);
    }

    #[tokio::test]
    async fn test_security_analytics_event_analysis() {
        let test_db = TestDatabase::new().await.unwrap();
        let logger = AuditLogger::new(test_db.pool.clone());
        let analytics = SecurityAnalyticsEngine::new(test_db.pool.clone(), logger.clone());
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        // Log multiple events
        for i in 0..10 {
            logger.log_event(
                "authentication",
                Some(device_id),
                Some(user_id),
                Some("session"),
                Some("login"),
                if i % 2 == 0 { "success" } else { "failure" },
                None,
            ).await.unwrap();
        }
        
        // Analyze events
        let metrics = analytics.analyze_events(
            Some(device_id),
            Some(user_id),
            None,
        ).await.unwrap();
        
        assert!(metrics.total_events > 0);
        assert!(metrics.successful_events > 0);
        assert!(metrics.failed_events > 0);
    }
}
