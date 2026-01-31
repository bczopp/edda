//! Tests for ValidationCacheManager (Phase 9.6.1): connection validation cache, TTL, invalidate.

use heimdall::bifrost::ValidationCacheManager;

const TTL_SECS: i64 = 300;

#[tokio::test]
async fn test_cache_set_and_get() {
    let cache = ValidationCacheManager::new(TTL_SECS);
    cache.set("dev-a:dev-b", true).await;
    let v = cache.get("dev-a:dev-b").await;
    assert_eq!(v, Some(true));
}

#[tokio::test]
async fn test_cache_miss_returns_none() {
    let cache = ValidationCacheManager::new(TTL_SECS);
    let v = cache.get("unknown:unknown").await;
    assert!(v.is_none());
}

#[tokio::test]
async fn test_invalidate_removes_entry() {
    let cache = ValidationCacheManager::new(TTL_SECS);
    cache.set("dev-a:dev-b", true).await;
    cache.invalidate("dev-a:dev-b").await;
    assert!(cache.get("dev-a:dev-b").await.is_none());
}

#[tokio::test]
async fn test_invalidate_device_removes_entries_for_device() {
    let cache = ValidationCacheManager::new(TTL_SECS);
    cache.set("dev-a:dev-b", true).await;
    cache.set("dev-a:dev-c", true).await;
    cache.set("dev-x:dev-y", true).await;
    cache.invalidate_device("dev-a").await;
    assert!(cache.get("dev-a:dev-b").await.is_none());
    assert!(cache.get("dev-a:dev-c").await.is_none());
    assert_eq!(cache.get("dev-x:dev-y").await, Some(true));
}
