#[derive(Debug, Clone)]
pub struct PoolStats {
    pub size: u32,
    pub idle: usize,
    pub is_closed: bool,
}

impl PoolStats {
    pub fn new(size: u32, idle: usize, is_closed: bool) -> Self {
        Self { size, idle, is_closed }
    }
}
