#[cfg(test)]
mod tests {
    use freki::embedding::{EmbeddingModel, SentenceTransformersModel};
    use freki::retrieval::QueryEmbeddingGenerator;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_query_embedding_generator_uses_same_model_dimension() {
        let model = SentenceTransformersModel::new("all-MiniLM-L6-v2").await;
        if let Ok(model) = model {
            let gen = QueryEmbeddingGenerator::new(Arc::new(model));
            assert_eq!(gen.vector_dimension(), 384);
            assert_eq!(gen.model_name(), "all-MiniLM-L6-v2");
        }
    }

    #[tokio::test]
    async fn test_query_embedding_generate_returns_vector() {
        let model = SentenceTransformersModel::new("all-MiniLM-L6-v2").await;
        if let Ok(model) = model {
            let gen = QueryEmbeddingGenerator::new(Arc::new(model));
            let result = gen.generate("test query").await;
            assert!(result.is_ok());
            let vec = result.unwrap();
            assert_eq!(vec.len(), 384);
        }
    }

    #[tokio::test]
    async fn test_query_embedding_empty_query() {
        let model = SentenceTransformersModel::new("all-MiniLM-L6-v2").await;
        if let Ok(model) = model {
            let gen = QueryEmbeddingGenerator::new(Arc::new(model));
            let result = gen.generate("").await;
            assert!(result.is_ok());
            assert_eq!(result.unwrap().len(), 384);
        }
    }
}
