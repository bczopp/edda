//! Optional TUI (Terminal UI) for Ragnarok – Phase 5.
//! Status-Dashboard and Chat-Interface using ratatui + crossterm.

mod manager;
mod state;

pub use manager::TuiManager;
pub use state::TuiState;
