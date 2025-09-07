use rubiks_cube_simulator::{State, RubiksCube, StateToDisplay, CubeNetWidget};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame, Terminal,
};
use std::io;

struct App {
    cube: RubiksCube,
    current_state: State,
    input_buffer: String,
    status_message: String,
    show_help: bool,
    debug_mode: bool,
}

impl App {
    fn new() -> Self {
        let cube = RubiksCube::new();
        let current_state = State::solved();
        
        Self {
            cube,
            current_state,
            input_buffer: String::new(),
            status_message: "Ready. Press 'h' for help, 'd' for debug, 'q' to quit.".to_string(),
            show_help: false,
            debug_mode: false,
        }
    }

    fn apply_scramble(&mut self) {
        if !self.input_buffer.trim().is_empty() {
            self.current_state = self.cube.scramble_to_state(&self.input_buffer);
            self.status_message = format!("Applied scramble: {}", self.input_buffer);
            self.input_buffer.clear();
        }
    }

    fn reset_cube(&mut self) {
        self.current_state = State::solved();
        self.status_message = "Cube reset to solved state.".to_string();
    }

    fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    fn toggle_debug(&mut self) {
        self.debug_mode = !self.debug_mode;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TUIモードを有効にする
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // TUIモードを無効にして元に戻す
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('h') => app.toggle_help(),
                KeyCode::Char('d') => app.toggle_debug(),
                KeyCode::Char('r') => app.reset_cube(),
                KeyCode::Enter => app.apply_scramble(),
                KeyCode::Backspace => {
                    app.input_buffer.pop();
                }
                KeyCode::Char(c) => {
                    app.input_buffer.push(c);
                }
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    if app.debug_mode {
        // デバッグモードでは構造体の詳細情報を表示
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(50), // キューブ表示
                Constraint::Percentage(50), // デバッグ情報
            ])
            .split(f.area());

        let left_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(15),        // キューブ表示
                Constraint::Length(3),      // 入力エリア
                Constraint::Length(2),      // ステータスエリア
            ])
            .split(chunks[0]);

        // キューブ表示
        let display = StateToDisplay::convert(&app.current_state);
        let cube_widget = CubeNetWidget::new(&display)
            .title(format!("🧩 Cube - {}", 
                if app.current_state.is_solved() { "SOLVED!" } else { "Scrambled" }));
        f.render_widget(cube_widget, left_chunks[0]);

        // 入力エリア
        let input_paragraph = Paragraph::new(app.input_buffer.as_str())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Scramble"))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(input_paragraph, left_chunks[1]);

        // ステータスエリア
        let status_paragraph = Paragraph::new(app.status_message.as_str())
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Left);
        f.render_widget(status_paragraph, left_chunks[2]);

        // デバッグ情報エリア
        let debug_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50), // State構造体
                Constraint::Percentage(50), // CubeDisplay構造体
            ])
            .split(chunks[1]);

        // State構造体のデバッグ出力
        let state_text = format!("State Debug:\n{}", app.current_state);
        let state_paragraph = Paragraph::new(state_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Internal State"))
            .style(Style::default().fg(Color::Cyan))
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(state_paragraph, debug_chunks[0]);

        // CubeDisplay構造体のデバッグ出力
        let display_text = display.to_debug_string();
        let display_paragraph = Paragraph::new(display_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Display State"))
            .style(Style::default().fg(Color::Magenta))
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(display_paragraph, debug_chunks[1]);

    } else {
        // 通常モード
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(15),  // キューブ表示エリア
                Constraint::Length(3), // 入力エリア
                Constraint::Length(2), // ステータスエリア
            ])
            .split(f.area());

        // キューブ表示
        let display = StateToDisplay::convert(&app.current_state);
        let cube_widget = CubeNetWidget::new(&display)
            .title(format!("🧩 Rubik's Cube - {}", 
                if app.current_state.is_solved() { "SOLVED!" } else { "Scrambled" }));
        f.render_widget(cube_widget, chunks[0]);

        // 入力エリア
        let input_paragraph = Paragraph::new(app.input_buffer.as_str())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Enter scramble (e.g., R U R' F R F')"))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(input_paragraph, chunks[1]);

        // ステータスエリア
        let status_paragraph = Paragraph::new(app.status_message.as_str())
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Left);
        f.render_widget(status_paragraph, chunks[2]);
    }

    // ヘルプオーバーレイ
    if app.show_help {
        let help_area = centered_rect(60, 80, f.area());
        f.render_widget(Clear, help_area);
        
        let help_text = vec![
            Line::from("🧩 Rubik's Cube Simulator Help"),
            Line::from(""),
            Line::from(vec![
                Span::styled("h", Style::default().fg(Color::Yellow)),
                Span::raw(" - Toggle this help"),
            ]),
            Line::from(vec![
                Span::styled("d", Style::default().fg(Color::Yellow)),
                Span::raw(" - Toggle debug mode"),
            ]),
            Line::from(vec![
                Span::styled("r", Style::default().fg(Color::Yellow)),
                Span::raw(" - Reset cube to solved state"),
            ]),
            Line::from(vec![
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(" - Apply scramble"),
            ]),
            Line::from(vec![
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(" - Quit"),
            ]),
            Line::from(""),
            Line::from("Moves: R L U D F B (+ ' for counter-clockwise, 2 for double)"),
            Line::from("Example: R U R' F R F' U R U' R' F' R F"),
        ];

        let help_paragraph = Paragraph::new(help_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Help")
                .style(Style::default().fg(Color::Cyan)))
            .alignment(Alignment::Left);
        f.render_widget(help_paragraph, help_area);
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solved_state() {
        let state = State::solved();
        assert!(state.is_solved());
    }

    #[test]
    fn test_app_creation() {
        let app = App::new();
        assert!(app.current_state.is_solved());
        assert!(app.input_buffer.is_empty());
    }
}
