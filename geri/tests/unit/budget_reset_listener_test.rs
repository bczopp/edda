//! Tests für Budget-Reset-Listener (Phase 10.3.1).

#[cfg(test)]
mod tests {
    use geri::fallback::{BudgetResetHandler, BudgetResetListener};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockHandler(Arc<AtomicBool>);
    impl BudgetResetHandler for MockHandler {
        fn on_budget_reset(&self) {
            self.0.store(true, Ordering::SeqCst);
        }
    }

    #[test]
    fn notify_reset_calls_handler() {
        let called = Arc::new(AtomicBool::new(false));
        let handler = MockHandler(Arc::clone(&called));
        let listener = BudgetResetListener::new(Box::new(handler));
        assert!(!called.load(Ordering::SeqCst));
        listener.notify_reset();
        assert!(called.load(Ordering::SeqCst));
    }

    struct CountHandler(Arc<AtomicUsize>);
    impl BudgetResetHandler for CountHandler {
        fn on_budget_reset(&self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn notify_reset_can_be_called_multiple_times() {
        let count = Arc::new(AtomicUsize::new(0));
        let handler = CountHandler(Arc::clone(&count));
        let listener = BudgetResetListener::new(Box::new(handler));
        listener.notify_reset();
        listener.notify_reset();
        assert_eq!(count.load(Ordering::SeqCst), 2);
    }
}
