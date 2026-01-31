//! Bifrost protocol: message types, version negotiation, serialization.
//! See IMPLEMENTATION_PLAN Phase 2.

mod version;

pub use version::{ProtocolVersion, VersionMismatchError, VersionNegotiator};
