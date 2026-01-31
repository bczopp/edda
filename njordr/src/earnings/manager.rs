use sqlx::PgPool;
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EarningsError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Invalid period: {0}")]
    InvalidPeriod(String),
}

pub struct Earnings {
    pub total_earnings: f64,
    pub commission: f64,
    pub net_earnings: f64,
}

pub struct EarningsManager {
    pool: PgPool,
    commission_rate: f64,
}

impl EarningsManager {
    pub fn new(pool: PgPool, commission_rate: f64) -> Self {
        Self { pool, commission_rate }
    }

    pub async fn calculate_earnings(
        &self,
        provider_id: &str,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Earnings, EarningsError> {
        if period_start >= period_end {
            return Err(EarningsError::InvalidPeriod("period_start must be before period_end".to_string()));
        }

        // Query trades from database for the period
        let total_earnings: Option<rust_decimal::Decimal> = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(amount), 0) as total
            FROM trades
            WHERE provider_id = $1
            AND timestamp >= $2
            AND timestamp < $3
            AND status = 'completed'
            "#,
            provider_id,
            period_start,
            period_end
        )
        .fetch_one(&self.pool)
        .await?;

        let total_earnings_f64 = total_earnings
            .unwrap_or(rust_decimal::Decimal::ZERO)
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);

        let commission = total_earnings_f64 * self.commission_rate;
        let net_earnings = total_earnings_f64 - commission;

        // Store earnings record
        sqlx::query!(
            r#"
            INSERT INTO provider_earnings (provider_id, period_start, period_end, total_amount, commission, net_earnings)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT DO NOTHING
            "#,
            provider_id,
            period_start,
            period_end,
            rust_decimal::Decimal::from_f64_retain(total_earnings_f64).unwrap_or(rust_decimal::Decimal::ZERO),
            rust_decimal::Decimal::from_f64_retain(commission).unwrap_or(rust_decimal::Decimal::ZERO),
            rust_decimal::Decimal::from_f64_retain(net_earnings).unwrap_or(rust_decimal::Decimal::ZERO),
        )
        .execute(&self.pool)
        .await?;

        Ok(Earnings {
            total_earnings: total_earnings_f64,
            commission,
            net_earnings,
        })
    }

    pub async fn track_earnings(
        &self,
        provider_id: &str,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<(), EarningsError> {
        self.calculate_earnings(provider_id, period_start, period_end).await?;
        Ok(())
    }

    pub async fn aggregate_earnings(
        &self,
        provider_id: &str,
    ) -> Result<Earnings, EarningsError> {
        let total: Option<rust_decimal::Decimal> = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(total_amount), 0) as total
            FROM provider_earnings
            WHERE provider_id = $1
            "#,
            provider_id
        )
        .fetch_one(&self.pool)
        .await?;

        let total_earnings_f64 = total
            .unwrap_or(rust_decimal::Decimal::ZERO)
            .to_string()
            .parse::<f64>()
            .unwrap_or(0.0);

        let commission = total_earnings_f64 * self.commission_rate;
        let net_earnings = total_earnings_f64 - commission;

        Ok(Earnings {
            total_earnings: total_earnings_f64,
            commission,
            net_earnings,
        })
    }
}
