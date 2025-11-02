use ratatui::{
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;

/// Interactive state input editor for cube state components
pub struct StateInputEditor {
    scramble: String,
    target_cp: [usize; 8],
    target_co: [usize; 8],
    target_ep: [usize; 12],
    target_eo: [usize; 12],
    current_field: StateField,
    cursor_position: usize,
    is_confirmed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum StateField {
    Scramble,
    Cp,
    Co,
    Ep,
    Eo,
}

impl StateInputEditor {
    /// Create a new editor with default values
    pub fn new() -> Self {
        Self {
            scramble: String::new(),
            target_cp: [0, 1, 2, 3, 4, 5, 6, 7],
            target_co: [0, 0, 0, 0, 0, 0, 0, 0],
            target_ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            target_eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            current_field: StateField::Scramble,
            cursor_position: 0,
            is_confirmed: false,
        }
    }

    /// Run the interactive editor
    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<Option<(String, [usize; 8], [usize; 8], [usize; 12], [usize; 12])>> {
        loop {
            terminal.draw(|f| self.ui(f))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => {
                            // Cancel input
                            return Ok(None);
                        }
                        KeyCode::Enter => {
                            // Move to next field or confirm if on last field
                            if self.current_field == StateField::Eo {
                                self.is_confirmed = true;
                                return Ok(Some((
                                    self.scramble.clone(),
                                    self.target_cp,
                                    self.target_co,
                                    self.target_ep,
                                    self.target_eo,
                                )));
                            } else {
                                self.current_field = self.next_field();
                                self.cursor_position = 0;
                            }
                        }
                        KeyCode::Tab => {
                            // Move to next field
                            self.current_field = self.next_field();
                            self.cursor_position = 0;
                        }
                        KeyCode::Left => {
                            if self.cursor_position > 0 {
                                self.cursor_position -= 1;
                            }
                        }
                        KeyCode::Right => {
                            let max_pos = match self.current_field {
                                StateField::Scramble => {
                                    if self.scramble.is_empty() {
                                        0
                                    } else {
                                        self.scramble.len() - 1
                                    }
                                }
                                StateField::Cp | StateField::Co => 7,
                                StateField::Ep | StateField::Eo => 11,
                            };
                            if self.cursor_position < max_pos {
                                self.cursor_position += 1;
                            }
                        }
                        KeyCode::Up => {
                            if self.current_field != StateField::Scramble {
                                self.increment_current_value();
                            }
                        }
                        KeyCode::Down => {
                            if self.current_field != StateField::Scramble {
                                self.decrement_current_value();
                            }
                        }
                        KeyCode::Char(c) => {
                            if self.current_field == StateField::Scramble {
                                self.scramble.insert(self.cursor_position, c);
                                self.cursor_position += 1;
                            }
                        }
                        KeyCode::Backspace => {
                            if self.current_field == StateField::Scramble && self.cursor_position > 0 {
                                self.scramble.remove(self.cursor_position - 1);
                                self.cursor_position -= 1;
                            }
                        }
                        KeyCode::Delete => {
                            if self.current_field == StateField::Scramble
                                && self.cursor_position < self.scramble.len()
                            {
                                self.scramble.remove(self.cursor_position);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn next_field(&self) -> StateField {
        match self.current_field {
            StateField::Scramble => StateField::Cp,
            StateField::Cp => StateField::Co,
            StateField::Co => StateField::Ep,
            StateField::Ep => StateField::Eo,
            StateField::Eo => StateField::Scramble,
        }
    }

    fn increment_current_value(&mut self) {
        match self.current_field {
            StateField::Scramble => {}
            StateField::Cp => {
                let max_value = 7;
                self.target_cp[self.cursor_position] =
                    (self.target_cp[self.cursor_position] + 1) % (max_value + 1);
            }
            StateField::Co => {
                let max_value = 2;
                self.target_co[self.cursor_position] =
                    (self.target_co[self.cursor_position] + 1) % (max_value + 1);
            }
            StateField::Ep => {
                let max_value = 11;
                self.target_ep[self.cursor_position] =
                    (self.target_ep[self.cursor_position] + 1) % (max_value + 1);
            }
            StateField::Eo => {
                let max_value = 1;
                self.target_eo[self.cursor_position] =
                    (self.target_eo[self.cursor_position] + 1) % (max_value + 1);
            }
        }
    }

    fn decrement_current_value(&mut self) {
        match self.current_field {
            StateField::Scramble => {}
            StateField::Cp => {
                let max_value = 7;
                self.target_cp[self.cursor_position] = if self.target_cp[self.cursor_position] == 0
                {
                    max_value
                } else {
                    self.target_cp[self.cursor_position] - 1
                };
            }
            StateField::Co => {
                let max_value = 2;
                self.target_co[self.cursor_position] = if self.target_co[self.cursor_position] == 0
                {
                    max_value
                } else {
                    self.target_co[self.cursor_position] - 1
                };
            }
            StateField::Ep => {
                let max_value = 11;
                self.target_ep[self.cursor_position] = if self.target_ep[self.cursor_position] == 0
                {
                    max_value
                } else {
                    self.target_ep[self.cursor_position] - 1
                };
            }
            StateField::Eo => {
                let max_value = 1;
                self.target_eo[self.cursor_position] = if self.target_eo[self.cursor_position] == 0
                {
                    max_value
                } else {
                    self.target_eo[self.cursor_position] - 1
                };
            }
        }
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Length(3), // Scramble field
                Constraint::Length(3), // CP field
                Constraint::Length(3), // CO field
                Constraint::Length(3), // EP field
                Constraint::Length(3), // EO field
                Constraint::Min(6),    // Instructions
            ])
            .split(f.area());

        // Title
        let title = Paragraph::new("Combined Nearby Search - Input Editor")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);

        // Render scramble field
        self.render_scramble_field(f, chunks[1]);

        // Render target state fields
        self.render_field(
            f,
            chunks[2],
            "Target CP (Corner Permutation)",
            &self.target_cp,
            StateField::Cp,
        );
        self.render_field(
            f,
            chunks[3],
            "Target CO (Corner Orientation)",
            &self.target_co,
            StateField::Co,
        );
        self.render_field(
            f,
            chunks[4],
            "Target EP (Edge Permutation)",
            &self.target_ep,
            StateField::Ep,
        );
        self.render_field(
            f,
            chunks[5],
            "Target EO (Edge Orientation)",
            &self.target_eo,
            StateField::Eo,
        );

        // Instructions
        let instructions = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Scramble: ", Style::default().fg(Color::Yellow)),
                Span::raw("Type moves (e.g., R U R' D)"),
            ]),
            Line::from(vec![
                Span::styled("State: ", Style::default().fg(Color::Yellow)),
                Span::raw("← → to move, ↑ ↓ to change value"),
            ]),
            Line::from(vec![
                Span::styled("Tab/Enter: ", Style::default().fg(Color::Yellow)),
                Span::raw("Next field / Submit (on EO)"),
            ]),
            Line::from(vec![
                Span::styled("Esc: ", Style::default().fg(Color::Yellow)),
                Span::raw("Cancel"),
            ]),
        ];
        let instructions_widget = Paragraph::new(instructions)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Instructions"),
            )
            .style(Style::default().fg(Color::Gray));
        f.render_widget(instructions_widget, chunks[6]);
    }

    fn render_scramble_field(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let is_active = self.current_field == StateField::Scramble;
        let border_style = if is_active {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        };

        let display_text = if self.scramble.is_empty() {
            "(empty)".to_string()
        } else {
            self.scramble.clone()
        };

        // Show cursor in scramble field
        let text = if is_active && !self.scramble.is_empty() {
            let before = &self.scramble[..self.cursor_position];
            let after = if self.cursor_position < self.scramble.len() {
                &self.scramble[self.cursor_position..]
            } else {
                ""
            };
            
            let mut spans = vec![Span::raw(before)];
            if self.cursor_position < self.scramble.len() {
                spans.push(Span::styled(
                    &self.scramble[self.cursor_position..self.cursor_position + 1],
                    Style::default().fg(Color::Black).bg(Color::Yellow),
                ));
                spans.push(Span::raw(&after[1..]));
            } else {
                spans.push(Span::styled(
                    " ",
                    Style::default().fg(Color::Black).bg(Color::Yellow),
                ));
            }
            Line::from(spans)
        } else if is_active && self.scramble.is_empty() {
            Line::from(vec![
                Span::raw("(empty) "),
                Span::styled(
                    " ",
                    Style::default().fg(Color::Black).bg(Color::Yellow),
                ),
            ])
        } else {
            Line::from(display_text)
        };

        let field_widget = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Scramble (e.g., R U R' D R U' R' D')")
                    .style(border_style),
            )
            .alignment(Alignment::Left);
        f.render_widget(field_widget, area);
    }

    fn render_field<T: std::fmt::Display>(
        &self,
        f: &mut Frame,
        area: ratatui::layout::Rect,
        title: &str,
        values: &[T],
        field: StateField,
    ) {
        let is_active = self.current_field == field;
        let border_style = if is_active {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Gray)
        };

        // Create the display text with cursor
        let mut spans = vec![Span::raw("[")];
        for (i, value) in values.iter().enumerate() {
            if is_active && i == self.cursor_position {
                // Highlight cursor position
                spans.push(Span::styled(
                    format!("{}", value),
                    Style::default().fg(Color::Black).bg(Color::Yellow),
                ));
            } else {
                spans.push(Span::styled(
                    format!("{}", value),
                    Style::default().fg(Color::White),
                ));
            }

            if i < values.len() - 1 {
                spans.push(Span::raw(", "));
            }
        }
        spans.push(Span::raw("]"));

        let field_widget = Paragraph::new(Line::from(spans))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .style(border_style),
            )
            .alignment(Alignment::Left);
        f.render_widget(field_widget, area);
    }
}
