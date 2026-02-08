//! Video-Stream-Processor (Phase 12.2.1): Chunks verarbeiten, Frame-Extraction, Vision-Analyse, Streaming-Results.

use thiserror::Error;

/// Ein Chunk Video-Daten (Rohbytes).
#[derive(Debug, Clone)]
pub struct VideoStreamChunk {
    data: Vec<u8>,
}

impl VideoStreamChunk {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

/// Fehler beim Video-Stream-Processing (Frame-Extraction, Vision-Analyse).
#[derive(Debug, Clone, Error)]
pub enum VideoStreamError {
    #[error("Frame analysis failed: {0}")]
    FrameAnalysisFailed(String),
}

/// Analysiert einen Frame (Bilddaten) – z. B. via Vision-Model; in Tests mockbar.
pub trait FrameAnalyzer: Send + Sync {
    fn analyze_frame(&self, data: &[u8]) -> Result<String, VideoStreamError>;
}

/// Ein analysierter Frame (Streaming-Result).
#[derive(Debug, Clone)]
pub struct VideoAnalysisChunk {
    frame_index: u32,
    analysis: String,
}

impl VideoAnalysisChunk {
    pub fn new(frame_index: u32, analysis: String) -> Self {
        Self {
            frame_index,
            analysis,
        }
    }
    pub fn frame_index(&self) -> u32 {
        self.frame_index
    }
    pub fn analysis(&self) -> &str {
        &self.analysis
    }
}

/// Verarbeitet Video-Stream-Chunks, extrahiert Frames (N Chunks = 1 Frame), nutzt FrameAnalyzer für Analyse, liefert Streaming-Results.
pub struct VideoStreamProcessor {
    frame_analyzer: Box<dyn FrameAnalyzer>,
    chunks_per_frame: usize,
    buffer: Vec<Vec<u8>>,
    frame_index: u32,
}

impl std::fmt::Debug for VideoStreamProcessor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VideoStreamProcessor")
            .field("chunks_per_frame", &self.chunks_per_frame)
            .field("frame_index", &self.frame_index)
            .finish_non_exhaustive()
    }
}

impl VideoStreamProcessor {
    /// Erstellt einen Processor mit FrameAnalyzer und Chunks pro Frame (z. B. 10 Chunks = 1 Frame).
    pub fn new(frame_analyzer: Box<dyn FrameAnalyzer>, chunks_per_frame: usize) -> Self {
        Self {
            frame_analyzer,
            chunks_per_frame: chunks_per_frame.max(1),
            buffer: Vec::new(),
            frame_index: 0,
        }
    }

    /// Verarbeitet einen Chunk; gibt bei vollständigem Frame ein Analysis-Chunk oder Fehler zurück.
    pub fn push_chunk(
        &mut self,
        chunk: VideoStreamChunk,
    ) -> Option<Result<VideoAnalysisChunk, VideoStreamError>> {
        self.buffer.push(chunk.data().to_vec());
        if self.buffer.len() < self.chunks_per_frame {
            return None;
        }
        let frame_data: Vec<u8> = self.buffer.drain(..).flatten().collect();
        let idx = self.frame_index;
        self.frame_index += 1;
        match self.frame_analyzer.analyze_frame(&frame_data) {
            Ok(analysis) => Some(Ok(VideoAnalysisChunk::new(idx, analysis))),
            Err(e) => Some(Err(e)),
        }
    }
}
