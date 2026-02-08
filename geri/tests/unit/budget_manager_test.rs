//! Tests für Budget-Manager (Phase 9.3.1).

#[cfg(test)]
mod tests {
    use geri::cost::{BudgetAlert, BudgetManager};

    #[test]
    fn budget_starts_zero_usage() {
        let mgr = BudgetManager::new(100.0);
        assert_eq!(mgr.get_usage(), 0.0);
        assert_eq!(mgr.get_limit(), 100.0);
    }

    #[test]
    fn add_usage_tracks_correctly() {
        let mgr = BudgetManager::new(100.0);
        let mgr = mgr.add_usage(10.0).add_usage(5.0);
        assert_eq!(mgr.get_usage(), 15.0);
    }

    #[test]
    fn is_over_limit_true_when_exceeds() {
        let mgr = BudgetManager::new(10.0).add_usage(15.0);
        assert!(mgr.is_over_limit());
    }

    #[test]
    fn is_over_limit_false_when_under() {
        let mgr = BudgetManager::new(100.0).add_usage(50.0);
        assert!(!mgr.is_over_limit());
    }

    #[test]
    fn remaining_returns_correct() {
        let mgr = BudgetManager::new(100.0).add_usage(30.0);
        assert_eq!(mgr.remaining(), 70.0);
    }

    #[test]
    fn check_alerts_when_over_limit() {
        let mgr = BudgetManager::new(10.0).add_usage(12.0);
        let alerts = mgr.check_alerts();
        assert!(alerts.iter().any(|a| matches!(a, BudgetAlert::OverLimit(_))));
    }

    #[test]
    fn check_alerts_when_near_limit() {
        let mgr = BudgetManager::new(100.0).add_usage(85.0);
        let alerts = mgr.check_alerts();
        assert!(alerts.iter().any(|a| matches!(a, BudgetAlert::NearLimit(_, _))));
    }
}
