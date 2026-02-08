//! Model management: registry, versioning, health (Phase 1.1.2, 6.1.1, 6.1.2, 6.2.1).

mod health;
mod info;
mod registry;
mod sqlx_registry;
mod discovery;

pub use health::{ModelHealthChecker, ModelHealthProbe, ModelHealthStatus};
pub use info::{ModelInfo, ModelType};
pub use registry::ModelRegistry;
pub use sqlx_registry::{ModelRegistryTrait, SqlxModelRegistry};
pub use discovery::{DiscoveryError, EinherjarCapabilityClient, ModelDiscovery};
