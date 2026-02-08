use sqlx::PgPool;
use crate::model::{ModelInfo, ModelType};
use async_trait::async_trait;

#[async_trait]
pub trait ModelRegistryTrait: Send + Sync {
    async fn register(&self, model: ModelInfo) -> Result<(), sqlx::Error>;
    async fn unregister(&self, id: &str) -> Result<(), sqlx::Error>;
    async fn get_by_id(&self, id: &str) -> Result<Option<ModelInfo>, sqlx::Error>;
    async fn list_all(&self) -> Result<Vec<ModelInfo>, sqlx::Error>;
    async fn filter_by_type(&self, model_type: ModelType) -> Result<Vec<ModelInfo>, sqlx::Error>;
    async fn filter_by_provider(&self, provider: &str) -> Result<Vec<ModelInfo>, sqlx::Error>;
}

pub struct SqlxModelRegistry {
    pool: PgPool,
}

impl SqlxModelRegistry {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ModelRegistryTrait for SqlxModelRegistry {
    async fn register(&self, model: ModelInfo) -> Result<(), sqlx::Error> {
        let model_type_str = model.model_type.to_string();
        
        sqlx::query!(
            r#"
            INSERT INTO models (
                id, name, provider, model_type, parameter_count, 
                max_context_tokens, is_local, cost_per_token_input, cost_per_token_output
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                provider = EXCLUDED.provider,
                model_type = EXCLUDED.model_type,
                parameter_count = EXCLUDED.parameter_count,
                max_context_tokens = EXCLUDED.max_context_tokens,
                is_local = EXCLUDED.is_local,
                cost_per_token_input = EXCLUDED.cost_per_token_input,
                cost_per_token_output = EXCLUDED.cost_per_token_output,
                updated_at = NOW()
            "#,
            model.id,
            model.name,
            model.provider,
            model_type_str,
            model.parameter_count.map(|p| p as i64),
            model.context_window.map(|c| c as i32).unwrap_or(0),
            model.is_local,
            model.cost_per_token_input.unwrap_or(0.0),
            model.cost_per_token_output.unwrap_or(0.0)
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn unregister(&self, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM models WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_by_id(&self, id: &str) -> Result<Option<ModelInfo>, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, provider, model_type, parameter_count, 
                   max_context_tokens, is_local, cost_per_token_input, cost_per_token_output
            FROM models WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| ModelInfo {
            id: r.id,
            name: r.name,
            provider: r.provider,
            model_type: if r.model_type == "Vision" { ModelType::Vision } else { ModelType::Llm },
            parameter_count: r.parameter_count.map(|p| p as u64),
            hardware_requirements: None, // We'd need another col if we wanted this
            context_window: Some(r.max_context_tokens as u32),
            is_local: r.is_local,
            cost_per_token_input: Some(r.cost_per_token_input.unwrap_or(0.0)),
            cost_per_token_output: Some(r.cost_per_token_output.unwrap_or(0.0)),
        }))
    }

    async fn list_all(&self) -> Result<Vec<ModelInfo>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, provider, model_type, parameter_count, 
                   max_context_tokens, is_local, cost_per_token_input, cost_per_token_output
            FROM models
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| ModelInfo {
            id: r.id,
            name: r.name,
            provider: r.provider,
            model_type: if r.model_type == "Vision" { ModelType::Vision } else { ModelType::Llm },
            parameter_count: r.parameter_count.map(|p| p as u64),
            hardware_requirements: None,
            context_window: Some(r.max_context_tokens as u32),
            is_local: r.is_local,
            cost_per_token_input: Some(r.cost_per_token_input.unwrap_or(0.0)),
            cost_per_token_output: Some(r.cost_per_token_output.unwrap_or(0.0)),
        }).collect())
    }

    async fn filter_by_type(&self, model_type: ModelType) -> Result<Vec<ModelInfo>, sqlx::Error> {
        let model_type_str = model_type.to_string();
        let rows = sqlx::query!(
            r#"
            SELECT id, name, provider, model_type, parameter_count, 
                   max_context_tokens, is_local, cost_per_token_input, cost_per_token_output
            FROM models WHERE model_type = $1
            "#,
            model_type_str
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| ModelInfo {
            id: r.id,
            name: r.name,
            provider: r.provider,
            model_type: if r.model_type == "Vision" { ModelType::Vision } else { ModelType::Llm },
            parameter_count: r.parameter_count.map(|p| p as u64),
            hardware_requirements: None,
            context_window: Some(r.max_context_tokens as u32),
            is_local: r.is_local,
            cost_per_token_input: Some(r.cost_per_token_input.unwrap_or(0.0)),
            cost_per_token_output: Some(r.cost_per_token_output.unwrap_or(0.0)),
        }).collect())
    }

    async fn filter_by_provider(&self, provider: &str) -> Result<Vec<ModelInfo>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, provider, model_type, parameter_count, 
                   max_context_tokens, is_local, cost_per_token_input, cost_per_token_output
            FROM models WHERE provider = $1
            "#,
            provider
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| ModelInfo {
            id: r.id,
            name: r.name,
            provider: r.provider,
            model_type: if r.model_type == "Vision" { ModelType::Vision } else { ModelType::Llm },
            parameter_count: r.parameter_count.map(|p| p as u64),
            hardware_requirements: None,
            context_window: Some(r.max_context_tokens as u32),
            is_local: r.is_local,
            cost_per_token_input: Some(r.cost_per_token_input.unwrap_or(0.0)),
            cost_per_token_output: Some(r.cost_per_token_output.unwrap_or(0.0)),
        }).collect())
    }
}
