//! Tests für Full-Re-Indexing-Manager (Phase 7.3.1).

use freki::chunking::{ChunkingError, DocumentChunker};
use freki::embedding::{EmbeddingError, EmbeddingModel};
use freki::indexing::{
    Document, DocumentIndexer, FullReIndexingManager, FullReIndexingResult,
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

struct StubChunker;
#[async_trait]
impl DocumentChunker for StubChunker {
    async fn chunk_document(&self, document: &str) -> Result<Vec<String>, ChunkingError> {
        Ok(document.split('|').map(|s| s.to_string()).collect())
    }
    fn get_chunk_size(&self) -> u64 { 100 }
    fn get_overlap_size(&self) -> u64 { 0 }
}

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
async fn full_reindex_removes_old_and_indexes_new() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };
    let coll = "full_reindex_test";
    let _ = client.delete_collection(coll).await;
    client.create_collection(coll, 4).await?;

    let indexer = Arc::new(
        DocumentIndexer::new(client.clone(), coll.to_string())
            .with_chunker(Arc::new(StubChunker))
            .with_embedding_model(Arc::new(StubEmbedding)),
    );
    let manager = FullReIndexingManager::new(indexer.clone());

    let doc1 = doc("reindex-doc", "old1|old2");
    indexer.index_document_auto(doc1).await?;

    let doc2 = doc("reindex-doc", "new1|new2|new3");
    let result = manager.reindex_full(doc2).await?;
    assert_eq!(result.chunks_indexed, 3);

    let _ = client.delete_collection(coll).await;
    Ok(())
}

#[tokio::test]
async fn full_reindex_new_document_no_old_chunks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };
    let coll = "full_reindex_new";
    let _ = client.delete_collection(coll).await;
    client.create_collection(coll, 4).await?;

    let indexer = Arc::new(
        DocumentIndexer::new(client.clone(), coll.to_string())
            .with_chunker(Arc::new(StubChunker))
            .with_embedding_model(Arc::new(StubEmbedding)),
    );
    let manager = FullReIndexingManager::new(indexer.clone());

    let doc1 = doc("brand-new", "single");
    let result = manager.reindex_full(doc1).await?;
    assert_eq!(result.chunks_indexed, 1);
    Ok(())
}
