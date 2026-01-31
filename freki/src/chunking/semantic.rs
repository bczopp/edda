use crate::chunking::sentence_boundary::SentenceBoundaryDetector;
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChunkingError {
    #[error("Chunking error: {0}")]
    ChunkingError(String),
    #[error("Token counting error: {0}")]
    TokenError(String),
}

#[async_trait]
pub trait DocumentChunker: Send + Sync {
    /// Chunk a document
    async fn chunk_document(&self, document: &str) -> Result<Vec<String>, ChunkingError>;
    
    /// Get chunk size in tokens
    fn get_chunk_size(&self) -> u64;
    
    /// Get overlap size in tokens
    fn get_overlap_size(&self) -> u64;
}

/// Semantic chunker implementation
pub struct SemanticChunker {
    chunk_size: u64,
    overlap_size: u64,
    sentence_detector: SentenceBoundaryDetector,
}

impl SemanticChunker {
    pub fn new(chunk_size: u64, overlap_size: u64) -> Self {
        Self {
            chunk_size,
            overlap_size,
            sentence_detector: SentenceBoundaryDetector::new(),
        }
    }

    /// Count tokens in text (simplified - would use tiktoken in real implementation)
    fn count_tokens(&self, text: &str) -> u64 {
        // Simplified token counting - split by whitespace
        // In real implementation, would use tiktoken
        text.split_whitespace().count() as u64
    }

    /// Create chunks with overlap
    fn create_chunks_with_overlap(&self, sentences: Vec<String>) -> Vec<String> {
        let mut chunks = Vec::new();
        let mut current_chunk = Vec::new();
        let mut current_tokens = 0;
        let mut overlap_buffer = Vec::new();

        for sentence in sentences {
            let sentence_tokens = self.count_tokens(&sentence);
            
            // Check if adding this sentence would exceed chunk size
            if current_tokens + sentence_tokens > self.chunk_size && !current_chunk.is_empty() {
                // Save current chunk
                let chunk_text = current_chunk.join(" ");
                chunks.push(chunk_text.clone());
                
                // Create overlap buffer from end of chunk
                let overlap_tokens = self.overlap_size.min(current_tokens);
                let words: Vec<&str> = chunk_text.split_whitespace().collect();
                let overlap_word_count = (overlap_tokens as usize).min(words.len());
                overlap_buffer = words[words.len().saturating_sub(overlap_word_count)..].iter()
                    .map(|s| s.to_string())
                    .collect();
                
                // Start new chunk with overlap
                current_chunk = overlap_buffer.clone();
                current_tokens = self.count_tokens(&current_chunk.join(" "));
            }
            
            current_chunk.push(sentence);
            current_tokens += sentence_tokens;
        }
        
        // Add final chunk if not empty
        if !current_chunk.is_empty() {
            chunks.push(current_chunk.join(" "));
        }
        
        chunks
    }
}

#[async_trait]
impl DocumentChunker for SemanticChunker {
    async fn chunk_document(&self, document: &str) -> Result<Vec<String>, ChunkingError> {
        // Split into sentences
        let sentences = self.sentence_detector.detect_sentences(document);
        
        if sentences.is_empty() {
            return Ok(vec![document.to_string()]);
        }
        
        // Create chunks with overlap
        let chunks = self.create_chunks_with_overlap(sentences);
        
        // Ensure no chunk exceeds max size
        let mut final_chunks = Vec::new();
        for chunk in chunks {
            let tokens = self.count_tokens(&chunk);
            if tokens <= self.chunk_size {
                final_chunks.push(chunk);
            } else {
                // Split chunk that's too large
                // Simple split by sentences
                let sub_sentences = self.sentence_detector.detect_sentences(&chunk);
                let sub_chunks = self.create_chunks_with_overlap(sub_sentences);
                final_chunks.extend(sub_chunks);
            }
        }
        
        Ok(final_chunks)
    }
    
    fn get_chunk_size(&self) -> u64 {
        self.chunk_size
    }
    
    fn get_overlap_size(&self) -> u64 {
        self.overlap_size
    }
}
