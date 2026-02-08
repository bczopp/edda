#[cfg(test)]
mod tests {
    use odin::scheduler::device_registry::{DeviceRegistry, LogicalDevice, LogicalDeviceKind};

    #[tokio::test]
    async fn registry_empty_after_new() {
        let registry = DeviceRegistry::new();
        let list = registry.list().await;
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn registry_register_then_get() {
        let registry = DeviceRegistry::new();
        let device = LogicalDevice {
            id: "house-1".to_string(),
            kind: LogicalDeviceKind::House,
            platform_id: "asgard-main".to_string(),
            name: "Wohnzimmer".to_string(),
        };
        registry.register(device.clone()).await;
        let got = registry.get("house-1").await;
        assert!(got.as_ref().is_some_and(|d| d.id == "house-1" && d.platform_id == "asgard-main"));
    }

    #[tokio::test]
    async fn registry_register_then_list() {
        let registry = DeviceRegistry::new();
        registry
            .register(LogicalDevice {
                id: "vehicle-1".to_string(),
                kind: LogicalDeviceKind::Vehicle,
                platform_id: "midgard-car".to_string(),
                name: "Auto".to_string(),
            })
            .await;
        registry
            .register(LogicalDevice {
                id: "robot-1".to_string(),
                kind: LogicalDeviceKind::Robot,
                platform_id: "midgard-lab".to_string(),
                name: "Greifer".to_string(),
            })
            .await;
        let list = registry.list().await;
        assert_eq!(list.len(), 2);
    }

    #[tokio::test]
    async fn registry_get_unknown_returns_none() {
        let registry = DeviceRegistry::new();
        let got = registry.get("unknown").await;
        assert!(got.is_none());
    }
}
