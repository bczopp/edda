//! Version negotiation (Phase 2.2.1–2.2.2): Semantic Versioning, highest common version, backward-compat.

use std::cmp::Ordering;
use thiserror::Error;

/// Semantic version (Major.Minor.Patch) for Bifrost protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProtocolVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ProtocolVersion {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }
}

impl PartialOrd for ProtocolVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ProtocolVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        self.major
            .cmp(&other.major)
            .then(self.minor.cmp(&other.minor))
            .then(self.patch.cmp(&other.patch))
    }
}

/// Error when client and server have no common protocol version (Phase 2.2.2).
#[derive(Debug, Error)]
pub enum VersionMismatchError {
    #[error("no common protocol version between client and server")]
    NoCommonVersion,
}

/// Selects the highest protocol version supported by both client and server.
pub struct VersionNegotiator;

impl VersionNegotiator {
    /// Returns the highest version that appears in both `client_versions` and `server_versions`.
    pub fn select_highest_common(
        client_versions: &[ProtocolVersion],
        server_versions: &[ProtocolVersion],
    ) -> Option<ProtocolVersion> {
        let server_set: std::collections::HashSet<ProtocolVersion> = server_versions.iter().copied().collect();
        client_versions
            .iter()
            .filter(|v| server_set.contains(v))
            .max()
            .copied()
    }

    /// Negotiates a common version or returns an error (Phase 2.2.2 backward-compat / major-mismatch).
    pub fn negotiate(
        client_versions: &[ProtocolVersion],
        server_versions: &[ProtocolVersion],
    ) -> Result<ProtocolVersion, VersionMismatchError> {
        Self::select_highest_common(client_versions, server_versions)
            .ok_or(VersionMismatchError::NoCommonVersion)
    }

    /// Returns the list of protocol versions this implementation supports (for connection establishment).
    pub fn supported_versions() -> Vec<ProtocolVersion> {
        vec![ProtocolVersion::new(1, 0, 0)]
    }
}
