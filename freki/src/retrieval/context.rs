use crate::vector_db::VectorDbClient;
use serde::{Deserialize, Serialize};

/// Kontext mit abgerufenen Dokumenten und Relevanz-Scores.
///
/// # Beispiel
///
/// ```no_run
/// # use freki::retrieval::RetrievedContext;
/// let context = RetrievedContext {
///     documents: vec![],
///     relevance_scores: vec![],
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedContext {
    /// Abgerufene Dokumente (sortiert nach Score, absteigend).
    pub documents: Vec<RetrievedDocument>,
    /// Relevanz-Scores parallel zu `documents` (0.0-1.0, höher = relevanter).
    pub relevance_scores: Vec<f32>,
}

/// Ein abgerufenes Dokument mit Relevanz-Score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedDocument {
    /// Dokument-ID (Chunk-ID).
    pub id: String,
    /// Dokumentinhalt (Chunk-Text).
    pub content: String,
    /// Metadaten (JSON-Objekt).
    pub metadata: serde_json::Value,
    /// Ähnlichkeits-Score (0.0-1.0, höher = relevanter).
    pub score: f32,
}

/// Ruft relevante Dokumente basierend auf Query-Embedding ab (RAG-Context).
///
/// # Beispiel
///
/// ```no_run
/// # use freki::retrieval::ContextRetriever;
/// # use freki::vector_db::VectorDbClient;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let vector_db = VectorDbClient::new("http://localhost:6333").await?;
/// let retriever = ContextRetriever::new(vector_db, "my_collection".to_string());
/// let query_embedding = vec![0.2; 384];
/// let context = retriever.retrieve(query_embedding, 10).await?;
/// for (doc, score) in context.documents.iter().zip(context.relevance_scores.iter()) {
///     println!("Doc {}: {} (score: {})", doc.id, doc.content, score);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct ContextRetriever {
    vector_db: VectorDbClient,
    collection_name: String,
}

impl ContextRetriever {
    /// Erstellt einen neuen ContextRetriever für die angegebene Collection.
    ///
    /// # Argumente
    ///
    /// * `vector_db` - Vector-Database-Client (z. B. Qdrant)
    /// * `collection_name` - Name der Collection in der Vector-DB
    pub fn new(vector_db: VectorDbClient, collection_name: String) -> Self {
        Self { vector_db, collection_name }
    }

    /// Ruft relevante Dokumente basierend auf Query-Embedding ab.
    ///
    /// Führt eine Vector-Search (Cosine-Similarity) durch und gibt die Top-K Dokumente zurück.
    ///
    /// # Argumente
    ///
    /// * `query_embedding` - Query-Embedding (Dimension muss mit Collection übereinstimmen)
    /// * `limit` - Maximale Anzahl zurückzugebender Dokumente (1-1000)
    ///
    /// # Fehler
    ///
    /// Gibt einen Fehler zurück bei Vector-DB-Fehlern oder ungültigem Embedding.
    pub async fn retrieve(&self, query_embedding: Vec<f32>, limit: u64) -> Result<RetrievedContext, Box<dyn std::error::Error>> {
        let results = self.vector_db.search(&self.collection_name, query_embedding, limit).await?;
        
        let documents: Vec<RetrievedDocument> = results.into_iter().map(|point| {
            RetrievedDocument {
                id: point.id.to_string(),
                content: serde_json::from_value(point.payload.get("content").cloned().unwrap_or(serde_json::Value::String(String::new()))).unwrap_or_default(),
                metadata: point.payload,
                score: point.score,
            }
        }).collect();
        
        let scores: Vec<f32> = documents.iter().map(|d| d.score).collect();
        
        Ok(RetrievedContext {
            documents,
            relevance_scores: scores,
        })
    }
}
