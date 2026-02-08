//! Tests für Video-Stream-Processor (Phase 12.2.1).

#[cfg(test)]
mod tests {
    use geri::streaming::{
        VideoAnalysisChunk, VideoStreamChunk, VideoStreamError, VideoStreamProcessor,
    };
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockFrameAnalyzer {
        call_count: AtomicUsize,
        result: Result<String, String>,
    }
    impl MockFrameAnalyzer {
        fn ok(analysis: &str) -> Self {
            Self {
                call_count: AtomicUsize::new(0),
                result: Ok(analysis.to_string()),
            }
        }
        fn err(msg: &str) -> Self {
            Self {
                call_count: AtomicUsize::new(0),
                result: Err(msg.to_string()),
            }
        }
    }
    impl geri::streaming::FrameAnalyzer for MockFrameAnalyzer {
        fn analyze_frame(&self, _data: &[u8]) -> Result<String, VideoStreamError> {
            self.call_count.fetch_add(1, Ordering::SeqCst);
            self.result
                .clone()
                .map_err(|s| VideoStreamError::FrameAnalysisFailed(s))
        }
    }

    #[test]
    fn push_chunk_returns_none_until_frame_complete() {
        let analyzer = MockFrameAnalyzer::ok("analysis");
        let mut proc =
            VideoStreamProcessor::new(Box::new(analyzer), 2);
        assert!(proc.push_chunk(VideoStreamChunk::new(vec![1, 2, 3])).is_none());
    }

    #[test]
    fn push_chunk_returns_analysis_when_frame_complete() {
        let analyzer = MockFrameAnalyzer::ok("frame1");
        let mut proc = VideoStreamProcessor::new(Box::new(analyzer), 2);
        proc.push_chunk(VideoStreamChunk::new(vec![1]));
        let result = proc.push_chunk(VideoStreamChunk::new(vec![2]));
        assert!(result.is_some());
        let chunk = result.unwrap().unwrap();
        assert_eq!(chunk.frame_index(), 0);
        assert_eq!(chunk.analysis(), "frame1");
    }

    #[test]
    fn push_chunk_returns_error_when_analyzer_fails() {
        let analyzer = MockFrameAnalyzer::err("vision failed");
        let mut proc = VideoStreamProcessor::new(Box::new(analyzer), 1);
        let result = proc.push_chunk(VideoStreamChunk::new(vec![1]));
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn multiple_frames_increment_frame_index() {
        let analyzer = MockFrameAnalyzer::ok("ok");
        let mut proc = VideoStreamProcessor::new(Box::new(analyzer), 1);
        let r0 = proc.push_chunk(VideoStreamChunk::new(vec![1])).unwrap().unwrap();
        let r1 = proc.push_chunk(VideoStreamChunk::new(vec![2])).unwrap().unwrap();
        assert_eq!(r0.frame_index(), 0);
        assert_eq!(r1.frame_index(), 1);
    }
}
