//! Tests für Cache-Invalidator (Phase 11.2.1).

#[cfg(test)]
mod tests {
    use geri::cache::{CacheInvalidator, CacheManager, InvalidationEvent};
    use std::time::Duration;

    #[test]
    fn invalidate_on_event_clears_cache() {
        let mut cache = CacheManager::new(Duration::from_secs(60));
        cache.insert("p", "r".to_string());
        let mut invalidator = CacheInvalidator::new(Duration::from_secs(300));
        invalidator.invalidate_on_event(&mut cache, InvalidationEvent::ModelUpdate);
        assert!(cache.get("p").is_none());
    }

    #[test]
    fn invalidate_on_event_provider_status_clears_cache() {
        let mut cache = CacheManager::new(Duration::from_secs(60));
        cache.insert("x", "y".to_string());
        let mut invalidator = CacheInvalidator::new(Duration::from_secs(300));
        invalidator.invalidate_on_event(&mut cache, InvalidationEvent::ProviderStatusChange);
        assert!(cache.get("x").is_none());
    }

    #[test]
    fn invalidate_on_timeout_clears_cache_when_first_call() {
        let mut cache = CacheManager::new(Duration::from_secs(60));
        cache.insert("a", "b".to_string());
        let mut invalidator = CacheInvalidator::new(Duration::from_secs(300));
        let invalidated = invalidator.invalidate_on_timeout(&mut cache);
        assert!(invalidated);
        assert!(cache.get("a").is_none());
    }

    #[test]
    fn invalidate_on_timeout_does_not_clear_before_timeout() {
        let mut cache = CacheManager::new(Duration::from_secs(60));
        cache.insert("a", "b".to_string());
        let mut invalidator = CacheInvalidator::new(Duration::from_secs(300));
        invalidator.invalidate_on_timeout(&mut cache);
        cache.insert("a", "b2".to_string());
        let invalidated = invalidator.invalidate_on_timeout(&mut cache);
        assert!(!invalidated);
        assert_eq!(cache.get("a"), Some("b2".to_string()));
    }
}
