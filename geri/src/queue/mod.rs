//! Request-Queuing (Phase 13.1, 13.2): FIFO-Queue, Priority-Queue.

mod manager;
mod priority;
pub use manager::{QueueFullError, RequestQueueManager};
pub use priority::PriorityQueueManager;
