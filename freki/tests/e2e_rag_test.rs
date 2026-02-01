//! E2E-Tests (Phase 19.1.1): Document-Indexing → Query → Context-Retrieval.
//! Erfordert QDRANT_URL (z. B. im Container via docker-compose.test.yml).

use freki::indexing::{Document, DocumentIndexer};
use freki::retrieval::ContextRetriever;
use freki::vector_db::VectorDbClient;
use serde_json::json;

const COLLECTION: &str = "e2e_rag_test";
const VECTOR_SIZE: u64 = 384;

#[tokio::test]
async fn e2e_index_then_retrieve_context() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("QDRANT_URL").unwrap_or_else(|_| "http://localhost:6333".to_string());
    let client = match VectorDbClient::new(&url).await {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Qdrant not available at {} ({}), skipping E2E RAG test",
                url, e
            );
            return Ok(());
        }
    };

    let _ = client.delete_collection(COLLECTION).await;
    client.create_collection(COLLECTION, VECTOR_SIZE).await?;

    let document = Document {
        id: "e2e-doc-1".to_string(),
        content: "E2E RAG workflow: index then retrieve.".to_string(),
        metadata: json!({}),
    };
    let embedding = vec![0.15; VECTOR_SIZE as usize];

    let indexer = DocumentIndexer::new(client.clone(), COLLECTION.to_string());
    indexer.index_document(document.clone(), embedding.clone()).await?;

    let retriever = ContextRetriever::new(client.clone(), COLLECTION.to_string());
    let context = retriever.retrieve(embedding, 10).await?;

    let _ = client.delete_collection(COLLECTION).await;

    assert!(!context.documents.is_empty(), "retrieve should return at least one document");
    assert_eq!(context.documents.len(), context.relevance_scores.len());
    let found = context
        .documents
        .iter()
        .any(|d| d.content.contains("E2E RAG workflow"));
    assert!(found, "retrieved content should contain indexed text");
    Ok(())
}
