//! Message Queuing (Phase 16.1). Queue for offline devices; FIFO delivery when online.

use crate::message::BifrostMessage;
use crate::routing::MessageRouter;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, RwLock};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum QueueError {
    #[error("queue full")]
    QueueFull,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueOverflowStrategy {
    EvictOldest,
    Reject,
}

/// Queues messages for offline devices; configurable size limit and overflow handling.
pub struct MessageQueueManager {
    max_per_device: usize,
    overflow: QueueOverflowStrategy,
    queues: RwLock<HashMap<String, VecDeque<BifrostMessage>>>,
}

impl MessageQueueManager {
    pub fn new(max_per_device: usize, overflow: QueueOverflowStrategy) -> Self {
        Self {
            max_per_device: max_per_device.max(1),
            overflow,
            queues: RwLock::new(HashMap::new()),
        }
    }

    pub fn enqueue(&self, device_id: &str, msg: BifrostMessage) -> Result<(), QueueError> {
        let mut map = self.queues.write().unwrap();
        let q = map.entry(device_id.to_string()).or_default();
        if q.len() >= self.max_per_device {
            match self.overflow {
                QueueOverflowStrategy::EvictOldest => {
                    q.pop_front();
                }
                QueueOverflowStrategy::Reject => return Err(QueueError::QueueFull),
            }
        }
        q.push_back(msg);
        Ok(())
    }

    pub fn queue_len(&self, device_id: &str) -> usize {
        let map = self.queues.read().unwrap();
        map.get(device_id).map(|q| q.len()).unwrap_or(0)
    }

    /// Drains and returns all queued messages for the device (FIFO). Removes them from the queue.
    pub fn drain(&self, device_id: &str) -> Vec<BifrostMessage> {
        let mut map = self.queues.write().unwrap();
        map.remove(device_id)
            .map(|q| q.into_iter().collect())
            .unwrap_or_default()
    }
}

/// Delivers queued messages to a device when it comes online (FIFO); uses MessageRouter.
pub struct QueueDeliveryManager {
    queue_manager: Arc<MessageQueueManager>,
    router: MessageRouter,
}

impl QueueDeliveryManager {
    pub fn new(queue_manager: Arc<MessageQueueManager>, router: MessageRouter) -> Self {
        Self {
            queue_manager,
            router,
        }
    }

    /// Drains queue for device and routes each message. Returns (delivered_count, failed_count).
    pub async fn deliver_to(&self, device_id: &str) -> (u32, u32) {
        let messages = self.queue_manager.drain(device_id);
        let mut delivered = 0u32;
        let mut failed = 0u32;
        for msg in messages {
            match self.router.route_message(msg).await {
                Ok(()) => delivered += 1,
                Err(_) => failed += 1,
            }
        }
        (delivered, failed)
    }
}
