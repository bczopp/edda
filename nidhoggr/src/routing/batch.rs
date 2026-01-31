use ratatoskr::messages::RatatoskrRequest;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct MessageBatchProcessor {
    batch_size: usize,
    batch_timeout: Duration,
    pending_batch: Arc<RwLock<VecDeque<RatatoskrRequest>>>,
    last_batch_time: Arc<RwLock<Instant>>,
}

impl MessageBatchProcessor {
    pub fn new(batch_size: usize, batch_timeout_ms: u64) -> Self {
        Self {
            batch_size,
            batch_timeout: Duration::from_millis(batch_timeout_ms),
            pending_batch: Arc::new(RwLock::new(VecDeque::new())),
            last_batch_time: Arc::new(RwLock::new(Instant::now())),
        }
    }

    pub async fn add_message(&self, request: RatatoskrRequest) -> Option<Vec<RatatoskrRequest>> {
        let mut batch = self.pending_batch.write().await;
        batch.push_back(request);
        
        // Check if batch is full
        if batch.len() >= self.batch_size {
            let messages: Vec<RatatoskrRequest> = batch.drain(..).collect();
            *self.last_batch_time.write().await = Instant::now();
            return Some(messages);
        }
        
        None
    }

    pub async fn check_timeout(&self) -> Option<Vec<RatatoskrRequest>> {
        let last_time = *self.last_batch_time.read().await;
        let now = Instant::now();
        
        if now.duration_since(last_time) >= self.batch_timeout {
            let mut batch = self.pending_batch.write().await;
            if !batch.is_empty() {
                let messages: Vec<RatatoskrRequest> = batch.drain(..).collect();
                *self.last_batch_time.write().await = now;
                return Some(messages);
            }
        }
        
        None
    }

    pub async fn flush(&self) -> Vec<RatatoskrRequest> {
        let mut batch = self.pending_batch.write().await;
        let messages: Vec<RatatoskrRequest> = batch.drain(..).collect();
        *self.last_batch_time.write().await = Instant::now();
        messages
    }
}
