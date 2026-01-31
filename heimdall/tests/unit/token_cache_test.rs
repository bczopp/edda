//! Tests for Token-Cache-Manager (Phase 13.1.1): token validation cache, TTL, invalidate on revocation.

use heimdall::utils::TokenValidationCache;

#[tokio::test]
async fn test_token_cache_set_and_get() {
    let cache = TokenValidationCache::new(300);
    cache.set("token-id-1".to_string(), true).await;
    assert_eq!(cache.get("token-id-1").await, Some(true));
}

#[tokio::test]
async fn test_token_cache_miss_returns_none() {
    let cache = TokenValidationCache::new(300);
    assert!(cache.get("unknown").await.is_none());
}

#[tokio::test]
async fn test_token_cache_invalidate_removes_entry() {
    let cache = TokenValidationCache::new(300);
    cache.set("token-id-1".to_string(), true).await;
    cache.invalidate("token-id-1").await;
    assert!(cache.get("token-id-1").await.is_none());
}

#[tokio::test]
async fn test_token_cache_ttl_expiration() {
    let cache = TokenValidationCache::new(0);
    cache.set("token-id-1".to_string(), true).await;
    // TTL 0 => expires_at = now (seconds); wait >1s so get() sees entry as expired
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    assert!(cache.get("token-id-1").await.is_none());
}
