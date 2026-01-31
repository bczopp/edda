use sqlx::PgPool;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub struct ModelRegistry {
    pool: PgPool,
}

impl ModelRegistry {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn register_model(&self, model_id: &str, model_type: &str, framework: &str) -> Result<(), RegistryError> {
        sqlx::query!(
            r#"
            INSERT INTO models (id, model_type, framework, status)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (id) DO UPDATE
            SET model_type = EXCLUDED.model_type, framework = EXCLUDED.framework, updated_at = CURRENT_TIMESTAMP
            "#,
            model_id,
            model_type,
            framework,
            "registered"
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_model(&self, model_id: &str) -> Result<String, RegistryError> {
        let row = sqlx::query!(
            "SELECT model_type FROM models WHERE id = $1",
            model_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(row.model_type)
    }
}
