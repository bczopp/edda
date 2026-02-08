//! Search-Performance-Test (Phase 14.2.1): Vector-Search-Latenz gegen Qdrant.
//! Erfordert QDRANT_URL (z. B. im Container via docker-compose.test.yml).

use freki::vector_db::VectorDbClient;
use std::time::Instant;

const COLLECTION: &str = "search_perf_test";
const VECTOR_SIZE: u64 = 384;
const MAX_SEARCH_LATENCY_MS: u128 = 150;

#[tokio::test]
async fn search_latency_under_threshold() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Qdrant not available at {} ({}), skipping search performance test", url, e);
            return Ok(());
        }
    };
    let _ = client.delete_collection(COLLECTION).await;
    client.create_collection(COLLECTION, VECTOR_SIZE).await?;

    let points = vec![
        freki::vector_db::client::point_for_test(
            "00000000-0000-0000-0000-000000000001",
            vec![0.1; VECTOR_SIZE as usize],
            "doc1",
        ),
        freki::vector_db::client::point_for_test(
            "00000000-0000-0000-0000-000000000002",
            vec![0.2; VECTOR_SIZE as usize],
            "doc2",
        ),
    ];
    client.upsert_points(COLLECTION, points).await?;

    let query = vec![0.15; VECTOR_SIZE as usize];
    let start = Instant::now();
    let results = client.search(COLLECTION, query, 10).await?;
    let elapsed = start.elapsed();

    let _ = client.delete_collection(COLLECTION).await;
    assert!(!results.is_empty(), "search should return at least one result");
    assert!(
        elapsed.as_millis() <= MAX_SEARCH_LATENCY_MS,
        "search should complete within {}ms, took {}ms",
        MAX_SEARCH_LATENCY_MS,
        elapsed.as_millis()
    );
    Ok(())
}
