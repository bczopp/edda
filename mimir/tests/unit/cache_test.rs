use mimir::cache::CacheManager;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
async fn test_cache_hit_miss() {
    let cache = CacheManager::new(100, Duration::from_secs(60));
    
    // Cache miss - data not in cache
    let result = cache.get("key1").await;
    assert!(result.is_none());
    
    // Store data
    cache.set("key1", b"value1").await;
    
    // Cache hit - data found in cache
    let result = cache.get("key1").await;
    assert_eq!(result, Some(b"value1".to_vec()));
}

#[tokio::test]
async fn test_cache_expiration() {
    let cache = CacheManager::new(100, Duration::from_millis(100));
    
    // Store data
    cache.set("key1", b"value1").await;
    
    // Should be in cache immediately
    assert_eq!(cache.get("key1").await, Some(b"value1".to_vec()));
    
    // Wait for expiration
    sleep(Duration::from_millis(150)).await;
    
    // Should be expired (cache miss)
    assert_eq!(cache.get("key1").await, None);
}

#[tokio::test]
async fn test_cache_invalidation() {
    let cache = CacheManager::new(100, Duration::from_secs(60));
    
    // Store data
    cache.set("key1", b"value1").await;
    assert_eq!(cache.get("key1").await, Some(b"value1".to_vec()));
    
    // Invalidate
    cache.invalidate("key1").await;
    
    // Should be removed
    assert_eq!(cache.get("key1").await, None);
}

#[tokio::test]
async fn test_cache_invalidate_user() {
    let cache = CacheManager::new(100, Duration::from_secs(60));
    
    // Store data for user1
    cache.set("user1_data1", b"value1").await;
    cache.set("user1_data2", b"value2").await;
    
    // Store data for user2
    cache.set("user2_data1", b"value3").await;
    
    // Invalidate all data for user1
    cache.invalidate_user("user1").await;
    
    // user1 data should be gone
    assert_eq!(cache.get("user1_data1").await, None);
    assert_eq!(cache.get("user1_data2").await, None);
    
    // user2 data should still be there
    assert_eq!(cache.get("user2_data1").await, Some(b"value3".to_vec()));
}

#[tokio::test]
async fn test_cache_size_limit() {
    let cache = CacheManager::new(2, Duration::from_secs(60));
    
    // Fill cache to capacity
    cache.set("key1", b"value1").await;
    cache.set("key2", b"value2").await;
    
    // Add one more - should evict oldest (LRU)
    cache.set("key3", b"value3").await;
    
    // key1 should be evicted (oldest)
    assert_eq!(cache.get("key1").await, None);
    
    // key2 and key3 should still be there
    assert_eq!(cache.get("key2").await, Some(b"value2".to_vec()));
    assert_eq!(cache.get("key3").await, Some(b"value3".to_vec()));
}

#[tokio::test]
async fn test_cache_stats() {
    let cache = CacheManager::new(100, Duration::from_secs(60));
    
    // Initial stats
    let stats = cache.get_stats().await;
    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    
    // Cache miss
    cache.get("key1").await;
    let stats = cache.get_stats().await;
    assert_eq!(stats.misses, 1);
    
    // Store and hit
    cache.set("key1", b"value1").await;
    cache.get("key1").await;
    let stats = cache.get_stats().await;
    assert_eq!(stats.hits, 1);
    assert_eq!(stats.misses, 1);
}
