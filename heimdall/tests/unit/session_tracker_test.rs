#[cfg(test)]
mod tests {
    use heimdall::utils::{SessionRepository, SessionManager, TokenRepository};
    use uuid::Uuid;
    use chrono::Utc;
    use std::sync::Arc;
    use crate::common::{TestDatabase, create_test_device};

    #[tokio::test]
    async fn test_create_session() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo, 1);
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        let session_id = manager.create_session(device_id, user_id, None).await.unwrap();

        let session = session_repo.get_by_session_id(&session_id).await.unwrap();
        assert_eq!(session.device_id, device_id);
        assert_eq!(session.user_id, user_id);
        assert!(session.is_active);
    }

    #[tokio::test]
    async fn test_track_active_sessions() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo, 1);
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        let session_id1 = manager.create_session(device_id, user_id, None).await.unwrap();
        let session_id2 = manager.create_session(device_id, user_id, None).await.unwrap();
        
        let active_sessions = session_repo.get_active_sessions_by_user(user_id).await.unwrap();
        assert_eq!(active_sessions.len(), 2);
        assert!(active_sessions.iter().any(|s| s.session_id == session_id1));
        assert!(active_sessions.iter().any(|s| s.session_id == session_id2));
    }

    #[tokio::test]
    async fn test_session_timeout_inactivity() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo, 1);
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        let session_id = manager.create_session(device_id, user_id, None).await.unwrap();
        
        // Update activity to simulate recent activity
        manager.update_activity(&session_id).await.unwrap();
        
        // Check session is not timed out
        let session = session_repo.get_by_session_id(&session_id).await.unwrap();
        assert!(manager.is_session_active(&session).await);
        
        // Simulate timeout by setting last_activity to 2 hours ago
        sqlx::query(
            "UPDATE sessions SET last_activity = last_activity - INTERVAL '2 hours' WHERE session_id = $1"
        )
        .bind(&session_id)
        .execute(&test_db.pool)
        .await
        .unwrap();
        
        // Reload session
        let expired_session = session_repo.get_by_session_id(&session_id).await.unwrap();
        assert!(!manager.is_session_active(&expired_session).await);
    }

    #[tokio::test]
    async fn test_session_expiration() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo, 1);
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        let session_id = manager.create_session(device_id, user_id, None).await.unwrap();
        
        // Check session is not expired
        let session = session_repo.get_by_session_id(&session_id).await.unwrap();
        assert!(session.expires_at > Utc::now());
        assert!(manager.is_session_active(&session).await);
        
        // Simulate expiration by setting expires_at to past
        sqlx::query(
            "UPDATE sessions SET expires_at = expires_at - INTERVAL '2 hours' WHERE session_id = $1"
        )
        .bind(&session_id)
        .execute(&test_db.pool)
        .await
        .unwrap();
        
        // Reload session
        let expired_session = session_repo.get_by_session_id(&session_id).await.unwrap();
        assert!(!manager.is_session_active(&expired_session).await);
    }

    #[tokio::test]
    async fn test_deactivate_session() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo, 1);
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        let session_id = manager.create_session(device_id, user_id, None).await.unwrap();
        
        // Deactivate session
        manager.deactivate_session(&session_id).await.unwrap();
        
        let session = session_repo.get_by_session_id(&session_id).await.unwrap();
        assert!(!session.is_active);
        assert!(!manager.is_session_active(&session).await);
    }

    #[tokio::test]
    async fn test_get_active_sessions_by_device() {
        let test_db = TestDatabase::new().await.unwrap();
        let session_repo = Arc::new(SessionRepository::new(test_db.pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(test_db.pool.clone()));
        let manager = SessionManager::new(session_repo.clone(), token_repo, 1);
        let user_id = Uuid::new_v4();
        let device = create_test_device(&test_db.pool, &Uuid::new_v4().to_string(), user_id).await.unwrap();
        let device_id = device.id;

        let session_id1 = manager.create_session(device_id, user_id, None).await.unwrap();
        let session_id2 = manager.create_session(device_id, user_id, None).await.unwrap();
        
        // Deactivate one session
        manager.deactivate_session(&session_id1).await.unwrap();
        
        let active_sessions = session_repo.get_active_sessions_by_device(device_id).await.unwrap();
        assert_eq!(active_sessions.len(), 1);
        assert_eq!(active_sessions[0].session_id, session_id2);
    }
}
