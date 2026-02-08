//! TaskScheduler (Phase 6.2.1, TDD).

use std::sync::Mutex;

type Task = Box<dyn FnOnce() + Send>;

/// Priority-based task scheduler with optional concurrency limit.
pub struct TaskScheduler {
    max_queued: usize,
    queue: Mutex<Vec<(u32, Task)>>,
}

impl TaskScheduler {
    pub fn new(max_queued: usize) -> Self {
        Self {
            max_queued,
            queue: Mutex::new(Vec::new()),
        }
    }

    pub fn submit(&self, priority: u32, task: Task) {
        if let Ok(mut q) = self.queue.lock() {
            if q.len() < self.max_queued {
                q.push((priority, task));
            }
        }
    }

    /// Run the highest-priority task. Returns true if a task was run.
    pub fn run_next(&self) -> bool {
        let task = {
            let mut q = match self.queue.lock() {
                Ok(g) => g,
                Err(_) => return false,
            };
            if q.is_empty() {
                return false;
            }
            let idx = q
                .iter()
                .enumerate()
                .max_by_key(|(_, (p, _))| *p)
                .map(|(i, _)| i)
                .unwrap();
            let (_, t) = q.remove(idx);
            t
        };
        task();
        true
    }
}
