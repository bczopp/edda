//! MemoryPool (Phase 6.1.2, TDD).

use std::sync::Mutex;

/// Fixed-size buffer pool for efficient reuse.
pub struct MemoryPool {
    buffer_size: usize,
    max_size: usize,
    available: Mutex<Vec<Vec<u8>>>,
}

impl MemoryPool {
    pub fn new(max_buffers: usize, buffer_size: usize) -> Self {
        let available: Vec<Vec<u8>> = (0..max_buffers).map(|_| vec![0u8; buffer_size]).collect();
        Self {
            buffer_size,
            max_size: max_buffers,
            available: Mutex::new(available),
        }
    }

    pub fn acquire(&self) -> Option<Vec<u8>> {
        self.available.lock().ok()?.pop()
    }

    pub fn release(&self, mut buf: Vec<u8>) {
        buf.clear();
        if buf.capacity() >= self.buffer_size {
            buf.resize(self.buffer_size, 0);
            if let Ok(mut av) = self.available.lock() {
                if av.len() < self.max_size {
                    av.push(buf);
                }
            }
        }
    }
}
