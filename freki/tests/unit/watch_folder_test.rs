//! Tests für Watch-Folder-Manager (Phase 8.1.1).

use freki::watch::{WatchEvent, WatchFolderManager};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;

fn wait_for_events(count: &AtomicUsize, expected: usize, timeout_ms: u64) -> bool {
    let deadline = std::time::Instant::now() + Duration::from_millis(timeout_ms);
    while count.load(Ordering::SeqCst) < expected && std::time::Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    count.load(Ordering::SeqCst) >= expected
}

#[test]
fn watch_folder_detects_new_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().to_path_buf();
    let created = Arc::new(AtomicUsize::new(0));
    let created_clone = Arc::clone(&created);

    let (mut manager, mut rx) = WatchFolderManager::new();
    let _ = manager.watch(&path, true);
    std::thread::spawn(move || {
        while let Ok(ev) = rx.recv() {
            if matches!(ev, WatchEvent::Created(_)) {
                created_clone.fetch_add(1, Ordering::SeqCst);
            }
        }
    });

    std::fs::write(dir.path().join("new.txt"), "content").unwrap();
    assert!(
        wait_for_events(&created, 1, 2000),
        "expected at least one Create event"
    );
}

#[test]
fn watch_folder_detects_modified_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().to_path_buf();
    let file = dir.path().join("m.txt");
    std::fs::write(&file, "initial").unwrap();

    let modified = Arc::new(AtomicUsize::new(0));
    let modified_clone = Arc::clone(&modified);

    let (mut manager, mut rx) = WatchFolderManager::new();
    let _ = manager.watch(&path, true);
    std::thread::spawn(move || {
        while let Ok(ev) = rx.blocking_recv() {
            if matches!(ev, WatchEvent::Modified(_)) {
                modified_clone.fetch_add(1, Ordering::SeqCst);
            }
        }
    });

    std::thread::sleep(Duration::from_millis(100));
    std::fs::write(&file, "updated").unwrap();
    assert!(
        wait_for_events(&modified, 1, 2000),
        "expected at least one Modify event"
    );
}

#[test]
fn watch_folder_detects_removed_file() {
    let dir = TempDir::new().unwrap();
    let path = dir.path().to_path_buf();
    let file = dir.path().join("r.txt");
    std::fs::write(&file, "x").unwrap();

    let removed = Arc::new(AtomicUsize::new(0));
    let removed_clone = Arc::clone(&removed);

    let (mut manager, mut rx) = WatchFolderManager::new();
    let _ = manager.watch(&path, true);
    std::thread::spawn(move || {
        while let Ok(ev) = rx.recv() {
            if matches!(ev, WatchEvent::Removed(_)) {
                removed_clone.fetch_add(1, Ordering::SeqCst);
            }
        }
    });

    std::thread::sleep(Duration::from_millis(100));
    std::fs::remove_file(&file).unwrap();
    assert!(
        wait_for_events(&removed, 1, 2000),
        "expected at least one Remove event"
    );
}

#[test]
fn watch_event_path_returns_pathbuf() {
    let p = PathBuf::from("/tmp/foo.txt");
    let ev = WatchEvent::Created(p.clone());
    assert!(matches!(ev, WatchEvent::Created(_)));
    if let WatchEvent::Created(path) = ev {
        assert_eq!(path, PathBuf::from("/tmp/foo.txt"));
    }
}
