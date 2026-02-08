//! TUIManager – Status-Dashboard and Chat-Interface (ratatui + crossterm).

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io::{self, Stdout};
use std::time::Duration;

use super::TuiState;
use crate::services::OdinServiceIntegration;
use gladsheim::skirnir::Skirnir;

/// Manages the TUI: terminal setup, drawing Status-Dashboard and Chat-Interface.
pub struct TuiManager {
    state: TuiState,
    odin: OdinServiceIntegration,
    skirnir: Skirnir,
}

impl TuiManager {
    pub fn new(state: TuiState, odin: OdinServiceIntegration, skirnir: Skirnir) -> Self {
        Self { state, odin, skirnir }
    }

    /// Run the TUI event loop (alternate screen, raw mode, draw until quit).
    pub async fn run(&self) -> io::Result<()> {
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_loop(&mut terminal).await;
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        res
    }

    async fn run_loop(&self, terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
        let skirnir = self.skirnir.clone();
        let state = self.state.clone();
        
        tokio::spawn(async move {
            let mut ticker = tokio::time::interval(Duration::from_secs(5));
            loop {
                ticker.tick().await;
                let services = skirnir.list_services().await;
                state.clear_status_lines();
                for s in services {
                    let mem_mb = s.resource_usage.memory_bytes as f32 / (1024.0 * 1024.0);
                    let health_desc = if s.health.is_healthy { "H" } else { "U" };
                    state.add_status_line(format!("{:<10} [{:?}] MEM:{:>5.1}MB CPU:{:>4.1}% HLTH:{}", 
                        s.name, s.status, mem_mb, s.resource_usage.cpu_percent, health_desc));
                }
            }
        });

        loop {
            terminal.draw(|f| self.draw(f))?;
            if crossterm::event::poll(Duration::from_millis(100))? {
                if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                    if key.kind != crossterm::event::KeyEventKind::Press {
                        continue;
                    }

                    match key.code {
                        crossterm::event::KeyCode::Char('q') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                            break;
                        },
                        crossterm::event::KeyCode::Tab => {
                            let next = match self.state.focus() {
                                crate::tui::ActivePanel::Chat => crate::tui::ActivePanel::Status,
                                crate::tui::ActivePanel::Status => crate::tui::ActivePanel::Services,
                                crate::tui::ActivePanel::Services => crate::tui::ActivePanel::Chat,
                            };
                            self.state.set_focus(next);
                        }
                        crossterm::event::KeyCode::Up => {
                            let curr = self.state.scroll_offset();
                            if curr > 0 { self.state.set_scroll_offset(curr - 1); }
                        }
                        crossterm::event::KeyCode::Down => {
                            let curr = self.state.scroll_offset();
                            self.state.set_scroll_offset(curr + 1);
                        }
                        crossterm::event::KeyCode::Char(c) => {
                            let mut buf = self.state.input_buffer();
                            buf.push(c);
                            self.state.set_input_buffer(buf);
                        }
                        crossterm::event::KeyCode::Backspace => {
                            let mut buf = self.state.input_buffer();
                            buf.pop();
                            self.state.set_input_buffer(buf);
                        }
                        crossterm::event::KeyCode::Enter => {
                            let input = self.state.input_buffer();
                            if !input.is_empty() {
                                self.state.set_input_buffer(String::new());
                                self.state.add_chat_message("user", input.clone());
                                
                                // Send to Odin (clone client for async call)
                                let mut odin = self.odin.clone();
                                let state = self.state.clone();
                                tokio::spawn(async move {
                                    match odin.send_chat(&input).await {
                                        Ok(resp) => {
                                            state.add_chat_message("odin", resp.response);
                                            for action in resp.actions_taken {
                                                state.add_chat_message("action", action);
                                            }
                                        }
                                        Err(e) => state.add_chat_message("error", e.to_string()),
                                    }
                                });
                            }
                        }
                        crossterm::event::KeyCode::Esc => {
                             // Global toggle for focus or menu
                             self.state.set_focus(crate::tui::ActivePanel::Chat);
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn draw(&self, f: &mut Frame) {
        let area = f.size();
        let focus = self.state.focus();

        // Main Layout: [Dashboard (Top)] + [Bottom Bar (Fixed)]
        let main_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(3),    // Dashboard
                Constraint::Length(1), // Help Bar
            ])
            .split(area);

        // Dashboard Layout: [Left (Chat/Input)] + [Right (Status)]
        let dashboard_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70), // Chat + Input
                Constraint::Percentage(30), // Status Dashboard
            ])
            .split(main_chunks[0]);

        // Left Chunks: [Chat (Top)] + [Input (Bottom)]
        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),   // Chat
                Constraint::Length(3), // Input
            ])
            .split(dashboard_chunks[0]);

        // --- STYLING ---
        let active_style = ratatui::style::Style::default().fg(ratatui::style::Color::Cyan);
        let inactive_style = ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray);

        // --- CHAT PANEL ---
        let chat_messages = self.state.chat_messages();
        let chat_lines: Vec<Line> = chat_messages
            .iter()
            .flat_map(|(sender, text)| {
                let color = match sender.as_str() {
                    "user" => ratatui::style::Color::Green,
                    "odin" => ratatui::style::Color::Magenta,
                    "action" => ratatui::style::Color::Blue,
                    "error" => ratatui::style::Color::Red,
                    _ => ratatui::style::Color::DarkGray,
                };
                vec![
                    Line::from(vec![
                        Span::styled(format!("{}: ", sender), ratatui::style::Style::default().fg(color).bold()),
                        Span::raw(text.as_str()),
                    ]),
                    Line::from(""), // Buffer line
                ]
            })
            .collect();

        let chat_title = if self.state.odin_connected() {
            " 💬 Chat (● ONLINE) "
        } else {
            " 💬 Chat (● OFFLINE) "
        };

        let chat_border_style = if focus == crate::tui::ActivePanel::Chat { active_style } else { ratatui::style::Style::default() };
        let chat = Paragraph::new(chat_lines)
            .block(Block::default()
                .title(chat_title)
                .borders(Borders::ALL)
                .border_style(chat_border_style))
            .wrap(Wrap { trim: true });
        f.render_widget(chat, left_chunks[0]);

        // --- INPUT PANEL ---
        let input_buf = self.state.input_buffer();
        let input_border_style = if focus == crate::tui::ActivePanel::Chat { active_style } else { ratatui::style::Style::default() };
        let input = Paragraph::new(input_buf.as_str())
            .block(Block::default()
                .title(" ⌨️ Input ")
                .borders(Borders::ALL)
                .border_style(input_border_style));
        f.render_widget(input, left_chunks[1]);

        // --- STATUS PANEL ---
        let status_lines = self.state.status_lines();
        let items: Vec<ListItem> = status_lines
            .iter()
            .map(|s| {
                let color = if s.contains("UNHEALTHY") || s.contains("] U") {
                    ratatui::style::Color::Red
                } else if s.contains("HEALTHY") || s.contains("] H") {
                    ratatui::style::Color::Green
                } else {
                    ratatui::style::Color::White
                };
                ListItem::new(Span::styled(s.as_str(), ratatui::style::Style::default().fg(color)))
            })
            .collect();

        let status_border_style = if focus == crate::tui::ActivePanel::Status { active_style } else { ratatui::style::Style::default() };
        let status_list = List::new(items).block(
            Block::default()
                .title(" 🛡️ Services ")
                .borders(Borders::ALL)
                .border_style(status_border_style),
        );
        f.render_widget(status_list, dashboard_chunks[1]);

        // --- BOTTOM BAR ---
        let help_text = vec![
            Span::styled(" Esc ", ratatui::style::Style::default().bg(ratatui::style::Color::White).fg(ratatui::style::Color::Black)),
            Span::raw(" Menu | "),
            Span::styled(" Tab ", ratatui::style::Style::default().bg(ratatui::style::Color::White).fg(ratatui::style::Color::Black)),
            Span::raw(" Switch | "),
            Span::styled(" Ctrl+Q ", ratatui::style::Style::default().bg(ratatui::style::Color::White).fg(ratatui::style::Color::Black)),
            Span::raw(" Quit "),
        ];
        let help_bar = Paragraph::new(Line::from(help_text))
            .style(ratatui::style::Style::default().bg(ratatui::style::Color::Black));
        f.render_widget(help_bar, main_chunks[1]);
    }
}
