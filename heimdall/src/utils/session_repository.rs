use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use crate::utils::models::Session;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SessionRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Session not found")]
    NotFound,
}

pub struct SessionRepository {
    pool: PgPool,
}

impl SessionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        session_id: &str,
        device_id: Uuid,
        user_id: Uuid,
        token_id: Option<Uuid>,
        expires_at: DateTime<Utc>,
    ) -> Result<Session, SessionRepositoryError> {
        let session = sqlx::query_as::<_, Session>(
            r#"
            INSERT INTO sessions (session_id, device_id, user_id, token_id, expires_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(session_id)
        .bind(device_id)
        .bind(user_id)
        .bind(token_id)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(session)
    }

    pub async fn get_by_session_id(&self, session_id: &str) -> Result<Session, SessionRepositoryError> {
        let session = sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE session_id = $1",
        )
        .bind(session_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(SessionRepositoryError::NotFound)?;

        Ok(session)
    }

    pub async fn update_last_activity(&self, session_id: &str) -> Result<(), SessionRepositoryError> {
        sqlx::query(
            "UPDATE sessions SET last_activity = CURRENT_TIMESTAMP WHERE session_id = $1",
        )
        .bind(session_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn deactivate(&self, session_id: &str) -> Result<(), SessionRepositoryError> {
        sqlx::query(
            "UPDATE sessions SET is_active = false WHERE session_id = $1",
        )
        .bind(session_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn cleanup_expired(&self) -> Result<u64, SessionRepositoryError> {
        let result = sqlx::query(
            "DELETE FROM sessions WHERE expires_at < CURRENT_TIMESTAMP"
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    pub async fn get_active_sessions_by_user(&self, user_id: Uuid) -> Result<Vec<Session>, SessionRepositoryError> {
        let sessions = sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions
            WHERE user_id = $1 AND is_active = true AND expires_at > CURRENT_TIMESTAMP
            ORDER BY created_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions)
    }

    pub async fn get_active_sessions_by_device(&self, device_id: Uuid) -> Result<Vec<Session>, SessionRepositoryError> {
        let sessions = sqlx::query_as::<_, Session>(
            r#"
            SELECT * FROM sessions
            WHERE device_id = $1 AND is_active = true AND expires_at > CURRENT_TIMESTAMP
            ORDER BY created_at DESC
            "#
        )
        .bind(device_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(sessions)
    }
}
