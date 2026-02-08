//! Tests for StorageManager (TDD – Phase 8.3.1).

use hel::StorageManager;
use tempfile::TempDir;

#[test]
fn storage_in_memory_get_set_remove() {
    let s = StorageManager::new_in_memory();
    assert!(s.get("k").unwrap().is_none());
    s.set("k", "v").unwrap();
    assert_eq!(s.get("k").unwrap(), Some("v".into()));
    assert_eq!(s.remove("k").unwrap(), Some("v".into()));
    assert!(s.get("k").unwrap().is_none());
}

#[test]
fn storage_in_memory_keys() {
    let s = StorageManager::new_in_memory();
    s.set("b", "1").unwrap();
    s.set("a", "2").unwrap();
    let keys = s.keys().unwrap();
    assert_eq!(keys, ["a", "b"]);
}

#[test]
fn storage_persistent_roundtrip() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path();
    let s1 = StorageManager::new_persistent(path).unwrap();
    s1.set("x", "y").unwrap();
    drop(s1);
    let s2 = StorageManager::new_persistent(path).unwrap();
    assert_eq!(s2.get("x").unwrap(), Some("y".into()));
}
