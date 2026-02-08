use std::sync::Arc;

use freki::cache::{generate_cache_key, EmbeddingCache, EmbeddingCacheHelper, InMemoryEmbeddingCache};
use freki::embedding::EmbeddingModel;
use tokio::task;

use crate::utils::embedding_generators::TestEmbeddingModel;

#[tokio::test]
async fn test_in_memory_cache_get_set_and_miss() {
    let cache = InMemoryEmbeddingCache::new();
    let key = generate_cache_key("doc-1", "chunk-0", "test-model");
    let value = vec![0.1f32, 0.2, 0.3];

    // Miss vor dem Set
    assert!(cache.get(&key).await.unwrap().is_none());

    cache.set(&key, &value).await.unwrap();

    let retrieved = cache.get(&key).await.unwrap().unwrap();
    assert_eq!(retrieved, value);
}

#[tokio::test]
async fn test_in_memory_cache_invalidate_document() {
    let cache = InMemoryEmbeddingCache::new();

    let key1 = generate_cache_key("doc-1", "chunk-0", "test-model");
    let key2 = generate_cache_key("doc-1", "chunk-1", "test-model");
    let key_other = generate_cache_key("doc-2", "chunk-0", "test-model");

    cache.set(&key1, &[0.1]).await.unwrap();
    cache.set(&key2, &[0.2]).await.unwrap();
    cache.set(&key_other, &[0.3]).await.unwrap();

    cache.invalidate_document("doc-1").await.unwrap();

    assert!(cache.get(&key1).await.unwrap().is_none());
    assert!(cache.get(&key2).await.unwrap().is_none());
    assert!(cache.get(&key_other).await.unwrap().is_some());
}

#[tokio::test]
async fn test_in_memory_cache_clear() {
    let cache = InMemoryEmbeddingCache::new();
    let key = generate_cache_key("doc-1", "chunk-0", "test-model");

    cache.set(&key, &[0.1]).await.unwrap();
    assert!(cache.get(&key).await.unwrap().is_some());

    cache.clear().await.unwrap();
    assert!(cache.get(&key).await.unwrap().is_none());
}

#[tokio::test]
async fn test_cache_key_generation_format() {
    let key = generate_cache_key("doc-1", "chunk-0", "model-x");
    assert_eq!(key, "doc-1_chunk-0_model-x");
}

#[tokio::test]
async fn test_cache_helper_get_or_compute_all_miss_then_hit() {
    let cache = Arc::new(InMemoryEmbeddingCache::new());
    let model = Arc::new(TestEmbeddingModel::default_dimension()) as Arc<dyn EmbeddingModel>;
    let helper = EmbeddingCacheHelper::new(cache.clone(), model.clone());

    let chunks = vec!["alpha".to_string(), "beta".to_string()];

    // Erster Aufruf: alles Miss, Modell wird genutzt.
    let embeddings_first = helper
        .get_or_compute_embeddings("doc-1", &chunks)
        .await
        .unwrap();
    assert_eq!(embeddings_first.len(), 2);

    // Zweiter Aufruf: alles Hit, Modell sollte nicht erneut aufgerufen werden (implizit über Gleichheit).
    let embeddings_second = helper
        .get_or_compute_embeddings("doc-1", &chunks)
        .await
        .unwrap();

    assert_eq!(embeddings_first, embeddings_second);
}

#[tokio::test]
async fn test_cache_helper_get_or_compute_partial_hit() {
    let cache = Arc::new(InMemoryEmbeddingCache::new());
    let model = Arc::new(TestEmbeddingModel::default_dimension()) as Arc<dyn EmbeddingModel>;
    let helper = EmbeddingCacheHelper::new(cache.clone(), model.clone());

    let chunks = vec!["alpha".to_string(), "beta".to_string(), "gamma".to_string()];

    // Nur den ersten Chunk vorab in den Cache legen.
    let model_name = model.get_model_name();
    let key0 = generate_cache_key("doc-1", "chunk-0", model_name);
    let embed0 = model.embed_text(&chunks[0]).await.unwrap();
    cache.set(&key0, &embed0).await.unwrap();

    let embeddings = helper
        .get_or_compute_embeddings("doc-1", &chunks)
        .await
        .unwrap();

    assert_eq!(embeddings.len(), 3);
    // Der erste Eintrag muss exakt das vorberechnete Embedding sein.
    assert_eq!(embeddings[0], embed0);
}

#[tokio::test]
async fn test_in_memory_cache_concurrent_access() {
    let cache = Arc::new(InMemoryEmbeddingCache::new());
    let key = generate_cache_key("doc-1", "chunk-0", "test-model");

    let cache_writer = cache.clone();
    let writer = task::spawn(async move {
        for i in 0..10 {
            cache_writer.set(&key, &[i as f32]).await.unwrap();
        }
    });

    let cache_reader = cache.clone();
    let reader = task::spawn(async move {
        // Nur prüfen, dass keine Panics auftreten und Zugriffe funktionieren.
        for _ in 0..10 {
            let _ = cache_reader.get(&key).await.unwrap();
        }
    });

    writer.await.unwrap();
    reader.await.unwrap();
}

