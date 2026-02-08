//! Load Test Suite (Phase 20.3.1).
//! High-Concurrency, Queue-Backlog, Load-Balancing.

use geri::model::{ModelInfo, ModelType};
use geri::queue::RequestQueueManager;
use geri::selection::{EfficiencyInput, EfficiencyScoreCalculator, LoadBalancer};
use std::sync::Arc;
use tokio::sync::Mutex;

// --- High-Concurrency: parallele Enqueue/Dequeue ---

#[tokio::test]
async fn load_high_concurrency_queue_enqueue_dequeue() {
    let queue: Arc<Mutex<RequestQueueManager<u32>>> =
        Arc::new(Mutex::new(RequestQueueManager::with_capacity(200)));
    let mut handles = vec![];
    for i in 0..100u32 {
        let q = Arc::clone(&queue);
        handles.push(tokio::spawn(async move {
            let mut q = q.lock().await;
            let _ = q.enqueue(i);
        }));
    }
    for h in handles {
        h.await.expect("task");
    }
    let mut q = queue.lock().await;
    assert_eq!(q.len(), 100);
    for _ in 0..50 {
        q.dequeue();
    }
    assert_eq!(q.len(), 50);
}

#[tokio::test]
async fn load_high_concurrency_load_balancer_record_requests() {
    let balancer: Arc<Mutex<LoadBalancer>> = Arc::new(Mutex::new(LoadBalancer::new(0.8, 100)));
    let mut handles = vec![];
    for _ in 0..50 {
        let b = Arc::clone(&balancer);
        handles.push(tokio::spawn(async move {
            let mut b = b.lock().await;
            b.record_request("provider-a");
        }));
    }
    for _ in 0..20 {
        let b = Arc::clone(&balancer);
        handles.push(tokio::spawn(async move {
            let mut b = b.lock().await;
            b.record_request("provider-b");
        }));
    }
    for h in handles {
        h.await.expect("task");
    }
    let b = balancer.lock().await;
    assert!(b.get_load("provider-a") > b.get_load("provider-b"));
}

// --- Queue-Backlog: Kapazität, Full, Dequeue ---

#[test]
fn load_queue_backlog_fill_to_capacity() {
    let mut queue: RequestQueueManager<&str> = RequestQueueManager::with_capacity(5);
    for i in 0..5 {
        assert!(queue.enqueue(if i == 0 { "a" } else { "b" }).is_ok());
    }
    assert_eq!(queue.len(), 5);
    assert_eq!(queue.backlog_len(), 5);
    assert!(queue.enqueue("c").is_err());
    let a = queue.dequeue();
    assert!(a.is_some());
    assert_eq!(queue.len(), 4);
}

#[test]
fn load_queue_backlog_dequeue_fifo() {
    let mut queue: RequestQueueManager<i32> = RequestQueueManager::with_capacity(10);
    queue.enqueue(1).unwrap();
    queue.enqueue(2).unwrap();
    queue.enqueue(3).unwrap();
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.dequeue(), Some(3));
    assert!(queue.is_empty());
}

// --- Load-Balancing: überlasteter Provider wird abgewertet ---

fn make_candidate(id: &str, provider: &str, score_input: EfficiencyInput) -> (ModelInfo, EfficiencyInput) {
    (
        ModelInfo {
            id: id.to_string(),
            name: id.to_string(),
            provider: provider.to_string(),
            model_type: ModelType::Llm,
            parameter_count: Some(8_000_000_000),
            hardware_requirements: None,
            context_window: Some(8192),
        },
        score_input,
    )
}

#[test]
fn load_balancing_overloaded_provider_receives_lower_effective_score() {
    let mut balancer = LoadBalancer::new(0.8, 10);
    let calculator = EfficiencyScoreCalculator::default();
    let input = EfficiencyInput {
        parameter_count: Some(8_000_000_000),
        max_parameter_count: 70_000_000_000,
        hardware_score: 1.0,
        uptime_percentage: Some(100.0),
        error_rate: Some(0.0),
        ping_ms: Some(0),
        max_ping_ms: 1000,
        distance_km: Some(0.0),
        max_distance_km: 10_000.0,
        is_local: true,
        cost_per_token: Some(0.0),
        max_cost_per_token: 0.001,
    };
    let candidates = vec![
        make_candidate("model-a", "provider-a", input.clone()),
        make_candidate("model-b", "provider-b", input),
    ];
    // Ohne Last: beide gleich, erster oder zweiter kann gewählt werden
    let first = balancer.next(&candidates, &calculator);
    assert!(first.is_some());
    // Provider A stark belasten (über 80 %)
    for _ in 0..9 {
        balancer.record_request("provider-a");
    }
    let chosen = balancer.next(&candidates, &calculator);
    assert!(chosen.is_some());
    // model-b (provider-b) sollte gewählt werden, da provider-a über Threshold
    assert_eq!(chosen.unwrap().provider, "provider-b");
}

#[test]
fn load_balancing_load_tracked_per_provider() {
    let mut balancer = LoadBalancer::new(0.8, 100);
    assert_eq!(balancer.get_load("p1"), 0.0);
    balancer.record_request("p1");
    balancer.record_request("p1");
    assert_eq!(balancer.get_load("p1"), 0.02);
    balancer.record_request("p2");
    assert_eq!(balancer.get_load("p2"), 0.01);
}
