//! Metrics Collector for Huginn & Muninn

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;
use tracing::debug;

/// Snapshot of STT performance metrics
#[derive(Debug, Clone, Default)]
pub struct SttMetrics {
    pub request_count: u64,
    pub total_latency_ms: u64,
    pub last_latency_ms: u64,
    pub error_count: u64,
}

/// Snapshot of TTS performance metrics
#[derive(Debug, Clone, Default)]
pub struct TtsMetrics {
    pub request_count: u64,
    pub total_latency_ms: u64,
    pub last_latency_ms: u64,
    pub error_count: u64,
}

/// Snapshot of audio processing metrics
#[derive(Debug, Clone, Default)]
pub struct AudioProcessingMetrics {
    pub operation_count: u64,
    pub total_latency_ms: u64,
    pub last_latency_ms: u64,
}

/// Snapshot of video processing metrics
#[derive(Debug, Clone, Default)]
pub struct VideoProcessingMetrics {
    pub operation_count: u64,
    pub total_latency_ms: u64,
    pub last_latency_ms: u64,
}

/// Combined metrics snapshot
#[derive(Debug, Clone, Default)]
pub struct MetricsSnapshot {
    pub stt: SttMetrics,
    pub tts: TtsMetrics,
    pub audio_processing: AudioProcessingMetrics,
    pub video_processing: VideoProcessingMetrics,
}

struct MetricsCollectorInner {
    stt_count: AtomicU64,
    stt_total_latency_ms: AtomicU64,
    stt_last_latency_ms: AtomicU64,
    stt_errors: AtomicU64,
    tts_count: AtomicU64,
    tts_total_latency_ms: AtomicU64,
    tts_last_latency_ms: AtomicU64,
    tts_errors: AtomicU64,
    audio_count: AtomicU64,
    audio_total_latency_ms: AtomicU64,
    audio_last_latency_ms: AtomicU64,
    video_count: AtomicU64,
    video_total_latency_ms: AtomicU64,
    video_last_latency_ms: AtomicU64,
}

/// Metrics collector for STT, TTS, audio and video processing performance
pub struct MetricsCollector {
    inner: std::sync::Arc<MetricsCollectorInner>,
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCollector {
    /// Create a new metrics collector
    pub fn new() -> Self {
        Self {
            inner: std::sync::Arc::new(MetricsCollectorInner {
                stt_count: AtomicU64::new(0),
                stt_total_latency_ms: AtomicU64::new(0),
                stt_last_latency_ms: AtomicU64::new(0),
                stt_errors: AtomicU64::new(0),
                tts_count: AtomicU64::new(0),
                tts_total_latency_ms: AtomicU64::new(0),
                tts_last_latency_ms: AtomicU64::new(0),
                tts_errors: AtomicU64::new(0),
                audio_count: AtomicU64::new(0),
                audio_total_latency_ms: AtomicU64::new(0),
                audio_last_latency_ms: AtomicU64::new(0),
                video_count: AtomicU64::new(0),
                video_total_latency_ms: AtomicU64::new(0),
                video_last_latency_ms: AtomicU64::new(0),
            }),
        }
    }

    /// Record STT request latency
    pub fn record_stt_latency(&self, latency: Duration) {
        let ms = latency.as_millis() as u64;
        self.inner.stt_count.fetch_add(1, Ordering::Relaxed);
        self.inner.stt_total_latency_ms.fetch_add(ms, Ordering::Relaxed);
        self.inner.stt_last_latency_ms.store(ms, Ordering::Relaxed);
        debug!("STT latency: {} ms", ms);
    }

    /// Record STT error
    pub fn record_stt_error(&self) {
        self.inner.stt_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Record TTS request latency
    pub fn record_tts_latency(&self, latency: Duration) {
        let ms = latency.as_millis() as u64;
        self.inner.tts_count.fetch_add(1, Ordering::Relaxed);
        self.inner.tts_total_latency_ms.fetch_add(ms, Ordering::Relaxed);
        self.inner.tts_last_latency_ms.store(ms, Ordering::Relaxed);
        debug!("TTS latency: {} ms", ms);
    }

    /// Record TTS error
    pub fn record_tts_error(&self) {
        self.inner.tts_errors.fetch_add(1, Ordering::Relaxed);
    }

    /// Record audio processing latency
    pub fn record_audio_processing_latency(&self, latency: Duration) {
        let ms = latency.as_millis() as u64;
        self.inner.audio_count.fetch_add(1, Ordering::Relaxed);
        self.inner.audio_total_latency_ms.fetch_add(ms, Ordering::Relaxed);
        self.inner.audio_last_latency_ms.store(ms, Ordering::Relaxed);
        debug!("Audio processing latency: {} ms", ms);
    }

    /// Record video processing latency
    pub fn record_video_processing_latency(&self, latency: Duration) {
        let ms = latency.as_millis() as u64;
        self.inner.video_count.fetch_add(1, Ordering::Relaxed);
        self.inner.video_total_latency_ms.fetch_add(ms, Ordering::Relaxed);
        self.inner.video_last_latency_ms.store(ms, Ordering::Relaxed);
        debug!("Video processing latency: {} ms", ms);
    }

    /// Get current metrics snapshot
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            stt: SttMetrics {
                request_count: self.inner.stt_count.load(Ordering::Relaxed),
                total_latency_ms: self.inner.stt_total_latency_ms.load(Ordering::Relaxed),
                last_latency_ms: self.inner.stt_last_latency_ms.load(Ordering::Relaxed),
                error_count: self.inner.stt_errors.load(Ordering::Relaxed),
            },
            tts: TtsMetrics {
                request_count: self.inner.tts_count.load(Ordering::Relaxed),
                total_latency_ms: self.inner.tts_total_latency_ms.load(Ordering::Relaxed),
                last_latency_ms: self.inner.tts_last_latency_ms.load(Ordering::Relaxed),
                error_count: self.inner.tts_errors.load(Ordering::Relaxed),
            },
            audio_processing: AudioProcessingMetrics {
                operation_count: self.inner.audio_count.load(Ordering::Relaxed),
                total_latency_ms: self.inner.audio_total_latency_ms.load(Ordering::Relaxed),
                last_latency_ms: self.inner.audio_last_latency_ms.load(Ordering::Relaxed),
            },
            video_processing: VideoProcessingMetrics {
                operation_count: self.inner.video_count.load(Ordering::Relaxed),
                total_latency_ms: self.inner.video_total_latency_ms.load(Ordering::Relaxed),
                last_latency_ms: self.inner.video_last_latency_ms.load(Ordering::Relaxed),
            },
        }
    }

    /// Get average STT latency in ms (0 if no requests)
    pub fn stt_avg_latency_ms(&self) -> u64 {
        let count = self.inner.stt_count.load(Ordering::Relaxed);
        if count == 0 {
            return 0;
        }
        self.inner.stt_total_latency_ms.load(Ordering::Relaxed) / count
    }

    /// Get average TTS latency in ms (0 if no requests)
    pub fn tts_avg_latency_ms(&self) -> u64 {
        let count = self.inner.tts_count.load(Ordering::Relaxed);
        if count == 0 {
            return 0;
        }
        self.inner.tts_total_latency_ms.load(Ordering::Relaxed) / count
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.inner.stt_count.store(0, Ordering::Relaxed);
        self.inner.stt_total_latency_ms.store(0, Ordering::Relaxed);
        self.inner.stt_last_latency_ms.store(0, Ordering::Relaxed);
        self.inner.stt_errors.store(0, Ordering::Relaxed);
        self.inner.tts_count.store(0, Ordering::Relaxed);
        self.inner.tts_total_latency_ms.store(0, Ordering::Relaxed);
        self.inner.tts_last_latency_ms.store(0, Ordering::Relaxed);
        self.inner.audio_count.store(0, Ordering::Relaxed);
        self.inner.audio_total_latency_ms.store(0, Ordering::Relaxed);
        self.inner.audio_last_latency_ms.store(0, Ordering::Relaxed);
        self.inner.video_count.store(0, Ordering::Relaxed);
        self.inner.video_total_latency_ms.store(0, Ordering::Relaxed);
        self.inner.video_last_latency_ms.store(0, Ordering::Relaxed);
    }
}

impl Clone for MetricsCollector {
    fn clone(&self) -> Self {
        Self {
            inner: std::sync::Arc::clone(&self.inner),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_collector_new() {
        let collector = MetricsCollector::new();
        let snap = collector.snapshot();
        assert_eq!(snap.stt.request_count, 0);
        assert_eq!(snap.tts.request_count, 0);
    }

    #[test]
    fn test_record_stt_latency() {
        let collector = MetricsCollector::new();
        collector.record_stt_latency(Duration::from_millis(100));
        collector.record_stt_latency(Duration::from_millis(200));
        let snap = collector.snapshot();
        assert_eq!(snap.stt.request_count, 2);
        assert_eq!(snap.stt.total_latency_ms, 300);
        assert_eq!(snap.stt.last_latency_ms, 200);
        assert_eq!(collector.stt_avg_latency_ms(), 150);
    }

    #[test]
    fn test_record_tts_latency() {
        let collector = MetricsCollector::new();
        collector.record_tts_latency(Duration::from_millis(50));
        let snap = collector.snapshot();
        assert_eq!(snap.tts.request_count, 1);
        assert_eq!(snap.tts.last_latency_ms, 50);
        assert_eq!(collector.tts_avg_latency_ms(), 50);
    }

    #[test]
    fn test_record_errors() {
        let collector = MetricsCollector::new();
        collector.record_stt_error();
        collector.record_stt_error();
        collector.record_tts_error();
        let snap = collector.snapshot();
        assert_eq!(snap.stt.error_count, 2);
        assert_eq!(snap.tts.error_count, 1);
    }

    #[test]
    fn test_record_audio_video_latency() {
        let collector = MetricsCollector::new();
        collector.record_audio_processing_latency(Duration::from_millis(10));
        collector.record_video_processing_latency(Duration::from_millis(500));
        let snap = collector.snapshot();
        assert_eq!(snap.audio_processing.operation_count, 1);
        assert_eq!(snap.audio_processing.last_latency_ms, 10);
        assert_eq!(snap.video_processing.operation_count, 1);
        assert_eq!(snap.video_processing.last_latency_ms, 500);
    }

    #[test]
    fn test_reset() {
        let collector = MetricsCollector::new();
        collector.record_stt_latency(Duration::from_millis(100));
        collector.reset();
        let snap = collector.snapshot();
        assert_eq!(snap.stt.request_count, 0);
        assert_eq!(snap.stt.total_latency_ms, 0);
    }

    #[test]
    fn test_clone_shares_inner() {
        let collector = MetricsCollector::new();
        let collector2 = collector.clone();
        collector.record_stt_latency(Duration::from_millis(100));
        let snap = collector2.snapshot();
        assert_eq!(snap.stt.request_count, 1);
    }
}
