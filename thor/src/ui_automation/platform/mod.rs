#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "linux")]
pub mod linux;

/// Operating System types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
    FreeRTOS,
    Zephyr,
    Unknown,
}

/// Device Platform types (device category, not OS)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Smartphone,
    Tablet,
    Laptop,
    Desktop,
    Server,
    IoT,
    ESP32,
    Unknown,
}

/// Operating System detector
pub struct OperatingSystemDetector;

impl OperatingSystemDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect(&self) -> OperatingSystem {
        #[cfg(target_os = "windows")]
        return OperatingSystem::Windows;

        #[cfg(target_os = "macos")]
        return OperatingSystem::MacOS;

        #[cfg(target_os = "linux")]
        return OperatingSystem::Linux;

        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        return OperatingSystem::Unknown;
    }
}

impl Default for OperatingSystemDetector {
    fn default() -> Self {
        Self::new()
    }
}
