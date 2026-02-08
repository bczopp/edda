//! TUI application state for Status-Dashboard and Chat-Interface.

use std::sync::{Arc, RwLock};

/// Application state for the TUI (status lines, chat messages, input buffer).
#[derive(Clone, Default)]
pub struct TuiState {
    inner: Arc<RwLock<TuiStateInner>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Chat,
    Status,
    Services,
}

#[derive(Default)]
struct TuiStateInner {
    status_lines: Vec<String>,
    chat_messages: Vec<(String, String)>,
    input_buffer: String,
    odin_connected: bool,
    focus: ActivePanel,
    scroll_offset: u16,
}

impl Default for ActivePanel {
    fn default() -> Self {
        Self::chat()
    }
}

impl ActivePanel {
    pub fn chat() -> Self { Self::Chat }
    pub fn status() -> Self { Self::Status }
    pub fn services() -> Self { Self::Services }
}

impl TuiState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn focus(&self) -> ActivePanel {
        self.inner.read().map(|g| g.focus).unwrap_or(ActivePanel::Chat)
    }

    pub fn set_focus(&self, panel: ActivePanel) {
        if let Ok(mut g) = self.inner.write() {
            g.focus = panel;
        }
    }

    pub fn scroll_offset(&self) -> u16 {
        self.inner.read().map(|g| g.scroll_offset).unwrap_or(0)
    }

    pub fn set_scroll_offset(&self, offset: u16) {
        if let Ok(mut g) = self.inner.write() {
            g.scroll_offset = offset;
        }
    }

    pub fn add_status_line(&self, line: impl Into<String>) {
        if let Ok(mut g) = self.inner.write() {
            g.status_lines.push(line.into());
        }
    }

    pub fn clear_status_lines(&self) {
        if let Ok(mut g) = self.inner.write() {
            g.status_lines.clear();
        }
    }

    pub fn add_chat_message(&self, sender: impl Into<String>, text: impl Into<String>) {
        if let Ok(mut g) = self.inner.write() {
            g.chat_messages.push((sender.into(), text.into()));
        }
    }

    pub fn set_input_buffer(&self, s: impl Into<String>) {
        if let Ok(mut g) = self.inner.write() {
            g.input_buffer = s.into();
        }
    }

    pub fn set_odin_connected(&self, connected: bool) {
        if let Ok(mut g) = self.inner.write() {
            g.odin_connected = connected;
        }
    }

    pub fn status_lines(&self) -> Vec<String> {
        self.inner.read().map(|g| g.status_lines.clone()).unwrap_or_default()
    }

    pub fn chat_messages(&self) -> Vec<(String, String)> {
        self.inner.read().map(|g| g.chat_messages.clone()).unwrap_or_default()
    }

    pub fn input_buffer(&self) -> String {
        self.inner.read().map(|g| g.input_buffer.clone()).unwrap_or_default()
    }

    pub fn odin_connected(&self) -> bool {
        self.inner.read().map(|g| g.odin_connected).unwrap_or(false)
    }
}
