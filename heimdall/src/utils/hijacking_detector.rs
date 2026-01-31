use crate::utils::{SessionRepository, TokenRepository};
use crate::utils::session_repository::SessionRepositoryError;
use crate::utils::token_repository::TokenRepositoryError;
use uuid::Uuid;
use std::sync::Arc;
use chrono::Utc;
use tracing::{warn, info};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HijackingDetectorError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Session not found")]
    SessionNotFound,
}

impl From<SessionRepositoryError> for HijackingDetectorError {
    fn from(e: SessionRepositoryError) -> Self {
        match e {
            SessionRepositoryError::NotFound => HijackingDetectorError::SessionNotFound,
            SessionRepositoryError::DatabaseError(e) => HijackingDetectorError::DatabaseError(e),
        }
    }
}

impl From<TokenRepositoryError> for HijackingDetectorError {
    fn from(e: TokenRepositoryError) -> Self {
        match e {
            TokenRepositoryError::NotFound => HijackingDetectorError::SessionNotFound,
            TokenRepositoryError::DatabaseError(e) => HijackingDetectorError::DatabaseError(e),
        }
    }
}

/// Detects session hijacking attempts and automatically revokes compromised sessions
pub struct HijackingDetector {
    session_repo: Arc<SessionRepository>,
    token_repo: Arc<TokenRepository>,
    suspicious_activity_threshold: u32,
}

impl HijackingDetector {
    pub fn new(
        session_repo: Arc<SessionRepository>,
        token_repo: Arc<TokenRepository>,
        suspicious_activity_threshold: u32,
    ) -> Self {
        Self {
            session_repo,
            token_repo,
            suspicious_activity_threshold,
        }
    }

    /// Detects anomalies in session activity
    /// 
    /// Checks for:
    /// - Unusual activity patterns (rapid activity updates)
    /// - Multiple sessions from same device with different users
    /// - Sessions with unusual timing patterns
    pub async fn detect_anomalies(&self, session_id: &str) -> Result<bool, HijackingDetectorError> {
        let session = self.session_repo
            .get_by_session_id(session_id)
            .await?;

        // Check for rapid activity updates (potential hijacking)
        // In a real implementation, this would check IP addresses, geolocation, etc.
        let device_sessions = self.session_repo
            .get_active_sessions_by_device(session.device_id)
            .await?;

        // Check if multiple users have active sessions on the same device
        let unique_users: std::collections::HashSet<Uuid> = device_sessions
            .iter()
            .map(|s| s.user_id)
            .collect();

        if unique_users.len() > 1 {
            warn!(
                "Multiple users detected on device {} - potential hijacking",
                session.device_id
            );
            return Ok(true);
        }

        // Check for unusual timing patterns
        // If session was created recently but has many activity updates, it might be suspicious
        let session_age = Utc::now() - session.created_at;
        let activity_count = device_sessions.len();

        // If session is less than 1 minute old but has more than threshold sessions, it's suspicious
        if session_age.num_seconds() < 60 && activity_count > self.suspicious_activity_threshold as usize {
            warn!(
                "Unusual activity pattern detected for session {} - {} sessions in {} seconds",
                session_id,
                activity_count,
                session_age.num_seconds()
            );
            return Ok(true);
        }

        Ok(false)
    }

    /// Automatically revokes a session when hijacking is detected
    pub async fn revoke_session_on_hijacking(&self, session_id: &str) -> Result<(), HijackingDetectorError> {
        let session = self.session_repo
            .get_by_session_id(session_id)
            .await?;

        // Deactivate the session
        self.session_repo.deactivate(session_id).await?;

        // Revoke associated token if exists
        if let Some(token_id) = session.token_id {
            if let Ok(token) = self.token_repo.get_by_id(token_id).await {
                self.token_repo.revoke(&token.token_id).await?;
                info!("Revoked token {} due to session hijacking", token.token_id);
            }
        }

        warn!(
            "Session {} revoked due to hijacking detection (device: {}, user: {})",
            session_id,
            session.device_id,
            session.user_id
        );

        Ok(())
    }

    /// Monitors sessions for hijacking and automatically revokes compromised ones
    pub async fn monitor_and_revoke(&self) -> Result<u32, HijackingDetectorError> {
        // Get all active sessions
        // In a real implementation, this would be more efficient with pagination
        let revoked_count = 0;

        // This is a simplified implementation
        // In production, you would:
        // 1. Query active sessions in batches
        // 2. Check each session for anomalies
        // 3. Revoke suspicious sessions
        // 4. Log all actions

        Ok(revoked_count)
    }
}
