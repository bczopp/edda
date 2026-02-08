//! Hel script API – Lua bindings for filesystem, storage, cache (Phase 8.5.1).

use std::sync::Arc;

use mlua::{UserData, UserDataMethods};

use crate::cache::CacheManager;
use crate::error::Result;
use crate::filesystem::FilesystemHandler;
use crate::storage::StorageManager;

/// Script API exposed to Lua: fs_*, storage_*, cache_*.
pub struct HelScriptAPI {
    fs: Arc<FilesystemHandler>,
    storage: Arc<StorageManager>,
    cache: Arc<CacheManager>,
}

impl HelScriptAPI {
    pub fn new(
        fs: Arc<FilesystemHandler>,
        storage: Arc<StorageManager>,
        cache: Arc<CacheManager>,
    ) -> Self {
        Self { fs, storage, cache }
    }

    /// Register this API as global "hel" in the given Lua state.
    pub fn register_into(&self, lua: &mlua::Lua) -> Result<()> {
        lua.globals()
            .set("hel", self.clone_for_lua())
            .map_err(|e| crate::error::HelError::NotAvailable(e.to_string()))?;
        Ok(())
    }

    fn clone_for_lua(&self) -> HelScriptAPI {
        HelScriptAPI {
            fs: Arc::clone(&self.fs),
            storage: Arc::clone(&self.storage),
            cache: Arc::clone(&self.cache),
        }
    }
}

impl Clone for HelScriptAPI {
    fn clone(&self) -> Self {
        Self {
            fs: Arc::clone(&self.fs),
            storage: Arc::clone(&self.storage),
            cache: Arc::clone(&self.cache),
        }
    }
}

impl UserData for HelScriptAPI {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("fs_read", |_, this, path: String| {
            this.fs.read(&path).map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("fs_write", |_, this, (path, contents): (String, String)| {
            this.fs.write(&path, &contents).map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("fs_delete", |_, this, path: String| {
            this.fs.delete(&path).map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("fs_list_dir", |_, this, path: String| {
            this.fs.list_dir(&path).map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("fs_create_dir", |_, this, path: String| {
            this.fs.create_dir(&path).map_err(|e| mlua::Error::external(e))
        });

        methods.add_method("storage_get", |_, this, key: String| {
            this.storage.get(&key).map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("storage_set", |_, this, (key, value): (String, String)| {
            this.storage.set(&key, &value).map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("storage_remove", |_, this, key: String| {
            this.storage.remove(&key).map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("storage_keys", |_, this, ()| {
            this.storage.keys().map_err(|e| mlua::Error::external(e))
        });

        methods.add_method("cache_get", |_, this, key: String| {
            this.cache.get(&key).map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("cache_set", |_, this, (key, value): (String, String)| {
            this.cache.set(&key, &value).map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("cache_invalidate", |_, this, key: String| {
            this.cache.invalidate(&key).map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("cache_invalidate_all", |_, this, ()| {
            this.cache.invalidate_all().map_err(|e| mlua::Error::external(e))
        });
    }
}
