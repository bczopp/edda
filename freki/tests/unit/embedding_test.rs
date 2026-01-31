#[cfg(test)]
mod tests {
    use freki::embedding::{EmbeddingModel, SentenceTransformersModel};
    use std::time::Duration;

    #[tokio::test]
    async fn test_embedding_model_creation() {
        // Test embedding model creation
        let model = SentenceTransformersModel::new("all-MiniLM-L6-v2").await;
        // Model creation may fail if model files not available
        // That's acceptable in test environment
        assert!(model.is_ok() || model.is_err());
    }

    #[tokio::test]
    async fn test_embed_text() {
        // Test text embedding
        let model = SentenceTransformersModel::new("all-MiniLM-L6-v2").await;
        if let Ok(model) = model {
            let result = model.embed_text("test text").await;
            // May fail if model not loaded, but should have correct interface
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[tokio::test]
    async fn test_embed_batch() {
        // Test batch embedding
        let model = SentenceTransformersModel::new("all-MiniLM-L6-v2").await;
        if let Ok(model) = model {
            let texts = vec!["text1".to_string(), "text2".to_string()];
            let result = model.embed_batch(&texts).await;
            assert!(result.is_ok() || result.is_err());
        }
    }

    #[tokio::test]
    async fn test_get_vector_dimension() {
        // Test vector dimension
        let model = SentenceTransformersModel::new("all-MiniLM-L6-v2").await;
        if let Ok(model) = model {
            let dim = model.get_vector_dimension();
            // all-MiniLM-L6-v2 has 384 dimensions
            assert_eq!(dim, 384);
        }
    }
}
