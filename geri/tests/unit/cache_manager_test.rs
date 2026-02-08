//! Tests für Cache-Manager (Phase 11.1.1).

#[cfg(test)]
mod tests {
    use geri::cache::CacheManager;
    use std::time::Duration;

    #[test]
    fn get_returns_none_for_unknown_prompt() {
        let mut cache = CacheManager::new(Duration::from_secs(60));
        assert!(cache.get("unknown prompt").is_none());
    }

    #[test]
    fn insert_and_get_returns_cached_response() {
        let mut cache = CacheManager::new(Duration::from_secs(60));
        cache.insert("Hello", "Hi there!".to_string());
        assert_eq!(cache.get("Hello"), Some("Hi there!".to_string()));
    }

    #[test]
    fn same_prompt_same_key() {
        let mut cache = CacheManager::new(Duration::from_secs(60));
        cache.insert("same", "first".to_string());
        assert_eq!(cache.get("same"), Some("first".to_string()));
        cache.insert("same", "second".to_string());
        assert_eq!(cache.get("same"), Some("second".to_string()));
    }

    #[test]
    fn different_prompts_different_entries() {
        let mut cache = CacheManager::new(Duration::from_secs(60));
        cache.insert("A", "resp A".to_string());
        cache.insert("B", "resp B".to_string());
        assert_eq!(cache.get("A"), Some("resp A".to_string()));
        assert_eq!(cache.get("B"), Some("resp B".to_string()));
    }

    #[test]
    fn expired_entry_returns_none() {
        let mut cache = CacheManager::new(Duration::from_secs(0));
        cache.insert("x", "y".to_string());
        assert!(cache.get("x").is_none());
    }

    #[test]
    fn cache_key_deterministic() {
        let mut cache = CacheManager::new(Duration::from_secs(60));
        cache.insert("deterministic", "value".to_string());
        assert_eq!(cache.get("deterministic"), Some("value".to_string()));
        assert_eq!(cache.get("deterministic"), Some("value".to_string()));
    }

    #[test]
    fn invalidate_all_clears_cache() {
        let mut cache = CacheManager::new(Duration::from_secs(60));
        cache.insert("a", "1".to_string());
        cache.insert("b", "2".to_string());
        cache.invalidate_all();
        assert!(cache.get("a").is_none());
        assert!(cache.get("b").is_none());
    }
}
