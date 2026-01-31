//! Tests for Phase 18.1.1: ConnectionCacheManager (cache, invalidation, TTL).

use bifrost::connection::{ConnectionCacheManager, ConnectionInfo, ConnectionManager};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn cache_miss_returns_none() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let cache = ConnectionCacheManager::new(Arc::clone(&conn_mgr), Duration::from_secs(60));
    let info = cache.get_connection_info("conn-1").await;
    assert!(info.is_none());
}

#[tokio::test]
async fn cache_stores_and_returns_info() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let cache = ConnectionCacheManager::new(Arc::clone(&conn_mgr), Duration::from_secs(60));
    cache.put_connection_info(ConnectionInfo {
        connection_id: "conn-1".to_string(),
        device_id: "device-1".to_string(),
        user_id: "user-1".to_string(),
    });
    let info = cache.get_connection_info("conn-1").await.unwrap();
    assert_eq!(info.connection_id, "conn-1");
    assert_eq!(info.device_id, "device-1");
    assert_eq!(info.user_id, "user-1");
}

#[tokio::test]
async fn invalidate_removes_entry() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let cache = ConnectionCacheManager::new(Arc::clone(&conn_mgr), Duration::from_secs(60));
    cache.put_connection_info(ConnectionInfo {
        connection_id: "conn-1".to_string(),
        device_id: "d1".to_string(),
        user_id: "u1".to_string(),
    });
    cache.invalidate("conn-1");
    let info = cache.get_connection_info("conn-1").await;
    assert!(info.is_none());
}

#[tokio::test]
async fn invalidate_all_clears_cache() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let cache = ConnectionCacheManager::new(Arc::clone(&conn_mgr), Duration::from_secs(60));
    cache.put_connection_info(ConnectionInfo {
        connection_id: "conn-1".to_string(),
        device_id: "d1".to_string(),
        user_id: "u1".to_string(),
    });
    cache.invalidate_all();
    assert!(cache.get_connection_info("conn-1").await.is_none());
}

#[tokio::test]
async fn expired_entry_returns_none() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let cache = ConnectionCacheManager::new(Arc::clone(&conn_mgr), Duration::from_millis(10));
    cache.put_connection_info(ConnectionInfo {
        connection_id: "conn-1".to_string(),
        device_id: "d1".to_string(),
        user_id: "u1".to_string(),
    });
    std::thread::sleep(Duration::from_millis(15));
    let info = cache.get_connection_info("conn-1").await;
    assert!(info.is_none());
}
