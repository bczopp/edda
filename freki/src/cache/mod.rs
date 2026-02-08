//! Cache-Module für Freki (Phase 10: Caching).
//!
//! Erste Implementierung: In-Memory-Embedding-Cache als Referenz für weitere
//! Backends (z. B. Redis). Siehe `embedding_cache` für Details.

pub mod embedding_cache;

pub use embedding_cache::{
    EmbeddingCache, EmbeddingCacheError, EmbeddingCacheHelper, InMemoryEmbeddingCache,
};

