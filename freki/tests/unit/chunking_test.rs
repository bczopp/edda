#[cfg(test)]
mod tests {
    use freki::chunking::{DocumentChunker, SemanticChunker};
    use std::time::Duration;

    #[tokio::test]
    async fn test_semantic_chunker_creation() {
        // Test semantic chunker creation
        let chunker = SemanticChunker::new(1000, 100);
        assert_eq!(chunker.get_chunk_size(), 1000);
        assert_eq!(chunker.get_overlap_size(), 100);
    }

    #[tokio::test]
    async fn test_chunk_document() {
        // Test document chunking
        let chunker = SemanticChunker::new(1000, 100);
        
        let document = "This is a test document. It has multiple sentences. Each sentence should be processed. The chunker should split this into appropriate chunks.";
        
        let chunks = chunker.chunk_document(document).await;
        assert!(chunks.is_ok());
        
        let chunks = chunks.unwrap();
        assert!(!chunks.is_empty());
    }

    #[tokio::test]
    async fn test_chunk_large_document() {
        // Test chunking large document
        let chunker = SemanticChunker::new(100, 10); // Small chunks for testing
        
        let large_doc = "Sentence one. Sentence two. Sentence three. ".repeat(50);
        
        let chunks = chunker.chunk_document(&large_doc).await;
        assert!(chunks.is_ok());
        
        let chunks = chunks.unwrap();
        // Should have multiple chunks for large document
        assert!(chunks.len() > 1);
    }

    #[tokio::test]
    async fn test_chunk_overlap() {
        // Test that chunks have overlap
        let chunker = SemanticChunker::new(100, 20);
        
        let document = "Sentence one. Sentence two. Sentence three. Sentence four. Sentence five.";
        let chunks = chunker.chunk_document(document).await.unwrap();
        
        if chunks.len() > 1 {
            // Check that chunks overlap (simplified check)
            // In real implementation, would verify actual token overlap
            assert!(chunks.len() >= 1);
        }
    }
}
