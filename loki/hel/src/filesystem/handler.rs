//! Filesystem handler – read, write, delete, directory ops (Phase 8.2.1).
//! Implementation follows TDD; tests in hel/tests/filesystem_handler_test.rs.

use std::path::Path;

use crate::error::{HelError, Result};

/// Filesystem handler: file read/write/delete, directory operations.
pub struct FilesystemHandler {
    base_path: std::path::PathBuf,
}

impl FilesystemHandler {
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let base = base_path.as_ref().to_path_buf();
        if !base.exists() {
            std::fs::create_dir_all(&base).map_err(HelError::Filesystem)?;
        }
        Ok(Self { base_path: base })
    }

    fn resolve(&self, path: &str) -> Result<std::path::PathBuf> {
        let path = path.trim_start_matches('/').trim_start_matches('\\');
        if path.contains("..") || Path::new(path).has_root() {
            return Err(HelError::Filesystem(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                "path escape",
            )));
        }
        Ok(self.base_path.join(path))
    }

    /// Read file contents as string.
    pub fn read(&self, path: &str) -> Result<String> {
        let p = self.resolve(path)?;
        std::fs::read_to_string(&p).map_err(HelError::Filesystem)
    }

    /// Write string to file.
    pub fn write(&self, path: &str, contents: &str) -> Result<()> {
        let p = self.resolve(path)?;
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).map_err(HelError::Filesystem)?;
        }
        std::fs::write(&p, contents).map_err(HelError::Filesystem)
    }

    /// Delete file or empty directory.
    pub fn delete(&self, path: &str) -> Result<()> {
        let p = self.resolve(path)?;
        if p.is_file() {
            std::fs::remove_file(&p).map_err(HelError::Filesystem)
        } else if p.is_dir() {
            std::fs::remove_dir(&p).map_err(HelError::Filesystem)
        } else {
            Err(HelError::Filesystem(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "not found",
            )))
        }
    }

    /// List directory entries (names only).
    pub fn list_dir(&self, path: &str) -> Result<Vec<String>> {
        let p = self.resolve(path)?;
        let mut names = Vec::new();
        for e in std::fs::read_dir(&p).map_err(HelError::Filesystem)? {
            let e = e.map_err(HelError::Filesystem)?;
            names.push(
                e.file_name()
                    .to_string_lossy()
                    .to_string(),
            );
        }
        names.sort();
        Ok(names)
    }

    /// Create directory (and parents).
    pub fn create_dir(&self, path: &str) -> Result<()> {
        let p = self.resolve(path)?;
        std::fs::create_dir_all(&p).map_err(HelError::Filesystem)
    }
}
