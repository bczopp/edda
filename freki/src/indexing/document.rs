use crate::vector_db::VectorDbClient;
use crate::chunking::DocumentChunker;
use crate::embedding::EmbeddingModel;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub content: String,
    pub metadata: serde_json::Value,
}

pub struct DocumentIndexer {
    vector_db: VectorDbClient,
    collection_name: String,
    chunker: Option<Arc<dyn DocumentChunker>>,
    embedding_model: Option<Arc<dyn EmbeddingModel>>,
}

impl DocumentIndexer {
    pub fn new(vector_db: VectorDbClient, collection_name: String) -> Self {
        Self {
            vector_db,
            collection_name,
            chunker: None,
            embedding_model: None,
        }
    }

    pub fn with_chunker(mut self, chunker: Arc<dyn DocumentChunker>) -> Self {
        self.chunker = Some(chunker);
        self
    }

    pub fn with_embedding_model(mut self, model: Arc<dyn EmbeddingModel>) -> Self {
        self.embedding_model = Some(model);
        self
    }

    /// Index document with automatic chunking and embedding
    pub async fn index_document_auto(
        &self,
        document: Document,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Chunk document if chunker available
        let chunks = if let Some(ref chunker) = self.chunker {
            chunker.chunk_document(&document.content).await?
        } else {
            vec![document.content.clone()]
        };

        // Generate embeddings if model available
        let embeddings = if let Some(ref model) = self.embedding_model {
            let chunk_strings: Vec<String> = chunks.iter().cloned().collect();
            model.embed_batch(&chunk_strings).await?
        } else {
            // Return error if embedding required but no model
            return Err("Embedding model required for automatic indexing".into());
        };

        // Index each chunk
        for (i, (chunk, embedding)) in chunks.iter().zip(embeddings.iter()).enumerate() {
            let chunk_doc = Document {
                id: format!("{}-chunk-{}", document.id, i),
                content: chunk.clone(),
                metadata: document.metadata.clone(),
            };
            self.index_document(chunk_doc, embedding.clone()).await?;
        }

        Ok(())
    }

    /// Index document with provided embedding (original method)
    pub async fn index_document(&self, document: Document, embedding: Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::*;
        
        let point = PointStruct::new(
            Uuid::parse_str(&document.id)?,
            embedding,
            serde_json::to_value(&document.metadata)?.as_object().unwrap().clone().into(),
        );

        self.vector_db.upsert_points(&self.collection_name, vec![point]).await?;
        Ok(())
    }
}
