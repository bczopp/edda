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

    pub async fn list_models(&self) -> Result<Vec<String>, RegistryError> {
        // Query database for registered models
        let models = sqlx::query!(
            "SELECT model_name FROM model_registry WHERE is_active = true"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(models.into_iter().map(|r| r.model_name).collect())
    }

    pub async fn register_model(&self, model_name: &str) -> Result<(), RegistryError> {
        // Insert model into database
        sqlx::query!(
            r#"
            INSERT INTO model_registry (model_name, is_active, registered_at)
            VALUES ($1, true, CURRENT_TIMESTAMP)
            ON CONFLICT (model_name) DO UPDATE
            SET is_active = true, registered_at = CURRENT_TIMESTAMP
            "#,
            model_name
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_model_info(&self, model_name: &str) -> Result<Option<ModelInfo>, RegistryError> {
        let model = sqlx::query_as!(
            ModelInfo,
            "SELECT model_name, model_type, provider, is_active FROM model_registry WHERE model_name = $1",
            model_name
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(model)
    }
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub model_name: String,
    pub model_type: Option<String>,
    pub provider: Option<String>,
    pub is_active: bool,
}
}
