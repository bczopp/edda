use sqlx::PgPool;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PreAuthError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Pre-authorization failed: {0}")]
    PreAuthFailed(String),
}

pub struct PreAuth {
    pub authorization_id: String,
    pub user_id: String,
    pub amount: f64,
    pub status: String,
    pub expires_at: DateTime<Utc>,
}

pub struct PreAuthManager {
    pool: PgPool,
}

impl PreAuthManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn pre_authorize(
        &self,
        user_id: &str,
        amount: f64,
        currency: &str,
    ) -> Result<PreAuth, PreAuthError> {
        let authorization_id = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::hours(24); // 24 hour expiry

        sqlx::query!(
            r#"
            INSERT INTO pre_authorizations (authorization_id, user_id, amount, currency, status, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            authorization_id,
            user_id,
            rust_decimal::Decimal::from_f64_retain(amount).unwrap_or(rust_decimal::Decimal::ZERO),
            currency,
            "approved",
            expires_at
        )
        .execute(&self.pool)
        .await?;

        // Log audit event
        sqlx::query!(
            r#"
            INSERT INTO audit_logs (event_type, user_id, authorization_id, details)
            VALUES ($1, $2, $3, $4)
            "#,
            "preauth_created",
            user_id,
            authorization_id,
            serde_json::json!({"amount": amount, "currency": currency})
        )
        .execute(&self.pool)
        .await?;

        Ok(PreAuth {
            authorization_id,
            user_id: user_id.to_string(),
            amount,
            status: "approved".to_string(),
            expires_at,
        })
    }

    pub async fn check_authorization(&self, authorization_id: &str) -> Result<bool, PreAuthError> {
        let auth = sqlx::query!(
            r#"
            SELECT status, expires_at FROM pre_authorizations
            WHERE authorization_id = $1
            "#,
            authorization_id
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(auth) = auth {
            if auth.status == "approved" && auth.expires_at > Utc::now() {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
