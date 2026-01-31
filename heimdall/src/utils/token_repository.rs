use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use crate::utils::models::Token;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Token not found")]
    NotFound,
}

pub struct TokenRepository {
    pool: PgPool,
}

impl TokenRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        token_id: &str,
        device_id: Uuid,
        user_id: Uuid,
        token_type: &str,
        token_data: &str,
        expires_at: DateTime<Utc>,
    ) -> Result<Token, TokenRepositoryError> {
        let token = sqlx::query_as::<_, Token>(
            r#"
            INSERT INTO tokens (token_id, device_id, user_id, token_type, token_data, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING *
            "#,
        )
        .bind(token_id)
        .bind(device_id)
        .bind(user_id)
        .bind(token_type)
        .bind(token_data)
        .bind(expires_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(token)
    }

    pub async fn get_by_token_id(&self, token_id: &str) -> Result<Token, TokenRepositoryError> {
        let token = sqlx::query_as::<_, Token>(
            "SELECT * FROM tokens WHERE token_id = $1",
        )
        .bind(token_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(TokenRepositoryError::NotFound)?;

        Ok(token)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Token, TokenRepositoryError> {
        let token = sqlx::query_as::<_, Token>(
            "SELECT * FROM tokens WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(TokenRepositoryError::NotFound)?;

        Ok(token)
    }

    pub async fn revoke(&self, token_id: &str) -> Result<(), TokenRepositoryError> {
        let result = sqlx::query(
            r#"
            UPDATE tokens
            SET is_revoked = true, revoked_at = CURRENT_TIMESTAMP
            WHERE token_id = $1
            "#,
        )
        .bind(token_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(TokenRepositoryError::NotFound);
        }

        Ok(())
    }

    pub async fn cleanup_expired(&self) -> Result<u64, TokenRepositoryError> {
        let result = sqlx::query(
            "DELETE FROM tokens WHERE expires_at < CURRENT_TIMESTAMP"
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }
}
