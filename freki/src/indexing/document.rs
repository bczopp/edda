use crate::vector_db::VectorDbClient;
use crate::chunking::DocumentChunker;
use crate::embedding::EmbeddingModel;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Ein Dokument mit ID, Inhalt und Metadaten für Indexierung und Retrieval.
///
/// # Beispiel
///
/// ```no_run
/// use freki::indexing::Document;
/// use serde_json::json;
///
/// let doc = Document {
///     id: "doc-1".to_string(),
///     content: "Sample content".to_string(),
///     metadata: json!({ "source": "example" }),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    /// Eindeutige Dokument-ID (wird für Chunks zu "{id}-chunk-{index}").
    pub id: String,
    /// Dokumentinhalt (Text).
    pub content: String,
    /// Optionale Metadaten (JSON-Objekt).
    pub metadata: serde_json::Value,
}

/// Indiziert Dokumente in der Vector-Database mit optionalem Chunking und Embedding.
///
/// # Beispiel
///
/// ```no_run
/// # use freki::indexing::{Document, DocumentIndexer};
/// # use freki::vector_db::VectorDbClient;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let vector_db = VectorDbClient::new("http://localhost:6333").await?;
/// let indexer = DocumentIndexer::new(vector_db, "my_collection".to_string())
///     .with_chunker(/* chunker */)
///     .with_embedding_model(/* model */);
///
/// let doc = Document { id: "doc-1".to_string(), content: "text".to_string(), metadata: serde_json::json!({}) };
/// indexer.index_document_auto(doc).await?;
/// # Ok(())
/// # }
/// ```
pub struct DocumentIndexer {
    vector_db: VectorDbClient,
    collection_name: String,
    chunker: Option<Arc<dyn DocumentChunker>>,
    embedding_model: Option<Arc<dyn EmbeddingModel>>,
}

impl DocumentIndexer {
    /// Erstellt einen neuen DocumentIndexer für die angegebene Collection.
    ///
    /// # Argumente
    ///
    /// * `vector_db` - Vector-Database-Client (z. B. Qdrant)
    /// * `collection_name` - Name der Collection in der Vector-DB
    pub fn new(vector_db: VectorDbClient, collection_name: String) -> Self {
        Self {
            vector_db,
            collection_name,
            chunker: None,
            embedding_model: None,
        }
    }

    /// Konfiguriert einen Chunker für automatisches Chunking.
    ///
    /// # Beispiel
    ///
    /// ```no_run
    /// # use freki::indexing::DocumentIndexer;
    /// # use freki::chunking::SemanticChunker;
    /// # use std::sync::Arc;
    /// # let indexer = DocumentIndexer::new(/* ... */);
    /// let chunker = Arc::new(SemanticChunker::new(512, 64));
    /// let indexer = indexer.with_chunker(chunker);
    /// ```
    pub fn with_chunker(mut self, chunker: Arc<dyn DocumentChunker>) -> Self {
        self.chunker = Some(chunker);
        self
    }

    /// Konfiguriert ein Embedding-Model für automatische Embedding-Generierung.
    ///
    /// # Beispiel
    ///
    /// ```no_run
    /// # use freki::indexing::DocumentIndexer;
    /// # use freki::embedding::SentenceTransformersModel;
    /// # use std::sync::Arc;
    /// # let indexer = DocumentIndexer::new(/* ... */);
    /// let model = Arc::new(SentenceTransformersModel::new("all-MiniLM-L6-v2").await?);
    /// let indexer = indexer.with_embedding_model(model);
    /// ```
    pub fn with_embedding_model(mut self, model: Arc<dyn EmbeddingModel>) -> Self {
        self.embedding_model = Some(model);
        self
    }

    pub fn chunker(&self) -> Option<&Arc<dyn DocumentChunker>> {
        self.chunker.as_ref()
    }

    pub fn embedding_model(&self) -> Option<&Arc<dyn EmbeddingModel>> {
        self.embedding_model.as_ref()
    }

    /// Indiziert ein Dokument mit automatischem Chunking und Embedding.
    ///
    /// Chunkt das Dokument (falls Chunker konfiguriert), generiert Embeddings (falls Model konfiguriert)
    /// und indiziert jeden Chunk in der Vector-Database.
    ///
    /// # Fehler
    ///
    /// Gibt einen Fehler zurück, wenn kein Embedding-Model konfiguriert ist.
    ///
    /// # Beispiel
    ///
    /// ```no_run
    /// # use freki::indexing::{Document, DocumentIndexer};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let indexer = DocumentIndexer::new(/* ... */).with_embedding_model(/* ... */);
    /// let doc = Document {
    ///     id: "doc-1".to_string(),
    ///     content: "Long document text...".to_string(),
    ///     metadata: serde_json::json!({}),
    /// };
    /// indexer.index_document_auto(doc).await?;
    /// # Ok(())
    /// # }
    /// ```
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

    /// Indiziert ein Dokument mit bereitgestelltem Embedding.
    ///
    /// Nutzt das übergebene Embedding direkt (kein automatisches Chunking/Embedding).
    /// Payload enthält `metadata` + `"content"` (Chunk-Text) für ContextRetriever.
    ///
    /// # Argumente
    ///
    /// * `document` - Das zu indizierende Dokument
    /// * `embedding` - Vector-Embedding (Dimension muss mit Collection übereinstimmen)
    ///
    /// # Beispiel
    ///
    /// ```no_run
    /// # use freki::indexing::{Document, DocumentIndexer};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// # let indexer = DocumentIndexer::new(/* ... */);
    /// let doc = Document { id: "chunk-0".to_string(), content: "chunk text".to_string(), metadata: serde_json::json!({}) };
    /// let embedding = vec![0.1; 384]; // 384-Dimension
    /// indexer.index_document(doc, embedding).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn index_document(&self, document: Document, embedding: Vec<f32>) -> Result<(), Box<dyn std::error::Error>> {
        use qdrant_client::qdrant::*;
        use std::collections::HashMap;

        let mut payload: HashMap<String, serde_json::Value> = document
            .metadata
            .as_object()
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();
        payload.insert("content".to_string(), serde_json::Value::String(document.content));
        let base_doc_id = document
            .id
            .rsplitn(2, "-chunk-")
            .nth(1)
            .unwrap_or(document.id.as_str());
        payload.insert(
            "document_id".to_string(),
            serde_json::Value::String(base_doc_id.to_string()),
        );

        let point_id = Uuid::parse_str(&document.id)
            .unwrap_or_else(|_| Uuid::new_v5(&Uuid::NAMESPACE_OID, document.id.as_bytes()));
        let point = PointStruct::new(point_id, embedding, payload.into());

        self.vector_db.upsert_points(&self.collection_name, vec![point]).await?;
        Ok(())
    }

    /// Entfernt alle Chunks eines Dokuments aus der Vector-DB (payload.document_id == document_id).
    /// Für Full-Re-Indexing: zuerst aufrufen, dann index_document_auto.
    pub async fn delete_document_chunks(
        &self,
        document_id: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let point_ids = self
            .vector_db
            .scroll_points_by_document_id(&self.collection_name, document_id)
            .await?;
        if point_ids.is_empty() {
            return Ok(());
        }
        self.vector_db
            .delete_points(&self.collection_name, &point_ids)
            .await?;
        Ok(())
    }
}
