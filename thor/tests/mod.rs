// Test module
#[cfg(test)]
pub mod common;

#[cfg(test)]
mod unit {
    pub mod terminal_test;
    pub mod cross_device_test;
    pub mod ui_automation_test;
    pub mod scheduler_test;
    pub mod jotunheim_test;
}

#[cfg(test)]
mod integration {
    pub mod action_execution_test;
}
