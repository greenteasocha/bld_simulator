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
    cp: [usize; 8],
    co: [usize; 8],
    ep: [usize; 12],
    eo: [usize; 12],
    current_field: StateField,
    cursor_position: usize,
    is_confirmed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum StateField {
    Cp,
    Co,
    Ep,
    Eo,
}

impl StateInputEditor {
    /// Create a new editor with default solved state
    pub fn new() -> Self {
        Self {
            cp: [0, 1, 2, 3, 4, 5, 6, 7],
            co: [0, 0, 0, 0, 0, 0, 0, 0],
            ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            eo: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            current_field: StateField::Cp,
            cursor_position: 0,
            is_confirmed: false,
        }
    }

    /// Run the interactive editor
    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<Option<([usize; 8], [usize; 8], [usize; 12], [usize; 12])>> {
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
                                return Ok(Some((self.cp, self.co, self.ep, self.eo)));
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
                                StateField::Cp | StateField::Co => 7,
                                StateField::Ep | StateField::Eo => 11,
                            };
                            if self.cursor_position < max_pos {
                                self.cursor_position += 1;
                            }
                        }
                        KeyCode::Up => {
                            self.increment_current_value();
                        }
                        KeyCode::Down => {
                            self.decrement_current_value();
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn next_field(&self) -> StateField {
        match self.current_field {
            StateField::Cp => StateField::Co,
            StateField::Co => StateField::Ep,
            StateField::Ep => StateField::Eo,
            StateField::Eo => StateField::Cp,
        }
    }

    fn increment_current_value(&mut self) {
        match self.current_field {
            StateField::Cp => {
                let max_value = 7;
                self.cp[self.cursor_position] = (self.cp[self.cursor_position] + 1) % (max_value + 1);
            }
            StateField::Co => {
                let max_value = 2;
                self.co[self.cursor_position] = (self.co[self.cursor_position] + 1) % (max_value + 1);
            }
            StateField::Ep => {
                let max_value = 11;
                self.ep[self.cursor_position] = (self.ep[self.cursor_position] + 1) % (max_value + 1);
            }
            StateField::Eo => {
                let max_value = 1;
                self.eo[self.cursor_position] = (self.eo[self.cursor_position] + 1) % (max_value + 1);
            }
        }
    }

    fn decrement_current_value(&mut self) {
        match self.current_field {
            StateField::Cp => {
                let max_value = 7;
                self.cp[self.cursor_position] = if self.cp[self.cursor_position] == 0 {
                    max_value
                } else {
                    self.cp[self.cursor_position] - 1
                };
            }
            StateField::Co => {
                let max_value = 2;
                self.co[self.cursor_position] = if self.co[self.cursor_position] == 0 {
                    max_value
                } else {
                    self.co[self.cursor_position] - 1
                };
            }
            StateField::Ep => {
                let max_value = 11;
                self.ep[self.cursor_position] = if self.ep[self.cursor_position] == 0 {
                    max_value
                } else {
                    self.ep[self.cursor_position] - 1
                };
            }
            StateField::Eo => {
                let max_value = 1;
                self.eo[self.cursor_position] = if self.eo[self.cursor_position] == 0 {
                    max_value
                } else {
                    self.eo[self.cursor_position] - 1
                };
            }
        }
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Title
                Constraint::Length(3), // CP field
                Constraint::Length(3), // CO field
                Constraint::Length(3), // EP field
                Constraint::Length(3), // EO field
                Constraint::Min(5),    // Instructions
            ])
            .split(f.area());

        // Title
        let title = Paragraph::new("Cube State Input Editor")
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center);
        f.render_widget(title, chunks[0]);

        // Render each field
        self.render_field(f, chunks[1], "CP (Corner Permutation)", &self.cp, StateField::Cp);
        self.render_field(f, chunks[2], "CO (Corner Orientation)", &self.co, StateField::Co);
        self.render_field(f, chunks[3], "EP (Edge Permutation)", &self.ep, StateField::Ep);
        self.render_field(f, chunks[4], "EO (Edge Orientation)", &self.eo, StateField::Eo);

        // Instructions
        let instructions = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("Navigation: ", Style::default().fg(Color::Yellow)),
                Span::raw("← → to move cursor, ↑ ↓ to change value"),
            ]),
            Line::from(vec![
                Span::styled("Tab: ", Style::default().fg(Color::Yellow)),
                Span::raw("Move to next field"),
            ]),
            Line::from(vec![
                Span::styled("Enter: ", Style::default().fg(Color::Yellow)),
                Span::raw("Confirm current field / Submit (on EO)"),
            ]),
            Line::from(vec![
                Span::styled("Esc: ", Style::default().fg(Color::Yellow)),
                Span::raw("Cancel and return"),
            ]),
        ];
        let instructions_widget = Paragraph::new(instructions)
            .block(Block::default().borders(Borders::ALL).title("Instructions"))
            .style(Style::default().fg(Color::Gray));
        f.render_widget(instructions_widget, chunks[5]);
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
            .block(Block::default().borders(Borders::ALL).title(title).style(border_style))
            .alignment(Alignment::Left);
        f.render_widget(field_widget, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_editor() {
        let editor = StateInputEditor::new();
        assert_eq!(editor.cp, [0, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(editor.co, [0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(editor.ep, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
        assert_eq!(editor.eo, [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(editor.cursor_position, 0);
        assert_eq!(editor.current_field, StateField::Cp);
    }

    #[test]
    fn test_increment_cp() {
        let mut editor = StateInputEditor::new();
        editor.current_field = StateField::Cp;
        editor.cursor_position = 0;
        
        editor.increment_current_value();
        assert_eq!(editor.cp[0], 1);
        
        // Test wrap around
        editor.cp[0] = 7;
        editor.increment_current_value();
        assert_eq!(editor.cp[0], 0);
    }

    #[test]
    fn test_decrement_co() {
        let mut editor = StateInputEditor::new();
        editor.current_field = StateField::Co;
        editor.cursor_position = 0;
        editor.co[0] = 1;
        
        editor.decrement_current_value();
        assert_eq!(editor.co[0], 0);
        
        // Test wrap around
        editor.decrement_current_value();
        assert_eq!(editor.co[0], 2);
    }

    #[test]
    fn test_next_field() {
        let editor = StateInputEditor::new();
        
        assert_eq!(editor.next_field(), StateField::Co);
        
        let mut editor = StateInputEditor::new();
        editor.current_field = StateField::Co;
        assert_eq!(editor.next_field(), StateField::Ep);
        
        editor.current_field = StateField::Ep;
        assert_eq!(editor.next_field(), StateField::Eo);
        
        editor.current_field = StateField::Eo;
        assert_eq!(editor.next_field(), StateField::Cp);
    }
}
