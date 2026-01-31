pub mod platform;
pub mod handler;

pub use platform::{OperatingSystem, OperatingSystemDetector, Platform};
pub use handler::*;
