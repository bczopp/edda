//! Tests for Phase 18.2.1: Validation Cache Manager (cache validation results, TTL, invalidation).

use bifrost::connection::validation_cache::{ValidationCacheManager, ValidationResult};
use std::time::Duration;

fn cache_key(user_id: &str, device_id: &str) -> String {
    format!("{}:{}", user_id, device_id)
}

#[tokio::test]
async fn get_misses_when_empty() {
    let mgr = ValidationCacheManager::new(Duration::from_secs(300));
    let key = cache_key("u1", "d1");
    assert!(mgr.get(&key).await.is_none());
}

#[tokio::test]
async fn get_returns_cached_result_within_ttl() {
    let mgr = ValidationCacheManager::new(Duration::from_secs(300));
    let key = cache_key("u1", "d1");
    mgr.set(&key, ValidationResult::Allowed).await;
    assert_eq!(mgr.get(&key).await, Some(ValidationResult::Allowed));
}

#[tokio::test]
async fn get_returns_denied_when_cached_denied() {
    let mgr = ValidationCacheManager::new(Duration::from_secs(300));
    let key = cache_key("u1", "d1");
    mgr.set(&key, ValidationResult::Denied).await;
    assert_eq!(mgr.get(&key).await, Some(ValidationResult::Denied));
}

#[tokio::test]
async fn invalidate_removes_entry() {
    let mgr = ValidationCacheManager::new(Duration::from_secs(300));
    let key = cache_key("u1", "d1");
    mgr.set(&key, ValidationResult::Allowed).await;
    mgr.invalidate(&key).await;
    assert!(mgr.get(&key).await.is_none());
}

#[tokio::test]
async fn invalidate_all_clears_cache() {
    let mgr = ValidationCacheManager::new(Duration::from_secs(300));
    mgr.set("u1:d1", ValidationResult::Allowed).await;
    mgr.set("u1:d2", ValidationResult::Denied).await;
    mgr.invalidate_all().await;
    assert!(mgr.get("u1:d1").await.is_none());
    assert!(mgr.get("u1:d2").await.is_none());
}

#[tokio::test]
async fn expired_entry_returns_none() {
    let mgr = ValidationCacheManager::new(Duration::from_millis(1));
    let key = cache_key("u1", "d1");
    mgr.set(&key, ValidationResult::Allowed).await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    assert!(mgr.get(&key).await.is_none());
}
