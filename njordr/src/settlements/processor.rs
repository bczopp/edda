use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;
use crate::earnings::manager::EarningsManager;

#[derive(Debug, Error)]
pub enum SettlementError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Settlement processing failed: {0}")]
    ProcessingFailed(String),
    #[error("Settlement not found: {0}")]
    NotFound(String),
    #[error("Settlement already executed")]
    AlreadyExecuted,
}

#[derive(Debug, Clone)]
pub struct Settlement {
    pub settlement_id: String,
    pub provider_id: String,
    pub amount: f64,
    pub status: String,
    pub payment_method: Option<String>,
    pub transaction_id: Option<String>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

pub struct SettlementProcessor {
    pool: PgPool,
    earnings_manager: EarningsManager,
}

impl SettlementProcessor {
    pub fn new(pool: PgPool, earnings_manager: EarningsManager) -> Self {
        Self { pool, earnings_manager }
    }

    pub async fn generate_settlement(
        &self,
        provider_id: &str,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Settlement, SettlementError> {
        // Calculate earnings for the period
        let earnings = self.earnings_manager
            .calculate_earnings(provider_id, period_start, period_end)
            .await
            .map_err(|e| SettlementError::ProcessingFailed(e.to_string()))?;

        let settlement_id = Uuid::new_v4().to_string();
        let amount = earnings.net_earnings;

        sqlx::query!(
            r#"
            INSERT INTO settlements (settlement_id, provider_id, period_start, period_end, amount, status)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            settlement_id,
            provider_id,
            period_start,
            period_end,
            rust_decimal::Decimal::from_f64_retain(amount).unwrap_or(rust_decimal::Decimal::ZERO),
            "pending"
        )
        .execute(&self.pool)
        .await?;

        // Log audit event
        sqlx::query!(
            r#"
            INSERT INTO audit_logs (event_type, provider_id, settlement_id, details)
            VALUES ($1, $2, $3, $4)
            "#,
            "settlement_generated",
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
            payment_method: None,
            transaction_id: None,
            period_start,
            period_end,
        })
    }

    pub async fn execute_settlement(
        &self,
        settlement_id: &str,
        payment_method: &str,
    ) -> Result<String, SettlementError> {
        // Get settlement
        let settlement = sqlx::query!(
            r#"
            SELECT settlement_id, provider_id, amount, status, payment_method, transaction_id
            FROM settlements
            WHERE settlement_id = $1
            "#,
            settlement_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| SettlementError::NotFound(settlement_id.to_string()))?;

        if settlement.status == "completed" {
            return Err(SettlementError::AlreadyExecuted);
        }

        // Process payment (this would call PaymentGateway in real implementation)
        let transaction_id = Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
            UPDATE settlements
            SET status = $1, transaction_id = $2, payment_method = $3, executed_at = CURRENT_TIMESTAMP, updated_at = CURRENT_TIMESTAMP
            WHERE settlement_id = $4
            "#,
            "completed",
            transaction_id,
            payment_method,
            settlement_id
        )
        .execute(&self.pool)
        .await?;

        // Log audit event
        sqlx::query!(
            r#"
            INSERT INTO audit_logs (event_type, provider_id, settlement_id, details)
            VALUES ($1, $2, $3, $4)
            "#,
            "settlement_executed",
            settlement.provider_id,
            settlement_id,
            serde_json::json!({"transaction_id": transaction_id, "payment_method": payment_method})
        )
        .execute(&self.pool)
        .await?;

        Ok(transaction_id)
    }

    pub async fn track_status(&self, settlement_id: &str) -> Result<String, SettlementError> {
        let status: String = sqlx::query_scalar!(
            r#"
            SELECT status FROM settlements WHERE settlement_id = $1
            "#,
            settlement_id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| SettlementError::NotFound(settlement_id.to_string()))?;

        Ok(status)
    }
}
