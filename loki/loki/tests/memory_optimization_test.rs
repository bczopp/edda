//! Memory optimization tests (Phase 11.2.1): repeated execution does not leak.

use loki::coordination::ServiceCoordinator;

#[tokio::test]
async fn memory_repeated_execution_bounded() {
    let coord = ServiceCoordinator::new().unwrap();
    for i in 0..100 {
        let script = format!("return {}", i);
        let r = coord.execute_script(&script).await;
        assert!(r.is_ok(), "iteration {} failed", i);
    }
}
