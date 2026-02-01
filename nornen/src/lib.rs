pub mod decision;
pub mod rules;
pub mod grpc;
pub mod utils;
pub mod urd;
pub mod verdandi;
pub mod coordinator;
pub mod mimir_client;
pub mod cache;
pub mod audit;
pub mod monitoring;
pub mod security;

// Re-export for convenience
pub use urd::registry::{ProviderRegistry, ProviderRegistryError};
pub use verdandi::router::{RequestRouter, RequestRouterError};
pub use coordinator::{
    NornenCoordinator, CoordinationError, CoordinationResult,
    ServiceHealth, ServiceStatus, ProviderStatistics,
};
pub use mimir_client::client::{MimirClient, MimirClientError};