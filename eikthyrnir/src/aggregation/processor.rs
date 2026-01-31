use sqlx::PgPool;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use thiserror::Error;
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum AggregationError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Aggregation failed: {0}")]
    AggregationFailed(String),
}

pub struct QualityAggregation {
    pub aggregation_id: String,
    pub provider_id: String,
    pub aggregated_score: f64,
    pub aggregated_metrics: HashMap<String, f64>,
}

pub struct QualityAggregator {
    pool: PgPool,
}

impl QualityAggregator {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn aggregate(
        &self,
        provider_id: &str,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<QualityAggregation, AggregationError> {
        // Get all assessments for the period
        let assessments = sqlx::query!(
            r#"
            SELECT quality_score, response_time, accuracy, availability, custom_metrics
            FROM quality_assessments
            WHERE provider_id = $1
            AND created_at >= $2
            AND created_at < $3
            "#,
            provider_id,
            period_start,
            period_end
        )
        .fetch_all(&self.pool)
        .await?;

        if assessments.is_empty() {
            return Err(AggregationError::AggregationFailed("No assessments found for period".to_string()));
        }

        // Calculate aggregated score (average)
        let total_score: f64 = assessments.iter()
            .map(|a| a.quality_score.to_string().parse::<f64>().unwrap_or(0.0))
            .sum();
        let aggregated_score = total_score / assessments.len() as f64;

        // Aggregate metrics
        let mut aggregated_metrics = HashMap::new();
        
        let avg_response_time: f64 = assessments.iter()
            .map(|a| a.response_time.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)).unwrap_or(0.0))
            .sum::<f64>() / assessments.len() as f64;
        
        let avg_accuracy: f64 = assessments.iter()
            .map(|a| a.accuracy.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)).unwrap_or(0.0))
            .sum::<f64>() / assessments.len() as f64;
        
        let avg_availability: f64 = assessments.iter()
            .map(|a| a.availability.map(|v| v.to_string().parse::<f64>().unwrap_or(0.0)).unwrap_or(0.0))
            .sum::<f64>() / assessments.len() as f64;

        aggregated_metrics.insert("response_time".to_string(), avg_response_time);
        aggregated_metrics.insert("accuracy".to_string(), avg_accuracy);
        aggregated_metrics.insert("availability".to_string(), avg_availability);

        // Aggregate custom metrics
        for assessment in &assessments {
            if let Some(custom) = &assessment.custom_metrics {
                if let Some(obj) = custom.as_object() {
                    for (key, value) in obj {
                        if let Some(num) = value.as_f64() {
                            *aggregated_metrics.entry(key.clone()).or_insert(0.0) += num;
                        }
                    }
                }
            }
        }

        // Average custom metrics
        for (key, value) in aggregated_metrics.iter_mut() {
            if !["response_time", "accuracy", "availability"].contains(&key.as_str()) {
                *value /= assessments.len() as f64;
            }
        }

        let aggregation_id = Uuid::new_v4().to_string();

        // Store aggregation
        sqlx::query!(
            r#"
            INSERT INTO quality_aggregations (aggregation_id, provider_id, period_start, period_end, aggregated_score, aggregated_metrics)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            aggregation_id,
            provider_id,
            period_start,
            period_end,
            rust_decimal::Decimal::from_f64_retain(aggregated_score).unwrap_or(rust_decimal::Decimal::ZERO),
            serde_json::json!(aggregated_metrics)
        )
        .execute(&self.pool)
        .await?;

        // Log audit event
        sqlx::query!(
            r#"
            INSERT INTO audit_logs (event_type, provider_id, aggregation_id, details)
            VALUES ($1, $2, $3, $4)
            "#,
            "quality_aggregated",
            provider_id,
            aggregation_id,
            serde_json::json!({"aggregated_score": aggregated_score, "period_start": period_start, "period_end": period_end})
        )
        .execute(&self.pool)
        .await?;

        Ok(QualityAggregation {
            aggregation_id,
            provider_id: provider_id.to_string(),
            aggregated_score,
            aggregated_metrics,
        })
    }
}
