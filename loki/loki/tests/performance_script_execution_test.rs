//! Performance tests for Script-Execution (Phase 14.2.1).

use loki::coordination::ServiceCoordinator;
use std::time::Instant;

#[tokio::test(flavor = "multi_thread")]
async fn performance_script_execution_repeated_succeeds() {
    let coord = ServiceCoordinator::new().unwrap();
    let n = 50;
    let start = Instant::now();
    for i in 0..n {
        let script = format!("return {}", i);
        let r = coord.execute_script(&script).await;
        assert!(r.is_ok(), "iteration {} failed", i);
    }
    let elapsed = start.elapsed();
    assert!(elapsed.as_secs_f64() < 60.0, "50 executions took too long: {:?}", elapsed);
}
