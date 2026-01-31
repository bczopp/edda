use sqlx::PgPool;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SettlementError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Settlement processing failed: {0}")]
    ProcessingFailed(String),
}

pub struct Settlement {
    pub settlement_id: String,
    pub provider_id: String,
    pub amount: f64,
    pub status: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

pub struct SettlementProcessor {
    pool: PgPool,
}

impl SettlementProcessor {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn process_settlement(
        &self,
        provider_id: &str,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Settlement, SettlementError> {
        // Calculate total amount from pricing_records for the period
        let total: Option<rust_decimal::Decimal> = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(net_price), 0) as total
            FROM pricing_records
            WHERE provider_id = $1
            AND created_at >= $2
            AND created_at < $3
            "#,
            provider_id,
            period_start,
            period_end
        )
        .fetch_one(&self.pool)
        .await?;

        let amount = total
            .unwrap_or(rust_decimal::Decimal::ZERO)
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);

        let settlement_id = Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
            INSERT INTO settlements (settlement_id, provider_id, amount, status, period_start, period_end)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            settlement_id,
            provider_id,
            rust_decimal::Decimal::from_f64_retain(amount).unwrap_or(rust_decimal::Decimal::ZERO),
            "pending",
            period_start,
            period_end
        )
        .execute(&self.pool)
        .await?;

        // Log audit event
        sqlx::query!(
            r#"
            INSERT INTO audit_logs (event_type, provider_id, settlement_id, details)
            VALUES ($1, $2, $3, $4)
            "#,
            "settlement_processed",
            provider_id,
            settlement_id,
            serde_json::json!({"amount": amount, "period_start": period_start, "period_end": period_end})
        )
        .execute(&self.pool)
        .await?;

        Ok(Settlement {
            settlement_id,
            provider_id: provider_id.to_string(),
            amount,
            status: "pending".to_string(),
            period_start,
            period_end,
        })
    }
}
