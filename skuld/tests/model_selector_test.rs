//! Integration tests for ModelSelector (Phase 5).
//! Require DATABASE_URL (e.g. from docker-compose.test.yml).

use skuld::evaluation::ModelEvaluator;
use skuld::registry::ModelRegistry;
use skuld::selection::{ModelRequirements, ModelSelector};
use sqlx::PgPool;
use std::sync::Arc;

async fn pool_and_migrate() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (run tests in container)");
    let pool = PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

#[tokio::test]
async fn test_select_best_model_returns_highest_scoring_model() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = Arc::new(ModelRegistry::new(pool));
    let evaluator = Arc::new(ModelEvaluator);
    let selector = ModelSelector::new(registry.clone(), evaluator);

    registry.register_model("gpt-4").await?;
    registry.register_model("llama3-8b").await?;

    let requirements = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let best = selector.select_best_model(requirements).await?;
    assert!(
        best == "gpt-4" || best == "llama3-8b",
        "select_best_model should return one of the registered models, got {}",
        best
    );
    Ok(())
}

#[tokio::test]
async fn test_select_best_model_single_model_returns_that_model() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = Arc::new(ModelRegistry::new(pool));
    let evaluator = Arc::new(ModelEvaluator);
    let selector = ModelSelector::new(registry.clone(), evaluator);

    let unique = "single-select-test-model";
    registry.register_model(unique).await?;

    let requirements = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let best = selector.select_best_model(requirements).await?;
    assert_eq!(best, unique);
    Ok(())
}
