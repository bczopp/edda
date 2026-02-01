//! Integration tests for ModelRegistry (Phase 3).
//! Require DATABASE_URL (e.g. from docker-compose.test.yml).

use skuld::registry::ModelRegistry;
use sqlx::PgPool;

async fn pool_and_migrate() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (run tests in container)");
    let pool = PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

#[tokio::test]
async fn test_register_and_list_models() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = ModelRegistry::new(pool);

    registry.register_model("gpt-4").await?;
    registry.register_model("claude-3").await?;

    let models = registry.list_models().await?;
    assert!(models.contains(&"gpt-4".to_string()));
    assert!(models.contains(&"claude-3".to_string()));

    Ok(())
}

#[tokio::test]
async fn test_register_idempotent() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = ModelRegistry::new(pool);

    registry.register_model("llama-3").await?;
    registry.register_model("llama-3").await?;

    let models = registry.list_models().await?;
    let count = models.iter().filter(|m| *m == "llama-3").count();
    assert_eq!(count, 1);

    Ok(())
}

#[tokio::test]
async fn test_get_model_info_returns_none_for_unknown() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = ModelRegistry::new(pool);

    let info = registry.get_model_info("unknown-model-xyz").await?;
    assert!(info.is_none());

    Ok(())
}

#[tokio::test]
async fn test_get_model_info_returns_some_after_register() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = ModelRegistry::new(pool);

    registry.register_model("test-model").await?;
    let info = registry.get_model_info("test-model").await?.expect("model should exist");
    assert_eq!(info.model_name, "test-model");
    assert!(info.is_active);

    Ok(())
}
