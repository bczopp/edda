//! Watch-Folder-Manager: nutzt notify für Ordnerüberwachung und Event-Handling.

use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::{Receiver, Sender};
use thiserror::Error;

/// Einzelnes Watch-Event (neue, geänderte oder gelöschte Datei/Ordner).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WatchEvent {
    Created(PathBuf),
    Modified(PathBuf),
    Removed(PathBuf),
}

impl WatchEvent {
    pub fn path(&self) -> &PathBuf {
        match self {
            WatchEvent::Created(p) | WatchEvent::Modified(p) | WatchEvent::Removed(p) => p,
        }
    }
}

#[derive(Debug, Error)]
pub enum WatchError {
    #[error("watch error: {0}")]
    Watch(String),
}

/// Überwacht einen Ordner und liefert Events über einen Kanal (Create, Modify, Remove).
///
/// Nutzt `notify` für plattformübergreifende Dateisystem-Überwachung.
///
/// # Beispiel
///
/// ```no_run
/// # use freki::watch::{WatchFolderManager, WatchEvent};
/// # use std::path::Path;
/// let (mut manager, mut rx) = WatchFolderManager::new();
/// manager.watch(Path::new("/path/to/watch"), true)?; // recursive
///
/// // Events in separatem Thread lesen
/// std::thread::spawn(move || {
///     while let Ok(ev) = rx.recv() {
///         match ev {
///             WatchEvent::Created(p) => println!("Created: {:?}", p),
///             WatchEvent::Modified(p) => println!("Modified: {:?}", p),
///             WatchEvent::Removed(p) => println!("Removed: {:?}", p),
///         }
///     }
/// });
/// ```
pub struct WatchFolderManager {
    _watcher: RecommendedWatcher,
    _sender: Sender<WatchEvent>,
}

impl WatchFolderManager {
    /// Erstellt einen neuen Manager und einen Empfänger für Events.
    /// Der Sender bleibt im Manager; der Aufrufer nutzt `rx` zum Lesen der Events.
    pub fn new() -> (Self, Receiver<WatchEvent>) {
        let (tx, rx) = std::sync::mpsc::channel();
        let tx_clone = tx.clone();

        let watcher = notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
            if let Ok(event) = res {
                for path in &event.paths {
                    let watch_ev = match &event.kind {
                        EventKind::Create(_) => WatchEvent::Created(path.clone()),
                        EventKind::Modify(_) => WatchEvent::Modified(path.clone()),
                        EventKind::Remove(_) => WatchEvent::Removed(path.clone()),
                        _ => continue,
                    };
                    let _ = tx_clone.send(watch_ev);
                }
            }
        })
        .expect("create watcher");

        (
            Self {
                _watcher: watcher,
                _sender: tx,
            },
            rx,
        )
    }

    /// Überwacht den angegebenen Pfad. Bei `recursive == true` inkl. Unterordner.
    pub fn watch(&mut self, path: &Path, recursive: bool) -> Result<(), WatchError> {
        let mode = if recursive {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        self._watcher
            .watch(path, mode)
            .map_err(|e| WatchError::Watch(e.to_string()))
    }
}
