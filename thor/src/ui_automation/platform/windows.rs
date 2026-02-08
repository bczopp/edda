//! Windows-specific UI automation.
//!
//! Implementation lives in `handler.rs` (`windows_impl` module, compiled only on `target_os = "windows"`).
//! Uses `windows` crate: SetCursorPos, SendInput (mouse click, KEYEVENTF_UNICODE for typing).
