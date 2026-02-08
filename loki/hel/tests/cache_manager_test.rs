//! Tests for CacheManager (TDD – Phase 8.4.1).

use hel::CacheManager;
use std::time::Duration;

#[test]
fn cache_set_get() {
    let c = CacheManager::new(60);
    c.set("k", "v").unwrap();
    assert_eq!(c.get("k").unwrap(), Some("v".into()));
}

#[test]
fn cache_invalidate() {
    let c = CacheManager::new(60);
    c.set("k", "v").unwrap();
    c.invalidate("k").unwrap();
    assert_eq!(c.get("k").unwrap(), None);
}

#[test]
fn cache_ttl_expires() {
    let c = CacheManager::new(0);
    c.set_with_ttl("k", "v", Duration::from_secs(0)).unwrap();
    std::thread::sleep(Duration::from_millis(50));
    assert_eq!(c.get("k").unwrap(), None);
}

#[test]
fn cache_invalidate_all() {
    let c = CacheManager::new(60);
    c.set("a", "1").unwrap();
    c.set("b", "2").unwrap();
    c.invalidate_all().unwrap();
    assert_eq!(c.get("a").unwrap(), None);
    assert_eq!(c.get("b").unwrap(), None);
}
