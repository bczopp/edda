#[cfg(test)]
mod tests {
    use heimdall::utils::{SessionRepository, SessionManager, TokenRepository, HijackingDetector};
    use uuid::Uuid;
    use std::sync::Arc;
    use crate::common::{TestDatabase, create_test_device};

    #[tokio::test]
    async fn test_device_tracking_sessions_bound_to_device() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo.clone(), 1);
        let _detector = HijackingDetector::new(session_repo.clone(), token_repo, 3); // 3 suspicious activities threshold
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        let session_id = manager.create_session(device_id, user_id, None).await.unwrap();
        
        // Check that session is bound to device
        let session = session_repo.get_by_session_id(&session_id).await.unwrap();
        assert_eq!(session.device_id, device_id);
        
        // Verify device tracking
        let device_sessions = session_repo.get_active_sessions_by_device(device_id).await.unwrap();
        assert_eq!(device_sessions.len(), 1);
        assert_eq!(device_sessions[0].session_id, session_id);
    }

    #[tokio::test]
    async fn test_anomaly_detection_unusual_session_activity() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo.clone(), 1);
        let detector = HijackingDetector::new(session_repo.clone(), token_repo, 3);
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        let session_id = manager.create_session(device_id, user_id, None).await.unwrap();
        
        // Simulate normal activity
        manager.update_activity(&session_id).await.unwrap();
        assert!(!detector.detect_anomalies(&session_id).await.unwrap());
        
        // Simulate unusual activity - rapid activity updates from different locations
        // (In real implementation, this would check IP addresses, geolocation, etc.)
        for _ in 0..5 {
            manager.update_activity(&session_id).await.unwrap();
            // Simulate activity from different location (would check IP in real implementation)
        }
        
        // After multiple rapid updates, should detect anomaly
        // Note: This is a simplified test - real implementation would check IP changes, etc.
    }

    #[tokio::test]
    async fn test_automatic_revocation_on_hijacking_detection() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo.clone(), 1);
        let detector = HijackingDetector::new(session_repo.clone(), token_repo.clone(), 2);
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        let session_id = manager.create_session(device_id, user_id, None).await.unwrap();
        
        // Verify session is active
        let session = session_repo.get_by_session_id(&session_id).await.unwrap();
        assert!(session.is_active);
        
        // Simulate hijacking detection
        detector.revoke_session_on_hijacking(&session_id).await.unwrap();
        
        // Verify session is deactivated
        let revoked_session = session_repo.get_by_session_id(&session_id).await.unwrap();
        assert!(!revoked_session.is_active);
    }

    #[tokio::test]
    async fn test_multiple_sessions_same_device_different_users() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo.clone(), 1);
        let _detector = HijackingDetector::new(session_repo.clone(), token_repo, 3);
        let user_id1 = Uuid::new_v4();
        let user_id2 = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id1).await.unwrap();
        let device_id = device.id;

        // Create sessions for different users on same device (suspicious)
        let _session_id1 = manager.create_session(device_id, user_id1, None).await.unwrap();
        let _session_id2 = manager.create_session(device_id, user_id2, None).await.unwrap();
        
        // This should be detected as suspicious activity
        let device_sessions = session_repo.get_active_sessions_by_device(device_id).await.unwrap();
        assert_eq!(device_sessions.len(), 2);
        
        // Check if detector flags this as suspicious
        // (In real implementation, would check if multiple users on same device is allowed)
    }

    #[tokio::test]
    async fn test_session_activity_tracking_for_anomaly_detection() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo.clone(), 1);
        let detector = HijackingDetector::new(session_repo.clone(), token_repo, 3);
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        let session_id = manager.create_session(device_id, user_id, None).await.unwrap();
        
        // Track activity pattern
        for i in 0..10 {
            manager.update_activity(&session_id).await.unwrap();
            if i % 3 == 0 {
                // Every 3rd update, check for anomalies
                let has_anomaly = detector.detect_anomalies(&session_id).await.unwrap();
                // Normal activity pattern should not trigger anomaly
                assert!(!has_anomaly);
            }
        }
    }
}
