//! Tests for ServiceCoordinator and ScriptRouter (Phase 9.1.1, 9.2.1).

use loki::coordination::{route, ScriptRoute, ServiceCoordinator};

#[test]
fn script_router_detects_apis() {
    let r = route("fenrir:gpio_read(1)");
    assert!(r.needs_fenrir());
    assert!(!r.needs_jormungandr());
    assert!(!r.needs_hel());

    let r = route("jormungandr:http_get(\"x\")");
    assert!(!r.needs_fenrir());
    assert!(r.needs_jormungandr());
    assert!(!r.needs_hel());

    let r = route("hel:storage_get(\"k\")");
    assert!(!r.needs_fenrir());
    assert!(!r.needs_jormungandr());
    assert!(r.needs_hel());
}

#[tokio::test]
async fn coordinator_execute_script_without_apis() {
    let coord = ServiceCoordinator::new().unwrap();
    let result = coord.execute_script("return 42").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "42");
}

#[tokio::test]
async fn coordinator_execute_script_with_hel() {
    let coord = ServiceCoordinator::new().unwrap();
    let script = r#"
        hel:storage_set("k", "v")
        return hel:storage_get("k")
    "#;
    let result = coord.execute_script(script).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "v");
}

#[tokio::test]
async fn coordinator_health_check() {
    let coord = ServiceCoordinator::new().unwrap();
    let (f, j, h) = coord.health_check();
    assert!(f);
    assert!(j);
    assert!(h);
}
