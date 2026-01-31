//! Plugin system: [`OdinPlugin`] trait and [`PluginManager`] registry.

pub mod manager;
pub mod trait_def;
pub mod grpc_proxy;

pub use manager::*;
pub use trait_def::*;
pub use grpc_proxy::{GrpcPluginProxy, OdinGrpcProcessClient, ProcessClient};
