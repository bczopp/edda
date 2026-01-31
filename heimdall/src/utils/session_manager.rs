use crate::utils::{SessionRepository, TokenRepository, models::Session};
use chrono::{Utc, Duration};
use uuid::Uuid;
use std::sync::Arc;
use tokio::time::{interval, Duration as TokioDuration};
use tracing::{info, warn};

pub struct SessionManager {
    session_repo: Arc<SessionRepository>,
    token_repo: Arc<TokenRepository>,
    timeout_hours: u64,
}

impl SessionManager {
    pub fn new(
        session_repo: Arc<SessionRepository>,
        token_repo: Arc<TokenRepository>,
        timeout_hours: u64,
    ) -> Self {
        Self {
            session_repo,
            token_repo,
            timeout_hours,
        }
    }

    pub async fn create_session(
        &self,
        device_id: Uuid,
        user_id: Uuid,
        token_id: Option<Uuid>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let session_id = uuid::Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::hours(self.timeout_hours as i64);

        self.session_repo.create(
            &session_id,
            device_id,
            user_id,
            token_id,
            expires_at,
        ).await?;

        Ok(session_id)
    }

    pub async fn update_activity(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.session_repo.update_last_activity(session_id).await?;
        Ok(())
    }

    pub async fn deactivate_session(&self, session_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.session_repo.deactivate(session_id).await?;
        Ok(())
    }

    pub async fn is_session_active(&self, session: &Session) -> bool {
        // Check if session is marked as active
        if !session.is_active {
            return false;
        }

        // Check if session is expired
        if session.expires_at <= Utc::now() {
            return false;
        }

        // Check if session has timed out due to inactivity (1 hour timeout)
        let inactivity_threshold = Utc::now() - Duration::hours(self.timeout_hours as i64);
        if session.last_activity < inactivity_threshold {
            return false;
        }

        true
    }

    pub fn start_cleanup_task(&self) {
        let session_repo = Arc::clone(&self.session_repo);
        let token_repo = Arc::clone(&self.token_repo);
        
        tokio::spawn(async move {
            let mut interval = interval(TokioDuration::from_secs(3600)); // Every hour
            
            loop {
                interval.tick().await;
                
                // Cleanup expired sessions
                match session_repo.cleanup_expired().await {
                    Ok(count) => {
                        if count > 0 {
                            info!("Cleaned up {} expired sessions", count);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to cleanup expired sessions: {}", e);
                    }
                }
                
                // Cleanup expired tokens
                match token_repo.cleanup_expired().await {
                    Ok(count) => {
                        if count > 0 {
                            info!("Cleaned up {} expired tokens", count);
                        }
                    }
                    Err(e) => {
                        warn!("Failed to cleanup expired tokens: {}", e);
                    }
                }
            }
        });
    }
}
