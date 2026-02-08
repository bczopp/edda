//! Tests für Key-Rotation-Manager (Phase 14.2.1).

#[cfg(test)]
mod tests {
    use geri::keys::{InMemoryKeyBackend, KeyRotationManager, SecureKeyStorage};

    #[test]
    fn rotate_replaces_old_key() {
        let backend = InMemoryKeyBackend::new();
        let mut storage = SecureKeyStorage::new(Box::new(backend));
        storage.store_key("openai", "old-key").unwrap();
        let manager = KeyRotationManager;
        manager
            .rotate(&mut storage, "openai", "new-key")
            .unwrap();
        assert_eq!(storage.load_key("openai").unwrap().as_deref(), Some("new-key"));
    }

    #[test]
    fn rotate_works_when_no_previous_key() {
        let backend = InMemoryKeyBackend::new();
        let mut storage = SecureKeyStorage::new(Box::new(backend));
        let manager = KeyRotationManager;
        manager.rotate(&mut storage, "openai", "first-key").unwrap();
        assert_eq!(storage.load_key("openai").unwrap().as_deref(), Some("first-key"));
    }

    #[test]
    fn rotate_does_not_affect_other_providers() {
        let backend = InMemoryKeyBackend::new();
        let mut storage = SecureKeyStorage::new(Box::new(backend));
        storage.store_key("anthropic", "other").unwrap();
        let manager = KeyRotationManager;
        manager.rotate(&mut storage, "openai", "new").unwrap();
        assert_eq!(
            storage.load_key("anthropic").unwrap().as_deref(),
            Some("other")
        );
    }
}
