//! Tests for HelScriptAPI – Lua bindings (TDD – Phase 8.5.1).

use hel::{CacheManager, FilesystemHandler, HelScriptAPI, StorageManager};
use mlua::Lua;
use std::sync::Arc;
use tempfile::TempDir;

fn api() -> (HelScriptAPI, TempDir) {
    let tmp = TempDir::new().unwrap();
    let base = tmp.path();
    let fs = Arc::new(FilesystemHandler::new(base).unwrap());
    let storage = Arc::new(StorageManager::new_in_memory());
    let cache = Arc::new(CacheManager::new(60));
    let api = HelScriptAPI::new(fs, storage, cache);
    (api, tmp)
}

#[test]
fn hel_script_api_register_and_fs() {
    let (api, _tmp) = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();

    lua.load("hel:fs_write(\"x.txt\", \"hello\")")
        .exec()
        .unwrap();
    let s: String = lua.load("return hel:fs_read(\"x.txt\")").eval().unwrap();
    assert_eq!(s, "hello");
}

#[test]
fn hel_script_api_storage() {
    let (api, _tmp) = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();

    lua.load("hel:storage_set(\"k\", \"v\")").exec().unwrap();
    let v: Option<String> = lua.load("return hel:storage_get(\"k\")").eval().unwrap();
    assert_eq!(v, Some("v".into()));
}

#[test]
fn hel_script_api_cache() {
    let (api, _tmp) = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();

    lua.load("hel:cache_set(\"c\", \"data\")").exec().unwrap();
    let v: Option<String> = lua.load("return hel:cache_get(\"c\")").eval().unwrap();
    assert_eq!(v, Some("data".into()));
}
