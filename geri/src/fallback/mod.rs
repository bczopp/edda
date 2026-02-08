//! Fallback-System (Phase 10.1, 10.2, 10.3): Cloud-to-Local Fallback, Notifications, Budget-Reset.

mod budget_reset;
mod manager;
mod notification;
pub use budget_reset::{BudgetResetHandler, BudgetResetListener};
pub use manager::{CloudLimitDetector, FallbackManager};
pub use notification::{
    FallbackNotificationGenerator, FallbackNotificationReason, NotificationSender,
};
