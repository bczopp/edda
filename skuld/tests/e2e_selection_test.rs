//! E2E-Tests (Phase 9): Request → Evaluation → Selection → Response.
//! Vollständiger Ablauf: Registry (Modelle) → Evaluator (Scores) → Selector (beste Wahl) → Response (Modell-ID).
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

/// E2E: Request (Requirements) → Registry list_models → Evaluator evaluate (parallel) → Selector best → Response.
#[tokio::test]
async fn e2e_request_evaluation_selection_response() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = Arc::new(ModelRegistry::new(pool));
    let evaluator = Arc::new(ModelEvaluator);
    let selector = ModelSelector::new(registry.clone(), evaluator);

    registry.register_model("gpt-4").await?;
    registry.register_model("llama3-8b").await?;
    registry.register_model("llama3-70b").await?;

    let request = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let response = selector.select_best_model(request).await?;

    assert!(!response.is_empty(), "response must be non-empty model id");
    let models = registry.list_models().await?;
    assert!(
        models.contains(&response),
        "response model {} must be one of registered models {:?}",
        response,
        models
    );
    Ok(())
}

/// E2E mit Cache: Zwei identische Requests → zweiter liefert gecachtes Ergebnis (gleiche Modell-ID).
#[tokio::test]
async fn e2e_cached_selection_same_response() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = Arc::new(ModelRegistry::new(pool));
    let evaluator = Arc::new(ModelEvaluator);
    let selector = Arc::new(ModelSelector::new(registry.clone(), evaluator));
    let cache = ModelSelectionCache::new(selector, None);

    registry.register_model("e2e-model-a").await?;
    registry.register_model("e2e-model-b").await?;

    let request = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let response1 = cache.select_best_model_cached(request).await?;
    let request2 = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let response2 = cache.select_best_model_cached(request2).await?;

    assert_eq!(response1, response2, "cached second request must return same model");
    assert!(
        response1 == "e2e-model-a" || response1 == "e2e-model-b",
        "response must be one of registered models, got {}",
        response1
    );
    Ok(())
}
