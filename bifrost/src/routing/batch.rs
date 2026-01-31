//! Message Batch Manager (Phase 18.3.1). Batch messages; size limit; batch delivery.

use crate::message::BifrostMessage;
use crate::routing::MessageRouter;
use std::collections::VecDeque;
use std::sync::RwLock;

/// Batches multiple messages; configurable size limit; delivers batch via router.
pub struct MessageBatchManager {
    max_batch_size: usize,
    pending: RwLock<VecDeque<BifrostMessage>>,
}

impl MessageBatchManager {
    pub fn new(max_batch_size: usize) -> Self {
        Self {
            max_batch_size: max_batch_size.max(1),
            pending: RwLock::new(VecDeque::new()),
        }
    }

    pub fn add(&self, msg: BifrostMessage) {
        self.pending.write().unwrap().push_back(msg);
    }

    pub fn pending_len(&self) -> usize {
        self.pending.read().unwrap().len()
    }

    /// Takes up to max_batch_size messages from pending (FIFO). Removes them from pending.
    pub fn take_batch(&self) -> Vec<BifrostMessage> {
        let mut q = self.pending.write().unwrap();
        let n = (q.len()).min(self.max_batch_size);
        (0..n).filter_map(|_| q.pop_front()).collect()
    }

    /// Takes one batch and delivers via router. Returns (delivered_count, failed_count).
    pub async fn deliver_batch(
        &self,
        router: &MessageRouter,
    ) -> (u32, u32) {
        let batch = self.take_batch();
        let mut ok = 0u32;
        let mut failed = 0u32;
        for msg in batch {
            match router.route_message(msg).await {
                Ok(()) => ok += 1,
                Err(_) => failed += 1,
            }
        }
        (ok, failed)
    }
}
