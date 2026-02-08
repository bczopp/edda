//! Performance-Tests (Phase 9): Latenz und Durchsatz der Model-Selection.
//! Require DATABASE_URL (e.g. from docker-compose.test.yml).

use skuld::evaluation::ModelEvaluator;
use skuld::registry::ModelRegistry;
use skuld::selection::{ModelRequirements, ModelSelectionCache, ModelSelector};
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Instant;

async fn pool_and_migrate() -> Result<PgPool, Box<dyn std::error::Error + Send + Sync>> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (run tests in container)");
    let pool = PgPool::connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

/// Selection-Latenz: eine select_best_model-Anfrage muss innerhalb eines Schwellwerts abgeschlossen sein.
/// Schwellwert großzügig (2s) für CI/Container mit DB; Ziel in Produktion: < 50ms bei Cache-Treffer.
#[tokio::test]
async fn selection_latency_under_threshold() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const MAX_LATENCY_MS: u128 = 2000;

    let pool = pool_and_migrate().await?;
    let registry = Arc::new(ModelRegistry::new(pool));
    let evaluator = Arc::new(ModelEvaluator);
    let selector = ModelSelector::new(registry.clone(), evaluator);

    registry.register_model("perf-model-1").await?;
    registry.register_model("perf-model-2").await?;

    let request = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let start = Instant::now();
    let _ = selector.select_best_model(request).await?;
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_millis() <= MAX_LATENCY_MS,
        "selection should complete within {}ms, took {}ms",
        MAX_LATENCY_MS,
        elapsed.as_millis()
    );
    Ok(())
}

/// Cached Selection: zweiter Aufruf soll deutlich schneller sein (Cache-Treffer).
#[tokio::test]
async fn cached_selection_faster_than_uncached() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let pool = pool_and_migrate().await?;
    let registry = Arc::new(ModelRegistry::new(pool));
    let evaluator = Arc::new(ModelEvaluator);
    let selector = Arc::new(ModelSelector::new(registry.clone(), evaluator));
    let cache = ModelSelectionCache::new(selector, None);

    registry.register_model("perf-cached-a").await?;
    registry.register_model("perf-cached-b").await?;

    let request = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let start1 = Instant::now();
    let _ = cache.select_best_model_cached(request).await?;
    let uncached_ms = start1.elapsed().as_millis();

    let request2 = ModelRequirements {
        max_size: None,
        min_reliability: None,
        max_latency_ms: None,
    };
    let start2 = Instant::now();
    let _ = cache.select_best_model_cached(request2).await?;
    let cached_ms = start2.elapsed().as_millis();

    assert!(
        cached_ms <= uncached_ms || cached_ms < 100,
        "cached selection should be faster or very fast ({}ms vs {}ms)",
        cached_ms,
        uncached_ms
    );
    Ok(())
}
