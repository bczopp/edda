//! Tests for TTS Cache Manager

use muninn::cache::TTSCacheManager;
use shared::audio::AudioBuffer;
use std::time::Duration;

#[tokio::test]
async fn test_tts_cache_manager_new() {
    let cache = TTSCacheManager::new(100, Duration::from_secs(3600));
    assert_eq!(cache.max_size(), 100);
    assert_eq!(cache.ttl(), Duration::from_secs(3600));
}

#[tokio::test]
async fn test_tts_cache_get_miss() {
    let cache = TTSCacheManager::new(100, Duration::from_secs(3600));
    let key = cache.generate_key("Hello", "en-US", "male");
    let result = cache.get(&key).await;
    assert!(result.is_none());
}

#[tokio::test]
async fn test_tts_cache_set_and_get() {
    let cache = TTSCacheManager::new(100, Duration::from_secs(3600));
    let key = cache.generate_key("Hello", "en-US", "male");
    
    let audio_buffer = AudioBuffer {
        samples: vec![0i16; 1000],
        sample_rate: 44100,
        channels: 1,
        duration_ms: 100,
    };
    
    cache.set(&key, audio_buffer.clone()).await;
    
    let cached = cache.get(&key).await;
    assert!(cached.is_some());
    let cached_buffer = cached.unwrap();
    assert_eq!(cached_buffer.samples.len(), audio_buffer.samples.len());
    assert_eq!(cached_buffer.sample_rate, audio_buffer.sample_rate);
}

#[tokio::test]
async fn test_tts_cache_expiration() {
    let cache = TTSCacheManager::new(100, Duration::from_millis(100));
    let key = cache.generate_key("Hello", "en-US", "male");
    
    let audio_buffer = AudioBuffer {
        samples: vec![0i16; 1000],
        sample_rate: 44100,
        channels: 1,
        duration_ms: 100,
    };
    
    cache.set(&key, audio_buffer).await;
    
    // Should be cached immediately
    assert!(cache.get(&key).await.is_some());
    
    // Wait for expiration
    tokio::time::sleep(Duration::from_millis(150)).await;
    
    // Should be expired
    assert!(cache.get(&key).await.is_none());
}

#[tokio::test]
async fn test_tts_cache_max_size() {
    let cache = TTSCacheManager::new(2, Duration::from_secs(3600));
    
    // Add 3 entries (exceeds max_size)
    for i in 0..3 {
        let key = cache.generate_key(&format!("Text{}", i), "en-US", "male");
        let audio_buffer = AudioBuffer {
            samples: vec![0i16; 100],
            sample_rate: 44100,
            channels: 1,
            duration_ms: 10,
        };
        cache.set(&key, audio_buffer).await;
    }
    
    // First entry should be evicted (LRU)
    let first_key = cache.generate_key("Text0", "en-US", "male");
    assert!(cache.get(&first_key).await.is_none());
    
    // Last two entries should still be cached
    let second_key = cache.generate_key("Text1", "en-US", "male");
    let third_key = cache.generate_key("Text2", "en-US", "male");
    assert!(cache.get(&second_key).await.is_some());
    assert!(cache.get(&third_key).await.is_some());
}

#[tokio::test]
async fn test_tts_cache_generate_key() {
    let cache = TTSCacheManager::new(100, Duration::from_secs(3600));
    
    let key1 = cache.generate_key("Hello", "en-US", "male");
    let key2 = cache.generate_key("Hello", "en-US", "male");
    let key3 = cache.generate_key("Hello", "de-DE", "male");
    
    assert_eq!(key1, key2); // Same text, language, voice
    assert_ne!(key1, key3); // Different language
}

#[tokio::test]
async fn test_tts_cache_clear() {
    let cache = TTSCacheManager::new(100, Duration::from_secs(3600));
    let key = cache.generate_key("Hello", "en-US", "male");
    
    let audio_buffer = AudioBuffer {
        samples: vec![0i16; 1000],
        sample_rate: 44100,
        channels: 1,
        duration_ms: 100,
    };
    
    cache.set(&key, audio_buffer).await;
    assert!(cache.get(&key).await.is_some());
    
    cache.clear().await;
    assert!(cache.get(&key).await.is_none());
}

#[tokio::test]
async fn test_tts_cache_stats() {
    let cache = TTSCacheManager::new(100, Duration::from_secs(3600));
    
    let key = cache.generate_key("Hello", "en-US", "male");
    let audio_buffer = AudioBuffer {
        samples: vec![0i16; 1000],
        sample_rate: 44100,
        channels: 1,
        duration_ms: 100,
    };
    
    cache.set(&key, audio_buffer).await;
    cache.get(&key).await; // Hit
    cache.get(&key).await; // Hit
    cache.generate_key("Miss", "en-US", "male"); // Miss (not set)
    
    let stats = cache.stats().await;
    assert_eq!(stats.hits, 2);
    assert_eq!(stats.misses, 1);
    assert_eq!(stats.size, 1);
}
