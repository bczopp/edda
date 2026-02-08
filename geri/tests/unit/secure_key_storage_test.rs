//! Tests für Secure-Key-Storage (Phase 14.1.1).

#[cfg(test)]
mod tests {
    use geri::keys::{InMemoryKeyBackend, SecureKeyStorage};

    #[test]
    fn store_and_load_key() {
        let backend = InMemoryKeyBackend::new();
        let mut storage = SecureKeyStorage::new(Box::new(backend));
        storage.store_key("openai", "sk-secret").unwrap();
        let loaded = storage.load_key("openai").unwrap();
        assert_eq!(loaded.as_deref(), Some("sk-secret"));
    }

    #[test]
    fn load_missing_key_returns_none() {
        let backend = InMemoryKeyBackend::new();
        let storage = SecureKeyStorage::new(Box::new(backend));
        assert!(storage.load_key("unknown").unwrap().is_none());
    }

    #[test]
    fn store_overwrites_existing() {
        let backend = InMemoryKeyBackend::new();
        let mut storage = SecureKeyStorage::new(Box::new(backend));
        storage.store_key("provider", "old").unwrap();
        storage.store_key("provider", "new").unwrap();
        assert_eq!(storage.load_key("provider").unwrap().as_deref(), Some("new"));
    }

    #[test]
    fn different_providers_separate_keys() {
        let backend = InMemoryKeyBackend::new();
        let mut storage = SecureKeyStorage::new(Box::new(backend));
        storage.store_key("openai", "key1").unwrap();
        storage.store_key("anthropic", "key2").unwrap();
        assert_eq!(storage.load_key("openai").unwrap().as_deref(), Some("key1"));
        assert_eq!(storage.load_key("anthropic").unwrap().as_deref(), Some("key2"));
    }
}
