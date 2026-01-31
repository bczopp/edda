use sqlx::PgPool;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct QualityMetric {
    pub metric_id: String,
    pub provider_id: String,
    pub metric_name: String,
    pub value: f64,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Error)]
pub enum MetricsError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Metrics tracking failed: {0}")]
    TrackingFailed(String),
}

pub struct MetricsTracker {
    pool: PgPool,
}

impl MetricsTracker {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn track_metric(
        &self,
        provider_id: &str,
        metric_name: &str,
        value: f64,
    ) -> Result<(), MetricsError> {
        let metric_id = Uuid::new_v4().to_string();

        sqlx::query!(
            r#"
            INSERT INTO quality_metrics (metric_id, provider_id, metric_name, value)
            VALUES ($1, $2, $3, $4)
            "#,
            metric_id,
            provider_id,
            metric_name,
            rust_decimal::Decimal::from_f64_retain(value).unwrap_or(rust_decimal::Decimal::ZERO)
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_metrics(
        &self,
        provider_id: &str,
        limit: i32,
    ) -> Result<Vec<QualityMetric>, MetricsError> {
        let limit = if limit > 0 { limit } else { 100 };
        
        let rows = sqlx::query!(
            r#"
            SELECT metric_id, provider_id, metric_name, value, timestamp
            FROM quality_metrics
            WHERE provider_id = $1
            ORDER BY timestamp DESC
            LIMIT $2
            "#,
            provider_id,
            limit
        )
        .fetch_all(&self.pool)
        .await?;

        let metrics: Vec<QualityMetric> = rows.into_iter().map(|row| {
            let value_f64 = row.value.to_string().parse::<f64>().unwrap_or(0.0);
            QualityMetric {
                metric_id: row.metric_id,
                provider_id: row.provider_id,
                metric_name: row.metric_name,
                value: value_f64,
                timestamp: row.timestamp,
            }
        }).collect();

        Ok(metrics)
    }
}
