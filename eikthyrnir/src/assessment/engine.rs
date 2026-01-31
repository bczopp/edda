use sqlx::PgPool;
use serde_json::Value;
use uuid::Uuid;
use thiserror::Error;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct QualityData {
    pub response_time: f64,
    pub accuracy: f64,
    pub availability: f64,
    pub custom_metrics: HashMap<String, f64>,
}

#[derive(Debug, Clone)]
pub struct QualityAssessment {
    pub assessment_id: String,
    pub provider_id: String,
    pub service_id: String,
    pub quality_score: f64,
    pub metrics: HashMap<String, f64>,
}

#[derive(Debug, Error)]
pub enum AssessmentError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Assessment failed: {0}")]
    AssessmentFailed(String),
}

pub struct QualityAssessor {
    pool: PgPool,
}

impl QualityAssessor {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn assess_quality(
        &self,
        provider_id: &str,
        service_id: &str,
        data: QualityData,
    ) -> Result<QualityAssessment, AssessmentError> {
        // Calculate quality score from metrics
        // Weighted average: response_time (30%), accuracy (40%), availability (30%)
        let response_time_score = (1.0 - (data.response_time / 1000.0).min(1.0)).max(0.0);
        let accuracy_score = data.accuracy;
        let availability_score = data.availability;
        
        let quality_score = (response_time_score * 0.3 + accuracy_score * 0.4 + availability_score * 0.3)
            .min(1.0)
            .max(0.0);

        let assessment_id = Uuid::new_v4().to_string();

        // Store assessment
        sqlx::query!(
            r#"
            INSERT INTO quality_assessments (assessment_id, provider_id, service_id, quality_score, response_time, accuracy, availability, custom_metrics)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            assessment_id,
            provider_id,
            service_id,
            rust_decimal::Decimal::from_f64_retain(quality_score).unwrap_or(rust_decimal::Decimal::ZERO),
            rust_decimal::Decimal::from_f64_retain(data.response_time).unwrap_or(rust_decimal::Decimal::ZERO),
            rust_decimal::Decimal::from_f64_retain(data.accuracy).unwrap_or(rust_decimal::Decimal::ZERO),
            rust_decimal::Decimal::from_f64_retain(data.availability).unwrap_or(rust_decimal::Decimal::ZERO),
            serde_json::json!(data.custom_metrics)
        )
        .execute(&self.pool)
        .await?;

        // Log audit event
        sqlx::query!(
            r#"
            INSERT INTO audit_logs (event_type, provider_id, service_id, assessment_id, details)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            "quality_assessed",
            provider_id,
            service_id,
            assessment_id,
            serde_json::json!({"quality_score": quality_score})
        )
        .execute(&self.pool)
        .await?;

        let mut metrics = HashMap::new();
        metrics.insert("response_time".to_string(), data.response_time);
        metrics.insert("accuracy".to_string(), data.accuracy);
        metrics.insert("availability".to_string(), data.availability);
        metrics.extend(data.custom_metrics);

        Ok(QualityAssessment {
            assessment_id,
            provider_id: provider_id.to_string(),
            service_id: service_id.to_string(),
            quality_score,
            metrics,
        })
    }
}
