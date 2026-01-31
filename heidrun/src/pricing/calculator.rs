use sqlx::PgPool;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum PricingError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Pricing calculation failed: {0}")]
    CalculationFailed(String),
}

pub struct PriceResult {
    pub price: f64,
    pub commission: f64,
    pub net_price: f64,
}

pub struct PricingCalculator {
    pool: PgPool,
    commission_rate: f64,
}

impl PricingCalculator {
    pub fn new(pool: PgPool, commission_rate: f64) -> Self {
        Self { pool, commission_rate }
    }

    pub async fn calculate_price(
        &self,
        token_count: i64,
        model: &str,
        provider_id: &str,
    ) -> Result<PriceResult, PricingError> {
        // Get price per token from database (or use default pricing table)
        let price_per_token = self.get_price_per_token(model).await?;
        
        let base_price = price_per_token * token_count as f64;
        let commission = base_price * self.commission_rate;
        let net_price = base_price - commission;

        // Store pricing record
        let request_id = Uuid::new_v4().to_string();
        sqlx::query!(
            r#"
            INSERT INTO pricing_records (request_id, token_count, model, provider_id, base_price, commission, net_price)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            request_id,
            token_count,
            model,
            provider_id,
            rust_decimal::Decimal::from_f64_retain(base_price).unwrap_or(rust_decimal::Decimal::ZERO),
            rust_decimal::Decimal::from_f64_retain(commission).unwrap_or(rust_decimal::Decimal::ZERO),
            rust_decimal::Decimal::from_f64_retain(net_price).unwrap_or(rust_decimal::Decimal::ZERO),
        )
        .execute(&self.pool)
        .await?;

        Ok(PriceResult {
            price: base_price,
            commission,
            net_price,
        })
    }

    async fn get_price_per_token(&self, model: &str) -> Result<f64, PricingError> {
        // Default pricing table (could be loaded from database)
        let default_prices: std::collections::HashMap<&str, f64> = [
            ("llama3-8b", 0.0001),
            ("gpt-4v", 0.03),
            ("gpt-4", 0.03),
            ("gpt-3.5-turbo", 0.002),
        ]
        .iter()
        .cloned()
        .collect();

        Ok(*default_prices.get(model).unwrap_or(&0.001))
    }
}
