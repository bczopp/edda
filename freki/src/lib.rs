//! Freki - RAG Service für Edda
//!
//! Freki stellt einen RAG (Retrieval Augmented Generation) Service bereit für Document-Indexing
//! und Context-Retrieval. Nutzt Qdrant als Vector-Database und unterstützt Embedding-basierte
//! Ähnlichkeitssuche.
//!
//! # Beispiel
//!
//! ```no_run
//! use freki::indexing::{Document, DocumentIndexer};
//! use freki::retrieval::ContextRetriever;
//! use freki::vector_db::VectorDbClient;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Vector-DB verbinden
//! let vector_db = VectorDbClient::new("http://localhost:6333").await?;
//! vector_db.create_collection("docs", 384).await?;
//!
//! // Dokument indizieren
//! let indexer = DocumentIndexer::new(vector_db.clone(), "docs".to_string());
//! let doc = Document {
//!     id: "doc-1".to_string(),
//!     content: "Sample content".to_string(),
//!     metadata: serde_json::json!({}),
//! };
//! indexer.index_document_auto(doc).await?;
//!
//! // Context abrufen
//! let retriever = ContextRetriever::new(vector_db, "docs".to_string());
//! let query_embedding = vec![0.1; 384];
//! let context = retriever.retrieve(query_embedding, 10).await?;
//! # Ok(())
//! # }
//! ```
//!
//! Siehe auch: [docs/API.md](docs/API.md) für gRPC-API-Dokumentation.

pub mod vector_db;
pub mod watch;
pub mod indexing;
pub mod retrieval;
pub mod embedding;
pub mod chunking;
pub mod grpc;
pub mod utils;
pub mod cache;
