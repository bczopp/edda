use sqlx::PgPool;
use serde_json::Value;
use chrono::Utc;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Trade {
    pub trade_id: String,
    pub provider_id: String,
    pub amount: f64,
    pub timestamp: chrono::DateTime<Utc>,
    pub status: String,
    pub metadata: Option<Value>,
}

#[derive(Debug, Error)]
pub enum TradeError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Trade not found: {0}")]
    NotFound(String),
}

pub struct TradeHistoryResult {
    pub trades: Vec<Trade>,
    pub total: i32,
}

pub struct TradeManager {
    pool: PgPool,
}

impl TradeManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn track_trade(
        &self,
        trade_id: &str,
        provider_id: &str,
        amount: f64,
        metadata: Option<Value>,
    ) -> Result<(), TradeError> {
        sqlx::query!(
            r#"
            INSERT INTO trades (trade_id, provider_id, amount, status, metadata)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (trade_id) DO UPDATE
            SET amount = EXCLUDED.amount, status = EXCLUDED.status, metadata = EXCLUDED.metadata
            "#,
            trade_id,
            provider_id,
            rust_decimal::Decimal::from_f64_retain(amount).unwrap_or(rust_decimal::Decimal::ZERO),
            "completed",
            metadata
        )
        .execute(&self.pool)
        .await?;

        // Log audit event
        sqlx::query!(
            r#"
            INSERT INTO audit_logs (event_type, provider_id, trade_id, details)
            VALUES ($1, $2, $3, $4)
            "#,
            "trade_tracked",
            provider_id,
            trade_id,
            serde_json::json!({"amount": amount})
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_trade_history(
        &self,
        provider_id: &str,
        limit: i32,
        offset: i32,
    ) -> Result<TradeHistoryResult, TradeError> {
        // Get total count
        let total: i64 = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) FROM trades WHERE provider_id = $1
            "#,
            provider_id
        )
        .fetch_one(&self.pool)
        .await?;

        // Get trades
        let rows = sqlx::query!(
            r#"
            SELECT trade_id, provider_id, amount, timestamp, status, metadata
            FROM trades
            WHERE provider_id = $1
            ORDER BY timestamp DESC
            LIMIT $2 OFFSET $3
            "#,
            provider_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        let trades: Vec<Trade> = rows.into_iter().map(|row| {
            let amount_f64 = row.amount.to_string().parse::<f64>().unwrap_or(0.0);
            Trade {
                trade_id: row.trade_id,
                provider_id: row.provider_id,
                amount: amount_f64,
                timestamp: row.timestamp,
                status: row.status,
                metadata: row.metadata,
            }
        }).collect();

        Ok(TradeHistoryResult {
            trades,
            total: total as i32,
        })
    }
}
