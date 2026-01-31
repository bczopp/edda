//! Device discovery: mDNS/Bonjour, IP-based, Yggdrasil.
//! See IMPLEMENTATION_PLAN Phase 8.

pub mod ip;
pub mod yggdrasil;

pub use ip::{build_ws_url, IpConnectionManager};
pub use yggdrasil::{
    DeviceInfo, YggdrasilDiscoveryClient, YggdrasilDiscoveryProvider, YggdrasilDiscoveryStub,
};
