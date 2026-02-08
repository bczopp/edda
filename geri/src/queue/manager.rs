//! Request-Queue-Manager (Phase 13.1.1): FIFO-Queue, Backlog bei hoher Last.

use std::collections::VecDeque;

use thiserror::Error;

/// Fehler, wenn die Queue ihre maximale Kapazität erreicht hat.
#[derive(Debug, Clone, Error, PartialEq, Eq)]
#[error("Queue full (max capacity reached)")]
pub struct QueueFullError;

/// FIFO-Queue für Requests (bei hoher Last); optional begrenzte Kapazität.
#[derive(Debug, Clone)]
pub struct RequestQueueManager<T> {
    queue: VecDeque<T>,
    max_capacity: Option<usize>,
}

impl<T> RequestQueueManager<T> {
    /// Erstellt eine Queue ohne Kapazitätsbegrenzung.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            max_capacity: None,
        }
    }

    /// Erstellt eine Queue mit maximaler Kapazität (Backlog-Handling).
    pub fn with_capacity(max_capacity: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            max_capacity: Some(max_capacity),
        }
    }

    /// Stellt einen Request in die Queue (FIFO). Fehler, wenn Kapazität erreicht.
    pub fn enqueue(&mut self, item: T) -> Result<(), QueueFullError> {
        if let Some(max) = self.max_capacity {
            if self.queue.len() >= max {
                return Err(QueueFullError);
            }
        }
        self.queue.push_back(item);
        Ok(())
    }

    /// Entnimmt den nächsten Request (FIFO).
    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    /// Aktuelle Anzahl der wartenden Requests.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// `true`, wenn keine Requests in der Queue sind.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Backlog-Länge (gleich `len()`).
    pub fn backlog_len(&self) -> usize {
        self.queue.len()
    }
}

impl<T> Default for RequestQueueManager<T> {
    fn default() -> Self {
        Self::new()
    }
}
