//! Priority-Queue-Manager (Phase 13.2.1): Prioritäts-basierte Queue, High-Priority zuerst.

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::queue::QueueFullError;

/// Eintrag mit Priorität (höhere Zahl = höhere Priorität); bei gleicher Priorität FIFO.
#[derive(Debug, Clone)]
struct Prioritized<T> {
    priority: u8,
    insert_order: u64,
    item: T,
}

impl<T> PartialEq for Prioritized<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.insert_order == other.insert_order
    }
}

impl<T> Eq for Prioritized<T> {}

impl<T> PartialOrd for Prioritized<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Prioritized<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| other.insert_order.cmp(&self.insert_order))
    }
}

/// Priority-Queue: bei dequeue wird stets das Element mit höchster Priorität entnommen (bei Gleichstand FIFO).
#[derive(Debug, Clone)]
pub struct PriorityQueueManager<T> {
    heap: BinaryHeap<Prioritized<T>>,
    max_capacity: Option<usize>,
    insert_counter: u64,
}

impl<T> PriorityQueueManager<T> {
    /// Erstellt eine Priority-Queue ohne Kapazitätsbegrenzung.
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            max_capacity: None,
            insert_counter: 0,
        }
    }

    /// Erstellt eine Priority-Queue mit maximaler Kapazität.
    pub fn with_capacity(max_capacity: usize) -> Self {
        Self {
            heap: BinaryHeap::new(),
            max_capacity: Some(max_capacity),
            insert_counter: 0,
        }
    }

    /// Reiht einen Request mit Priorität ein (höhere Zahl = höhere Priorität). Fehler bei voller Queue.
    pub fn enqueue(&mut self, item: T, priority: u8) -> Result<(), QueueFullError> {
        if let Some(max) = self.max_capacity {
            if self.heap.len() >= max {
                return Err(QueueFullError);
            }
        }
        self.insert_counter += 1;
        self.heap.push(Prioritized {
            priority,
            insert_order: self.insert_counter,
            item,
        });
        Ok(())
    }

    /// Entnimmt das Element mit höchster Priorität (bei Gleichstand: zuerst eingereiht).
    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.pop().map(|p| p.item)
    }

    /// Anzahl der wartenden Requests.
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// `true`, wenn die Queue leer ist.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

impl<T> Default for PriorityQueueManager<T> {
    fn default() -> Self {
        Self::new()
    }
}
