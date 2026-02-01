//! Integration tests for ModelSelectionCache (Phase 8).
//! Require DATABASE_URL (e.g. from docker-compose.test.yml).

use skuld::evaluation::ModelEvaluator;
use skuld::registry::ModelRegistry;
use skuld::selection::{ModelRequirements, ModelSelectionCache, ModelSelector};
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
async fn test_cached_same_requirements_returns_same_model() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = Arc::new(ModelRegistry::new(pool));
    let evaluator = Arc::new(ModelEvaluator);
    let selector = Arc::new(ModelSelector::new(registry.clone(), evaluator));
    let cache = ModelSelectionCache::new(selector, None);

    registry.register_model("gpt-4").await?;
    registry.register_model("llama3-8b").await?;

    let requirements = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let first = cache.select_best_model_cached(requirements).await?;
    let requirements2 = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let second = cache.select_best_model_cached(requirements2).await?;
    assert_eq!(first, second, "same requirements should return same cached model");
    assert!(
        first == "gpt-4" || first == "llama3-8b",
        "result should be one of registered models, got {}",
        first
    );
    Ok(())
}

#[tokio::test]
async fn test_cache_key_different_requirements() {
    let r1 = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let r2 = ModelRequirements {
        max_size: Some(100),
        min_reliability: None,
        max_latency_ms: None,
    };
    assert_ne!(r1.cache_key(), r2.cache_key());
}

#[tokio::test]
async fn test_invalidate_all_clears_cache() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = Arc::new(ModelRegistry::new(pool));
    let evaluator = Arc::new(ModelEvaluator);
    let selector = Arc::new(ModelSelector::new(registry.clone(), evaluator));
    let cache = ModelSelectionCache::new(selector, None);

    let unique = "invalidate-cache-test-model";
    registry.register_model(unique).await?;

    let requirements = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let a = cache.select_best_model_cached(requirements).await?;
    cache.invalidate_all();
    let requirements2 = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let b = cache.select_best_model_cached(requirements2).await?;
    assert_eq!(a, b);
    assert_eq!(a, unique);
    Ok(())
}
