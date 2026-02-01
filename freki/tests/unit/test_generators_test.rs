//! Unit tests for Test-Document-Generators and Test-Embedding-Generators (Phase 1.2.2).

#[cfg(test)]
mod tests {
    use freki::embedding::EmbeddingModel;
    use std::sync::Arc;

    #[test]
    fn test_sample_document_has_id_and_content() {
        let doc = crate::utils::document_generators::sample_document();
        assert!(!doc.id.is_empty());
        assert!(!doc.content.is_empty());
        assert!(doc.metadata.get("title").is_some());
    }

    #[test]
    fn test_sample_documents_count() {
        let docs = crate::utils::document_generators::sample_documents(3);
        assert_eq!(docs.len(), 3);
        assert_eq!(docs[0].id, "test-doc-0");
        assert_eq!(docs[1].id, "test-doc-1");
    }

    #[test]
    fn test_document_with_content() {
        let doc = crate::utils::document_generators::document_with_content("my-id", "my content");
        assert_eq!(doc.id, "my-id");
        assert_eq!(doc.content, "my content");
    }

    #[tokio::test]
    async fn test_embedding_model_dimension() {
        let model = crate::utils::embedding_generators::TestEmbeddingModel::new(384);
        assert_eq!(model.get_vector_dimension(), 384);
        assert_eq!(model.get_model_name(), "test-embedding-model");
    }

    #[tokio::test]
    async fn test_embedding_model_embed_text() {
        let model = crate::utils::embedding_generators::TestEmbeddingModel::default_dimension();
        let v = model.embed_text("hello").await.unwrap();
        assert_eq!(v.len(), 384);
    }

    #[tokio::test]
    async fn test_embedding_model_embed_batch() {
        let model = crate::utils::embedding_generators::TestEmbeddingModel::new(64);
        let texts = vec!["a".to_string(), "b".to_string()];
        let batch = model.embed_batch(&texts).await.unwrap();
        assert_eq!(batch.len(), 2);
        assert_eq!(batch[0].len(), 64);
        assert_eq!(batch[1].len(), 64);
    }
}
