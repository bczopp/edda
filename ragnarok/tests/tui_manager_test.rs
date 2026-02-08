//! Tests for TUIManager (TDD – Phase 5 Optional TUI)
//! Tests state and API only; no terminal/raw mode in tests.

use ragnarok::tui::{TuiManager, TuiState};

#[test]
fn tui_state_default_creation() {
    let state = TuiState::default();
    assert!(state.status_lines().is_empty());
    assert!(state.chat_messages().is_empty());
    assert!(state.input_buffer().is_empty());
    assert!(!state.odin_connected());
}

#[test]
fn tui_state_add_status_line() {
    let mut state = TuiState::default();
    state.add_status_line("Odin: connected");
    state.add_status_line("Thor: available");
    let lines = state.status_lines();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0], "Odin: connected");
    assert_eq!(lines[1], "Thor: available");
}

#[test]
fn tui_state_add_chat_message() {
    let mut state = TuiState::default();
    state.add_chat_message("user", "Hello");
    state.add_chat_message("assistant", "Hi there");
    let msgs = state.chat_messages();
    assert_eq!(msgs.len(), 2);
    assert_eq!(msgs[0].0, "user");
    assert_eq!(msgs[0].1, "Hello");
    assert_eq!(msgs[1].0, "assistant");
    assert_eq!(msgs[1].1, "Hi there");
}

#[test]
fn tui_state_set_input_buffer() {
    let mut state = TuiState::default();
    assert!(state.input_buffer().is_empty());
    state.set_input_buffer("type here");
    assert_eq!(state.input_buffer(), "type here");
}

#[test]
fn tui_state_set_odin_connected() {
    let mut state = TuiState::default();
    assert!(!state.odin_connected());
    state.set_odin_connected(true);
    assert!(state.odin_connected());
}

#[test]
fn tui_manager_creation() {
    let state = TuiState::default();
    let _manager = TuiManager::new(state);
    // TUIManager holds state; no terminal in test
}
