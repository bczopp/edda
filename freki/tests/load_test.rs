//! Load-Tests (Phase 19.1.2): Concurrent-Queries, Batch-Indexing-Performance, Query-Volumes.
//! Erfordert QDRANT_URL (z. B. im Container via docker-compose.test.yml).

mod utils;

use freki::chunking::SemanticChunker;
use freki::indexing::{BatchIndexingManager, DocumentIndexer, IndexingManager, TextParser};
use freki::retrieval::ContextRetriever;
use freki::vector_db::VectorDbClient;
use freki::vector_db::client::point_for_test;
use std::sync::Arc;
use std::time::Instant;
use utils::document_generators::sample_documents;
use utils::embedding_generators::TestEmbeddingModel;

const COLLECTION: &str = "load_test_collection";
const VECTOR_SIZE: u64 = 384;
const CONCURRENT_QUERIES: usize = 20;
const MAX_TOTAL_MS: u128 = 5000;

#[tokio::test]
async fn load_concurrent_retrieve_requests() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Qdrant not available at {} ({}), skipping load test",
                url, e
            );
            return Ok(());
        }
    };

    let _ = client.delete_collection(COLLECTION).await;
    client.create_collection(COLLECTION, VECTOR_SIZE).await?;

    let point = point_for_test(
        "00000000-0000-0000-0000-000000000001",
        vec![0.1; VECTOR_SIZE as usize],
        "load test doc",
    );
    client.upsert_points(COLLECTION, vec![point]).await?;

    let retriever = ContextRetriever::new(client.clone(), COLLECTION.to_string());
    let query = vec![0.1; VECTOR_SIZE as usize];

    let start = Instant::now();
    let handles: Vec<_> = (0..CONCURRENT_QUERIES)
        .map(|_| {
            let retriever = retriever.clone();
            let query = query.clone();
            tokio::spawn(async move { retriever.retrieve(query, 10).await })
        })
        .collect();

    let mut results = Vec::with_capacity(handles.len());
    for h in handles {
        results.push(h.await.map_err(|e| format!("join: {}", e))??);
    }
    let elapsed = start.elapsed();

    let _ = client.delete_collection(COLLECTION).await;

    assert_eq!(results.len(), CONCURRENT_QUERIES);
    assert!(
        results.iter().all(|r| !r.documents.is_empty()),
        "all retrieves should return at least one document"
    );
    assert!(
        elapsed.as_millis() <= MAX_TOTAL_MS,
        "{} concurrent queries should complete within {}ms, took {}ms",
        CONCURRENT_QUERIES,
        MAX_TOTAL_MS,
        elapsed.as_millis()
    );
    Ok(())
}

const BATCH_COLLECTION: &str = "load_batch_collection";
const BATCH_DOC_COUNT: usize = 10;
const BATCH_SIZE: usize = 4;
const MAX_BATCH_MS: u128 = 15000;

#[tokio::test]
async fn load_batch_indexing_performance() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Qdrant not available at {} ({}), skipping batch-indexing load test",
                url, e
            );
            return Ok(());
        }
    };

    let _ = client.delete_collection(BATCH_COLLECTION).await;
    client.create_collection(BATCH_COLLECTION, VECTOR_SIZE).await?;

    let chunker = Arc::new(SemanticChunker::new(512, 64));
    let embedding = Arc::new(TestEmbeddingModel::default_dimension());
    let document_indexer = Arc::new(
        DocumentIndexer::new(client.clone(), BATCH_COLLECTION.to_string())
            .with_chunker(chunker)
            .with_embedding_model(embedding),
    );
    let indexing_manager = Arc::new(IndexingManager::new(
        Arc::new(TextParser::new()),
        document_indexer,
    ));
    let batch_manager = BatchIndexingManager::new(indexing_manager, BATCH_SIZE);

    let items: Vec<(Vec<u8>, String)> = sample_documents(BATCH_DOC_COUNT)
        .into_iter()
        .map(|d| (d.content.into_bytes(), "txt".to_string()))
        .collect();

    let start = Instant::now();
    let result = batch_manager.index_batch(items).await?;
    let elapsed = start.elapsed();

    let _ = client.delete_collection(BATCH_COLLECTION).await;

    assert_eq!(
        result.indexed, BATCH_DOC_COUNT as u32,
        "all {} documents should be indexed",
        BATCH_DOC_COUNT
    );
    assert_eq!(result.failed, 0, "no documents should fail");
    assert!(
        elapsed.as_millis() <= MAX_BATCH_MS,
        "batch indexing {} docs should complete within {}ms, took {}ms",
        BATCH_DOC_COUNT,
        MAX_BATCH_MS,
        elapsed.as_millis()
    );
    Ok(())
}
