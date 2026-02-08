//! Model selection: multi-factor evaluation, load balancing (Phase 1.1.2, 7.1.1, 7.2.1, 7.3.1).

mod efficiency;
mod load_balancer;
mod selector;
mod dynamic_calculator;

pub use efficiency::{EfficiencyInput, EfficiencyScoreCalculator, EfficiencyWeights};
pub use dynamic_calculator::DynamicEfficiencyCalculator;
pub use load_balancer::LoadBalancer;
pub use selector::{ModelSelector, SelectionOptions};