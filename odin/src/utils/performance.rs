use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};
use std::collections::VecDeque;
use tracing::{info, warn};

/// Request queue for managing incoming requests
pub struct RequestQueue {
    queue: Arc<RwLock<VecDeque<QueuedRequest>>>,
    max_size: usize,
}

#[derive(Debug, Clone)]
pub struct QueuedRequest {
    pub request_id: String,
    pub user_id: String,
    pub device_id: String,
    pub input: String,
    pub input_type: String,
    pub queued_at: Instant,
}

impl RequestQueue {
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: Arc::new(RwLock::new(VecDeque::new())),
            max_size,
        }
    }

    pub async fn enqueue(&self, request: QueuedRequest) -> Result<(), String> {
        let mut queue = self.queue.write().await;
        if queue.len() >= self.max_size {
            return Err("Request queue is full".to_string());
        }
        queue.push_back(request);
        Ok(())
    }

    pub async fn dequeue(&self) -> Option<QueuedRequest> {
        let mut queue = self.queue.write().await;
        queue.pop_front()
    }

    pub async fn size(&self) -> usize {
        let queue = self.queue.read().await;
        queue.len()
    }

    pub async fn clear(&self) {
        let mut queue = self.queue.write().await;
        queue.clear();
    }
}

/// Performance metrics for Odin
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub avg_processing_time: Duration,
    pub total_processing_time: Duration,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            total_requests: 0,
            successful_requests: 0,
            failed_requests: 0,
            avg_processing_time: Duration::ZERO,
            total_processing_time: Duration::ZERO,
        }
    }
}

/// Performance monitor for Odin
pub struct PerformanceMonitor {
    metrics: Arc<RwLock<PerformanceMetrics>>,
    request_queue: Arc<RequestQueue>,
}

impl PerformanceMonitor {
    pub fn new(queue_size: usize) -> Self {
        Self {
            metrics: Arc::new(RwLock::new(PerformanceMetrics::new())),
            request_queue: Arc::new(RequestQueue::new(queue_size)),
        }
    }

    pub fn request_queue(&self) -> Arc<RequestQueue> {
        self.request_queue.clone()
    }

    pub async fn record_request(&self, duration: Duration, success: bool) {
        let mut metrics = self.metrics.write().await;
        metrics.total_requests += 1;
        metrics.total_processing_time += duration;
        
        if success {
            metrics.successful_requests += 1;
        } else {
            metrics.failed_requests += 1;
        }
        
        // Update average
        if metrics.total_requests > 0 {
            metrics.avg_processing_time = metrics.total_processing_time / metrics.total_requests as u32;
        }
    }

    pub async fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.read().await.clone()
    }

    pub async fn reset(&self) {
        let mut metrics = self.metrics.write().await;
        *metrics = PerformanceMetrics::new();
    }
}

/// Parallel processor for handling multiple requests concurrently
pub struct ParallelProcessor;

impl ParallelProcessor {
    /// Process multiple requests in parallel
    pub async fn process_parallel<F, R>(
        requests: Vec<QueuedRequest>,
        processor: F,
    ) -> Vec<Result<R, Box<dyn std::error::Error + Send + Sync>>>
    where
        F: Fn(QueuedRequest) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R, Box<dyn std::error::Error + Send + Sync>>> + Send>> + Send + Sync + Clone,
        R: Send + 'static,
    {
        use futures::future::join_all;
        
        let futures: Vec<_> = requests
            .into_iter()
            .map(|request| {
                let processor = processor.clone();
                tokio::spawn(async move {
                    processor(request).await
                })
            })
            .collect();

        let results = join_all(futures).await;
        results
            .into_iter()
            .map(|r| r.unwrap_or_else(|e| Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)))
            .collect()
    }
}

/// Cache for request responses
pub struct ResponseCache {
    cache: Arc<RwLock<std::collections::HashMap<String, CachedResponse>>>,
    ttl_seconds: u64,
}

#[derive(Debug, Clone)]
struct CachedResponse {
    response: String,
    cached_at: Instant,
}

impl ResponseCache {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(RwLock::new(std::collections::HashMap::new())),
            ttl_seconds,
        }
    }

    pub async fn get(&self, cache_key: &str) -> Option<String> {
        let cache = self.cache.read().await;
        if let Some(cached) = cache.get(cache_key) {
            if cached.cached_at.elapsed().as_secs() < self.ttl_seconds {
                return Some(cached.response.clone());
            }
        }
        None
    }

    pub async fn set(&self, cache_key: String, response: String) {
        let mut cache = self.cache.write().await;
        cache.insert(cache_key, CachedResponse {
            response,
            cached_at: Instant::now(),
        });
    }

    pub async fn invalidate(&self, cache_key: &str) {
        let mut cache = self.cache.write().await;
        cache.remove(cache_key);
    }

    pub async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        cache.retain(|_, cached| cached.cached_at.elapsed().as_secs() < self.ttl_seconds);
    }
}
