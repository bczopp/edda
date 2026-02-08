//! Hel – Data/Storage Service (Loki sub-service).
//! Filesystem, key-value storage, cache; script-accessible via Loki.

pub mod cache;
pub mod error;
pub mod filesystem;
pub mod script;
pub mod storage;

pub use cache::CacheManager;
pub use error::{HelError, Result};
pub use filesystem::FilesystemHandler;
pub use script::HelScriptAPI;
pub use storage::StorageManager;
