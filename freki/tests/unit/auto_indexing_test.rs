//! Tests für Auto-Indexing-Manager (Phase 8.1.2).

use freki::chunking::{ChunkingError, DocumentChunker};
use freki::embedding::{EmbeddingError, EmbeddingModel};
use freki::indexing::{
    AutoIndexingManager, DocumentIndexer, DocumentParser, FullReIndexingManager, TextParser,
};
use freki::utils::DataDeletionManager;
use freki::vector_db::VectorDbClient;
use serde_json::json;
use std::path::Path;
use std::sync::Arc;
use async_trait::async_trait;
use tempfile::TempDir;

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
async fn auto_indexing_created_indexes_file() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };
    let coll = "auto_idx_created";
    let _ = client.delete_collection(coll).await;
    client.create_collection(coll, 4).await?;

    let indexer = Arc::new(
        DocumentIndexer::new(client.clone(), coll.to_string())
            .with_chunker(Arc::new(StubChunker))
            .with_embedding_model(Arc::new(StubEmbedding)),
    );
    let parser = Arc::new(TextParser::new());
    let full_reindex = Arc::new(FullReIndexingManager::new(Arc::clone(&indexer)));
    let data_deletion = Arc::new(DataDeletionManager::new(client.clone(), coll.to_string()));
    let auto = AutoIndexingManager::new(parser, indexer, full_reindex, data_deletion);

    let dir = TempDir::new().unwrap();
    let file = dir.path().join("new.txt");
    std::fs::write(&file, "hello world").unwrap();

    auto.handle_created(file.as_path()).await?;
    let _ = client.delete_collection(coll).await;
    Ok(())
}

#[tokio::test]
async fn auto_indexing_modified_reindexes_file() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };
    let coll = "auto_idx_modified";
    let _ = client.delete_collection(coll).await;
    client.create_collection(coll, 4).await?;

    let indexer = Arc::new(
        DocumentIndexer::new(client.clone(), coll.to_string())
            .with_chunker(Arc::new(StubChunker))
            .with_embedding_model(Arc::new(StubEmbedding)),
    );
    let parser = Arc::new(TextParser::new());
    let full_reindex = Arc::new(FullReIndexingManager::new(Arc::clone(&indexer)));
    let data_deletion = Arc::new(DataDeletionManager::new(client.clone(), coll.to_string()));
    let auto = AutoIndexingManager::new(parser, indexer, full_reindex, data_deletion);

    let dir = TempDir::new().unwrap();
    let file = dir.path().join("m.txt");
    std::fs::write(&file, "first").unwrap();
    auto.handle_created(file.as_path()).await?;
    std::fs::write(&file, "second|content").unwrap();
    auto.handle_modified(file.as_path()).await?;
    let _ = client.delete_collection(coll).await;
    Ok(())
}

#[tokio::test]
async fn auto_indexing_removed_deletes_from_index() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(_) => return Ok(()),
    };
    let coll = "auto_idx_removed";
    let _ = client.delete_collection(coll).await;
    client.create_collection(coll, 4).await?;

    let indexer = Arc::new(
        DocumentIndexer::new(client.clone(), coll.to_string())
            .with_chunker(Arc::new(StubChunker))
            .with_embedding_model(Arc::new(StubEmbedding)),
    );
    let parser = Arc::new(TextParser::new());
    let full_reindex = Arc::new(FullReIndexingManager::new(Arc::clone(&indexer)));
    let data_deletion = Arc::new(DataDeletionManager::new(client.clone(), coll.to_string()));
    let auto = AutoIndexingManager::new(parser, indexer, full_reindex, data_deletion);

    let dir = TempDir::new().unwrap();
    let file = dir.path().join("r.txt");
    std::fs::write(&file, "content").unwrap();
    auto.handle_created(file.as_path()).await?;
    auto.handle_removed(file.as_path()).await?;
    let _ = client.delete_collection(coll).await;
    Ok(())
}

#[test]
fn path_to_document_id_stable() {
    let id1 = AutoIndexingManager::path_to_document_id(Path::new("/a/b.txt"));
    let id2 = AutoIndexingManager::path_to_document_id(Path::new("/a/b.txt"));
    assert_eq!(id1, id2);
    assert!(!id1.is_empty());
}
