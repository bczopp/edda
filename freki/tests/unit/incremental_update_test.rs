//! Tests für Incremental-Update-Manager (Phase 7.2.1).

use freki::chunking::{ChunkingError, DocumentChunker};
use freki::embedding::{EmbeddingError, EmbeddingModel};
use freki::indexing::{
    Document, DocumentChangeDetector, DocumentHash, DocumentIndexer, IncrementalUpdateManager,
    IncrementalUpdateResult,
};
use freki::vector_db::VectorDbClient;
use serde_json::json;
use std::sync::Arc;
use async_trait::async_trait;

fn doc(id: &str, content: &str) -> Document {
    Document {
        id: id.to_string(),
        content: content.to_string(),
        metadata: json!({}),
    }
}

/// Chunker stub: returns fixed chunks for testing.
struct StubChunker;
#[async_trait]
impl DocumentChunker for StubChunker {
    async fn chunk_document(&self, document: &str) -> Result<Vec<String>, ChunkingError> {
        Ok(document.split('|').map(|s| s.to_string()).collect())
    }
    fn get_chunk_size(&self) -> u64 { 100 }
    fn get_overlap_size(&self) -> u64 { 0 }
}

/// Embedding stub: deterministic vectors per chunk.
struct StubEmbedding;
#[async_trait]
impl EmbeddingModel for StubEmbedding {
    async fn embed_text(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        Ok(vec![text.len() as f32; 4])
    }
    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        Ok(texts.iter().map(|t| vec![t.len() as f32; 4]).collect())
    }
    fn get_model_name(&self) -> &str { "stub" }
    fn get_vector_dimension(&self) -> u64 { 4 }
}

#[tokio::test]
async fn incremental_update_only_changed_chunks_reindexed() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };
    let coll = "incr_update_test";
    let _ = client.delete_collection(coll).await;
    client.create_collection(coll, 4).await?;

    let indexer = Arc::new(
        DocumentIndexer::new(client.clone(), coll.to_string())
            .with_chunker(Arc::new(StubChunker))
            .with_embedding_model(Arc::new(StubEmbedding)),
    );
    let chunker = Arc::new(StubChunker);
    let embedding = Arc::new(StubEmbedding);
    let change_detector = DocumentChangeDetector;
    let manager = IncrementalUpdateManager::new(indexer.clone(), chunker, embedding, change_detector);

    let doc1 = doc("doc1", "a|b|c");
    indexer.index_document_auto(doc1.clone()).await?;

    let old_hashes: Vec<DocumentHash> = ["a", "b", "c"]
        .iter()
        .map(|s| change_detector.compute_content_hash(s))
        .collect();

    let updated_doc = doc("doc1", "a|b2|c");
    let result = manager.update_incremental(updated_doc, &old_hashes).await?;
    assert_eq!(result.updated_count, 1, "only chunk 1 should be reindexed");
    Ok(())
}

#[tokio::test]
async fn incremental_update_no_change_returns_zero() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };
    let coll = "incr_update_nochange";
    let _ = client.delete_collection(coll).await;
    client.create_collection(coll, 4).await?;

    let indexer = Arc::new(
        DocumentIndexer::new(client.clone(), coll.to_string())
            .with_chunker(Arc::new(StubChunker))
            .with_embedding_model(Arc::new(StubEmbedding)),
    );
    let change_detector = DocumentChangeDetector;
    let manager = IncrementalUpdateManager::new(
        indexer.clone(),
        Arc::new(StubChunker),
        Arc::new(StubEmbedding),
        change_detector,
    );

    let doc1 = doc("doc2", "x|y");
    indexer.index_document_auto(doc1.clone()).await?;
    let old_hashes: Vec<DocumentHash> = ["x", "y"]
        .iter()
        .map(|s| change_detector.compute_content_hash(s))
        .collect();

    let result = manager.update_incremental(doc1.clone(), &old_hashes).await?;
    assert_eq!(result.updated_count, 0);
    Ok(())
}

#[tokio::test]
async fn incremental_update_new_chunks_added() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };
    let coll = "incr_update_new";
    let _ = client.delete_collection(coll).await;
    client.create_collection(coll, 4).await?;

    let indexer = Arc::new(
        DocumentIndexer::new(client.clone(), coll.to_string())
            .with_chunker(Arc::new(StubChunker))
            .with_embedding_model(Arc::new(StubEmbedding)),
    );
    let change_detector = DocumentChangeDetector;
    let manager = IncrementalUpdateManager::new(
        indexer.clone(),
        Arc::new(StubChunker),
        Arc::new(StubEmbedding),
        change_detector,
    );

    let doc_old = doc("doc3", "one");
    indexer.index_document_auto(doc_old).await?;
    let old_hashes: Vec<DocumentHash> = ["one"].iter().map(|s| change_detector.compute_content_hash(s)).collect();

    let doc_new = doc("doc3", "one|two");
    let result = manager.update_incremental(doc_new, &old_hashes).await?;
    assert_eq!(result.updated_count, 1, "new chunk two should be indexed");
    Ok(())
}
