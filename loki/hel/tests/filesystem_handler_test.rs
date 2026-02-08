//! Tests for FilesystemHandler (TDD – Phase 8.2.1).

use hel::FilesystemHandler;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn fs_handler_read_write() {
    let tmp = TempDir::new().unwrap();
    let base = tmp.path();
    let handler = FilesystemHandler::new(base).unwrap();

    handler.write("a.txt", "hello").unwrap();
    let s = handler.read("a.txt").unwrap();
    assert_eq!(s, "hello");
}

#[test]
fn fs_handler_delete_file() {
    let tmp = TempDir::new().unwrap();
    let base = tmp.path();
    let handler = FilesystemHandler::new(base).unwrap();

    handler.write("b.txt", "x").unwrap();
    assert!(handler.read("b.txt").is_ok());
    handler.delete("b.txt").unwrap();
    assert!(handler.read("b.txt").is_err());
}

#[test]
fn fs_handler_list_dir() {
    let tmp = TempDir::new().unwrap();
    let base = tmp.path();
    let handler = FilesystemHandler::new(base).unwrap();

    handler.write("f1", "1").unwrap();
    handler.write("f2", "2").unwrap();
    handler.create_dir("sub").unwrap();
    let names = handler.list_dir(".").unwrap();
    assert!(names.contains(&"f1".into()));
    assert!(names.contains(&"f2".into()));
    assert!(names.contains(&"sub".into()));
}

#[test]
fn fs_handler_path_escape_rejected() {
    let tmp = TempDir::new().unwrap();
    let base = tmp.path();
    let handler = FilesystemHandler::new(base).unwrap();

    assert!(handler.read("..").is_err());
    assert!(handler.read("sub/../..").is_err());
}
