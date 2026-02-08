//! Tests für Priority-Queue-Manager (Phase 13.2.1).

#[cfg(test)]
mod tests {
    use geri::queue::PriorityQueueManager;

    #[test]
    fn high_priority_dequeued_first() {
        let mut q = PriorityQueueManager::<String>::new();
        q.enqueue("low".to_string(), 1).unwrap();
        q.enqueue("high".to_string(), 10).unwrap();
        q.enqueue("mid".to_string(), 5).unwrap();
        assert_eq!(q.dequeue(), Some("high".to_string()));
        assert_eq!(q.dequeue(), Some("mid".to_string()));
        assert_eq!(q.dequeue(), Some("low".to_string()));
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn same_priority_fifo_order() {
        let mut q = PriorityQueueManager::<i32>::new();
        q.enqueue(1, 5).unwrap();
        q.enqueue(2, 5).unwrap();
        q.enqueue(3, 5).unwrap();
        assert_eq!(q.dequeue(), Some(1));
        assert_eq!(q.dequeue(), Some(2));
        assert_eq!(q.dequeue(), Some(3));
    }

    #[test]
    fn empty_dequeue_returns_none() {
        let mut q = PriorityQueueManager::<i32>::new();
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn len_and_is_empty() {
        let mut q = PriorityQueueManager::<i32>::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
        q.enqueue(1, 1).unwrap();
        q.enqueue(2, 2).unwrap();
        assert!(!q.is_empty());
        assert_eq!(q.len(), 2);
        q.dequeue();
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn with_capacity_rejects_when_full() {
        let mut q = PriorityQueueManager::<i32>::with_capacity(2);
        q.enqueue(1, 1).unwrap();
        q.enqueue(2, 2).unwrap();
        assert!(q.enqueue(3, 3).is_err());
        q.dequeue();
        assert!(q.enqueue(3, 3).is_ok());
    }
}
