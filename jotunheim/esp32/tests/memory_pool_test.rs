// MemoryPool tests (Phase 6.1.2, TDD).

use jotunheim_esp32::resources::MemoryPool;

#[test]
fn acquire_returns_buffer_of_configured_size() {
    let pool = MemoryPool::new(2, 64);
    let buf = pool.acquire().unwrap();
    assert_eq!(buf.len(), 64);
}

#[test]
fn release_returns_buffer_to_pool() {
    let pool = MemoryPool::new(1, 32);
    let buf = pool.acquire().unwrap();
    assert_eq!(buf.len(), 32);
    pool.release(buf);
    let buf2 = pool.acquire().unwrap();
    assert_eq!(buf2.len(), 32);
}

#[test]
fn acquire_fails_when_pool_exhausted() {
    let pool = MemoryPool::new(1, 16);
    let _b1 = pool.acquire().unwrap();
    assert!(pool.acquire().is_none());
}
