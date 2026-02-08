// StreamingHandler trait tests (Phase 9.1.1, TDD).

use jotunheim_esp32::streaming::{StreamingError, StreamingHandler};

struct MockStreamingHandler;

#[async_trait::async_trait]
impl StreamingHandler for MockStreamingHandler {
    async fn send_video_stream(&self, _data: &[u8]) -> Result<(), StreamingError> {
        Ok(())
    }
    async fn send_audio_stream(&self, _data: &[u8]) -> Result<(), StreamingError> {
        Ok(())
    }
    async fn receive_video_stream(&self) -> Result<Vec<u8>, StreamingError> {
        Ok(vec![])
    }
    async fn receive_audio_stream(&self) -> Result<Vec<u8>, StreamingError> {
        Ok(vec![])
    }
}

#[tokio::test]
async fn streaming_handler_send_video_returns_ok() {
    let h = MockStreamingHandler;
    let r = h.send_video_stream(b"video").await;
    assert!(r.is_ok());
}

#[tokio::test]
async fn streaming_handler_receive_audio_returns_bytes() {
    let h = MockStreamingHandler;
    let r = h.receive_audio_stream().await.unwrap();
    assert!(r.is_empty());
}
