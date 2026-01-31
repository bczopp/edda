//! Tests for Permission-Cache-Manager (Phase 13.2.1): permission check cache, invalidate on change.

use heimdall::utils::PermissionCheckCache;

#[tokio::test]
async fn test_permission_cache_set_and_get() {
    let cache = PermissionCheckCache::new(300);
    let key = "dev:user:resource:read";
    cache.set(key.to_string(), true).await;
    assert_eq!(cache.get(key).await, Some(true));
}

#[tokio::test]
async fn test_permission_cache_invalidate_device_removes_entries() {
    let cache = PermissionCheckCache::new(300);
    cache.set("dev-a:user:res:read".to_string(), true).await;
    cache.set("dev-a:user:res:write".to_string(), true).await;
    cache.set("dev-b:user:res:read".to_string(), true).await;
    cache.invalidate_device("dev-a").await;
    assert!(cache.get("dev-a:user:res:read").await.is_none());
    assert!(cache.get("dev-a:user:res:write").await.is_none());
    assert_eq!(cache.get("dev-b:user:res:read").await, Some(true));
}
