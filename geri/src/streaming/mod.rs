//! Streaming-Support (Phase 12.1, 12.2): LLM-Response-Streaming, Video-Stream-Processing.

mod manager;
mod video_stream;
pub use manager::{StreamingError, StreamingManager};
pub use video_stream::{
    FrameAnalyzer, VideoAnalysisChunk, VideoStreamChunk, VideoStreamError, VideoStreamProcessor,
};
