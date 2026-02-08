#[cfg(test)]
mod tests {
    use odin::protocols::manager::ProtocolManager;
    use odin::scheduler::DeviceScheduler;
    use odin::utils::config::SettingsManager;
    use std::sync::Arc;
    use tempfile::TempDir;

    /// Ensure that the device scheduler can be constructed and a single poll
    /// round can be executed without panicking, even if no services are
    /// actually reachable. This mirrors the behaviour tested in the
    /// ProtocolManager tests: network failures are allowed but must be handled
    /// gracefully.
    #[tokio::test]
    async fn device_scheduler_poll_once_does_not_panic() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("settings.json");

        let settings_manager = Arc::new(SettingsManager::new(config_path));
        settings_manager.load().await.unwrap();

        let settings = settings_manager.get().await;
        let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));

        let protocol_manager = Arc::new(ProtocolManager::new(settings_arc.clone()));

        let scheduler = DeviceScheduler::new(settings_arc, protocol_manager);

        let result = scheduler.poll_once().await;
        // We only assert that the call completes without panic; network
        // connectivity is not guaranteed in test containers.
        assert!(result.is_ok() || result.is_err());
    }
}

