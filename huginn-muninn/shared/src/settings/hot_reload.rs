//! Hot-reload for settings (notify + Arc<RwLock<Settings>>)

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;
use tracing::info;

use crate::settings::error::SettingsError;
use crate::settings::loader::{load_huginn_settings, load_muninn_settings};
use crate::settings::schema::{HuginnSettings, MuninnSettings};

/// Shared handle for hot-reloaded Huginn settings.
pub type HuginnSettingsHandle = Arc<RwLock<HuginnSettings>>;

/// Shared handle for hot-reloaded Muninn settings.
pub type MuninnSettingsHandle = Arc<RwLock<MuninnSettings>>;

/// Handle to stop hot-reload watcher.
pub struct HotReloadHandle {
    _guard: Option<std::thread::JoinHandle<()>>,
}

/// Start hot-reload for Huginn settings: watch path and reload on change.
/// Returns (handle to current settings, stop handle).
pub fn start_huginn_hot_reload(
    path: PathBuf,
) -> Result<(HuginnSettingsHandle, HotReloadHandle), SettingsError> {
    let initial = load_huginn_settings(&path)?;
    let handle = Arc::new(RwLock::new(initial));
    let handle_clone = Arc::clone(&handle);
    let path_clone = path.clone();
    let thread = std::thread::spawn(move || {
        let mut last_mod = std::time::SystemTime::UNIX_EPOCH;
        loop {
            if let Ok(meta) = std::fs::metadata(&path_clone) {
                if let Ok(mtime) = meta.modified() {
                    if mtime > last_mod {
                        last_mod = mtime;
                        match load_huginn_settings(&path_clone) {
                            Ok(s) => {
                                if let Ok(mut w) = handle_clone.try_write() {
                                    *w = s;
                                    info!("Huginn settings reloaded from {:?}", path_clone);
                                }
                            }
                            Err(e) => {
                                tracing::warn!("Failed to reload Huginn settings: {}", e);
                            }
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_secs(2));
        }
    });
    Ok((
        handle,
        HotReloadHandle {
            _guard: Some(thread),
        },
    ))
}

/// Start hot-reload for Muninn settings: watch path and reload on change.
pub fn start_muninn_hot_reload(
    path: PathBuf,
) -> Result<(MuninnSettingsHandle, HotReloadHandle), SettingsError> {
    let initial = load_muninn_settings(&path)?;
    let handle = Arc::new(RwLock::new(initial));
    let handle_clone = Arc::clone(&handle);
    let path_clone = path.clone();
    let thread = std::thread::spawn(move || {
        let mut last_mod = std::time::SystemTime::UNIX_EPOCH;
        loop {
            if let Ok(meta) = std::fs::metadata(&path_clone) {
                if let Ok(mtime) = meta.modified() {
                    if mtime > last_mod {
                        last_mod = mtime;
                        match load_muninn_settings(&path_clone) {
                            Ok(s) => {
                                if let Ok(mut w) = handle_clone.try_write() {
                                    *w = s;
                                    info!("Muninn settings reloaded from {:?}", path_clone);
                                }
                            }
                            Err(e) => {
                                tracing::warn!("Failed to reload Muninn settings: {}", e);
                            }
                        }
                    }
                }
            }
            std::thread::sleep(Duration::from_secs(2));
        }
    });
    Ok((
        handle,
        HotReloadHandle {
            _guard: Some(thread),
        },
    ))
}
