//! Tests für Request-Queue-Manager (Phase 13.1.1).

#[cfg(test)]
mod tests {
    use geri::queue::RequestQueueManager;

    #[test]
    fn enqueue_dequeue_fifo() {
        let mut q = RequestQueueManager::<String>::new();
        q.enqueue("a".to_string()).unwrap();
        q.enqueue("b".to_string()).unwrap();
        assert_eq!(q.dequeue(), Some("a".to_string()));
        assert_eq!(q.dequeue(), Some("b".to_string()));
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn empty_dequeue_returns_none() {
        let mut q = RequestQueueManager::<i32>::new();
        assert_eq!(q.dequeue(), None);
    }

    #[test]
    fn len_increases_on_enqueue() {
        let mut q = RequestQueueManager::<i32>::new();
        assert_eq!(q.len(), 0);
        q.enqueue(1).unwrap();
        q.enqueue(2).unwrap();
        assert_eq!(q.len(), 2);
        q.dequeue();
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn is_empty() {
        let mut q = RequestQueueManager::<i32>::new();
        assert!(q.is_empty());
        q.enqueue(1).unwrap();
        assert!(!q.is_empty());
        q.dequeue();
        assert!(q.is_empty());
    }

    #[test]
    fn backlog_len_equals_len() {
        let mut q = RequestQueueManager::<i32>::new();
        q.enqueue(1).unwrap();
        assert_eq!(q.backlog_len(), q.len());
    }

    #[test]
    fn max_capacity_rejects_when_full() {
        let mut q = RequestQueueManager::with_capacity(2);
        assert!(q.enqueue("a".to_string()).is_ok());
        assert!(q.enqueue("b".to_string()).is_ok());
        assert!(q.enqueue("c".to_string()).is_err());
        assert_eq!(q.dequeue(), Some("a".to_string()));
        assert!(q.enqueue("c".to_string()).is_ok());
    }
}
