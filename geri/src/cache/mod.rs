//! Caching-System (Phase 11.1, 11.2): Response-Caching, Invalidation.

mod invalidator;
mod manager;
pub use invalidator::{CacheInvalidator, InvalidationEvent};
pub use manager::CacheManager;
