//! Integration tests for Port Forwarding Configurator (Phase 13.4 – NAT Traversal Fallback).
//!
//! - Manual / automatic port forwarding configuration
//! Uses PortForwardingConfiguratorStub for container-friendly tests.

use bifrost::nat::{PortForwardingConfigurator, PortForwardingConfiguratorStub};
use std::sync::Arc;

#[tokio::test]
async fn test_port_forwarding_stub_configure_success() {
    let stub = PortForwardingConfiguratorStub::new(true);
    let configurator = PortForwardingConfigurator::new(Arc::new(stub));
    let ok = configurator.try_configure_forward(9000, None).await.unwrap();
    assert!(ok);
}

#[tokio::test]
async fn test_port_forwarding_stub_configure_failure() {
    let stub = PortForwardingConfiguratorStub::new(false);
    let configurator = PortForwardingConfigurator::new(Arc::new(stub));
    let ok = configurator.try_configure_forward(9000, None).await.unwrap();
    assert!(!ok);
}

#[tokio::test]
async fn test_port_forwarding_stub_remove_forward() {
    let stub = PortForwardingConfiguratorStub::new(true);
    let configurator = PortForwardingConfigurator::new(Arc::new(stub));
    let ok = configurator.remove_forward(9000).await.unwrap();
    assert!(ok);
}

#[tokio::test]
async fn test_port_forwarding_stub_with_protocol() {
    let stub = PortForwardingConfiguratorStub::new(true);
    let configurator = PortForwardingConfigurator::new(Arc::new(stub));
    let ok = configurator
        .try_configure_forward(3478, Some("udp".to_string()))
        .await
        .unwrap();
    assert!(ok);
}
