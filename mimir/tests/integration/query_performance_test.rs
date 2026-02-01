use mimir::storage::EncryptedDatabase;
use mimir::encryption::EncryptionManager;
use tests::common::TestDatabase;
use ring::rand::{SecureRandom, SystemRandom};
use std::time::Instant;

fn generate_key() -> Vec<u8> {
    let mut key = vec![0u8; 32];
    let rng = SystemRandom::new();
    rng.fill(&mut key).unwrap();
    key
}

#[tokio::test]
async fn test_query_performance_standard_query() {
    let test_db = TestDatabase::new().await.unwrap();
    let key = generate_key();
    let encryption_manager = EncryptionManager::new(&key).unwrap();
    
    let database = EncryptedDatabase::new_with_encryption(
        &test_db.pool,
        encryption_manager,
    ).await.unwrap();
    
    let user_id = "user123";
    let data = b"test data";
    
    // Store data
    let data_id = database.store_data(user_id, data).await.unwrap();
    
    // Measure query performance
    let start = Instant::now();
    let _result = database.retrieve_data(&data_id, user_id).await.unwrap();
    let duration = start.elapsed();
    
    // Should be fast (< 50ms for standard queries)
    assert!(duration.as_millis() < 50, "Query took {}ms, expected < 50ms", duration.as_millis());
}

#[tokio::test]
async fn test_write_performance_standard_write() {
    let test_db = TestDatabase::new().await.unwrap();
    let key = generate_key();
    let encryption_manager = EncryptionManager::new(&key).unwrap();

    let database = EncryptedDatabase::new_with_encryption(
        &test_db.pool,
        encryption_manager,
    )
    .await
    .unwrap();

    let user_id = "user-perf";
    let data = b"standard write payload for performance test";

    let start = Instant::now();
    let _data_id = database.store_data(user_id, data).await.unwrap();
    let duration = start.elapsed();

    assert!(
        duration.as_millis() < 100,
        "Write took {}ms, expected < 100ms",
        duration.as_millis()
    );
}

#[tokio::test]
async fn test_query_performance_index_usage() {
    let test_db = TestDatabase::new().await.unwrap();
    let key = generate_key();
    let encryption_manager = EncryptionManager::new(&key).unwrap();
    
    let database = EncryptedDatabase::new_with_encryption(
        &test_db.pool,
        encryption_manager,
    ).await.unwrap();
    
    let user_id = "user123";
    
    // Store multiple data entries
    for i in 0..10 {
        let data = format!("data_{}", i).into_bytes();
        database.store_data(user_id, &data).await.unwrap();
    }
    
    // Query all user data (should use index on user_id)
    let start = Instant::now();
    let all_data = database.get_all_user_data(user_id).await.unwrap();
    let duration = start.elapsed();
    
    assert_eq!(all_data.len(), 10);
    // Should be fast even with multiple entries (< 100ms)
    assert!(duration.as_millis() < 100, "Query took {}ms, expected < 100ms", duration.as_millis());
}

#[tokio::test]
async fn test_query_performance_with_cache() {
    let test_db = TestDatabase::new().await.unwrap();
    let key = generate_key();
    let encryption_manager = EncryptionManager::new(&key).unwrap();
    
    // Create cache
    use mimir::cache::CacheManager;
    use std::time::Duration;
    use std::sync::Arc;
    let cache = Arc::new(CacheManager::new(1000, Duration::from_secs(300)));
    
    let database = EncryptedDatabase::new_with_cache(
        &test_db.pool,
        encryption_manager,
        cache.clone(),
    ).await.unwrap();
    
    let user_id = "user123";
    let data = b"test data";
    
    // Store data
    let data_id = database.store_data(user_id, data).await.unwrap();
    
    // First query (cache miss - should hit database)
    let start = Instant::now();
    let _result1 = database.retrieve_data(&data_id, user_id).await.unwrap();
    let duration1 = start.elapsed();
    
    // Second query (cache hit - should be much faster)
    let start = Instant::now();
    let _result2 = database.retrieve_data(&data_id, user_id).await.unwrap();
    let duration2 = start.elapsed();
    
    // Cache hit should be significantly faster
    assert!(duration2 < duration1, "Cache hit ({:?}) should be faster than cache miss ({:?})", duration2, duration1);
    // Cache hit should be very fast (< 1ms)
    assert!(duration2.as_micros() < 1000, "Cache hit took {}μs, expected < 1000μs", duration2.as_micros());
}
