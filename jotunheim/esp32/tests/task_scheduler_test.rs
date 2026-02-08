// TaskScheduler tests (Phase 6.2.1, TDD).

use jotunheim_esp32::resources::TaskScheduler;
use std::sync::atomic::{AtomicU32, Ordering};

#[test]
fn run_next_executes_highest_priority_task() {
    let ran = std::sync::Arc::new(AtomicU32::new(0));
    let r = ran.clone();
    let mut sched = TaskScheduler::new(2);
    sched.submit(10, Box::new(move || r.store(10, Ordering::SeqCst)));
    sched.submit(5, Box::new(move || ran.store(5, Ordering::SeqCst)));
    sched.run_next();
    assert_eq!(ran.load(Ordering::SeqCst), 10);
}

#[test]
fn run_next_returns_false_when_empty() {
    let mut sched = TaskScheduler::new(1);
    assert!(!sched.run_next());
}

#[test]
fn respects_max_concurrent() {
    let mut sched = TaskScheduler::new(1);
    sched.submit(1, Box::new(|| {}));
    sched.run_next();
    assert!(!sched.run_next());
}
