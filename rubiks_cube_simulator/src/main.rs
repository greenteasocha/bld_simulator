use rubiks_cube_simulator::{State, RubiksCube, StateToDisplay, CubeNetWidget, SolutionSearcher};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
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
    stickers_scroll: u16,
    is_pressing_enter: bool,
    selected_element: SelectedElement,
    solution: Option<String>,
}

#[derive(Clone, Copy, PartialEq)]
enum SelectedElement {
    InputField,
    SolveButton,
}

impl App {
    fn new() -> Self {
        let cube = RubiksCube::new();
        let current_state = State::solved();
        
        Self {
            cube,
            current_state,
            input_buffer: String::new(),
            status_message: "Ready. Press 'h' for help, 'd' for debug, 'q' to quit, Tab to switch between input/button.".to_string(),
            show_help: false,
            debug_mode: false,
            stickers_scroll: 0,
            is_pressing_enter: false,
            selected_element: SelectedElement::InputField,
            solution: None,
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
    
    fn solve_cube(&mut self) {
        let mut searcher = SolutionSearcher::with_bottom_layer_pattern(self.current_state.clone());
        
        match searcher.search() {
            Some(solutions) => {
                let moves = &solutions[0]; // Use the first solution found
                let solution_str = SolutionSearcher::format_solution(moves);
                self.solution = Some(solution_str.clone());
                
                // Apply the solution to actually solve the cube
                let mut state = self.current_state.clone();
                for move_action in moves {
                    if let Some(new_state) = move_action.apply_to_state(&state, &self.cube) {
                        state = new_state;
                    }
                }
                self.current_state = state;
                
                self.status_message = format!("✅ Solution found: {}", solution_str);
            }
            None => {
                self.solution = Some("No solution found within 3 moves".to_string());
                self.status_message = "❌ No solution found within 3 moves for bottom layer pattern.".to_string();
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TUIモードを有効にする
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // TUIモードを無効にして元に戻す
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
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
            match key.kind {
                KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('h') => app.toggle_help(),
                        KeyCode::Char('d') => app.toggle_debug(),
                        KeyCode::Char('r') => app.reset_cube(),
                        KeyCode::Tab => {
                            app.selected_element = match app.selected_element {
                                SelectedElement::InputField => SelectedElement::SolveButton,
                                SelectedElement::SolveButton => SelectedElement::InputField,
                            };
                        }
                        KeyCode::Enter => {
                            app.is_pressing_enter = true;
                            match app.selected_element {
                                SelectedElement::InputField => app.apply_scramble(),
                                SelectedElement::SolveButton => {
                                    // Show "Solving..." while key is pressed
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if app.selected_element == SelectedElement::InputField {
                                app.input_buffer.pop();
                            }
                        }
                        KeyCode::Up => {
                            if app.debug_mode && app.stickers_scroll > 0 {
                                app.stickers_scroll -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if app.debug_mode {
                                app.stickers_scroll += 1;
                            }
                        }
                        KeyCode::Char(c) => {
                            if app.selected_element == SelectedElement::InputField {
                                app.input_buffer.push(c);
                            }
                        }
                        _ => {}
                    }
                }
                KeyEventKind::Release => {
                    match key.code {
                        KeyCode::Enter => {
                            app.is_pressing_enter = false;
                            if app.selected_element == SelectedElement::SolveButton {
                                app.solve_cube();
                            }
                        }
                        _ => {}
                    }
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
                Constraint::Min(12),        // キューブ表示
                Constraint::Length(3),      // 入力エリア
                Constraint::Length(3),      // Solution表示エリア
                Constraint::Length(2),      // ステータスエリア
            ])
            .split(chunks[0]);

        // キューブ表示
        let display = StateToDisplay::convert(&app.current_state);
        let cube_widget = CubeNetWidget::new(&display)
            .title(format!("🧩 Cube - {}", 
                if app.current_state.is_solved() { "SOLVED!" } else { "Scrambled" }));
        f.render_widget(cube_widget, left_chunks[0]);

        // 入力エリアとボタンエリア
        let input_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70), // 入力フィールド
                Constraint::Percentage(30), // Solveボタン
            ])
            .split(left_chunks[1]);

        // 入力フィールド
        let input_style = if app.selected_element == SelectedElement::InputField {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Gray)
        };
        let input_paragraph = Paragraph::new(app.input_buffer.as_str())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Scramble"))
            .style(input_style);
        f.render_widget(input_paragraph, input_chunks[0]);

        // Solveボタン
        let is_solving = app.is_pressing_enter && app.selected_element == SelectedElement::SolveButton;
        let button_text = if is_solving { "Solving..." } else { "Solve" };
        let (button_style, button_title) = if is_solving {
            (Style::default().fg(Color::Yellow).bg(Color::Red), "🔄 SOLVING 🔄".to_string())
        } else if app.selected_element == SelectedElement::SolveButton {
            (Style::default().fg(Color::Black).bg(Color::Green), ">>> Solve <<<".to_string())
        } else {
            (Style::default().fg(Color::White), "Solve".to_string())
        };
        let button_paragraph = Paragraph::new(button_text)
            .block(Block::default().borders(Borders::ALL).title(button_title))
            .style(button_style)
            .alignment(Alignment::Center);
        f.render_widget(button_paragraph, input_chunks[1]);

        // Solution表示エリア
        let solution_text = app.solution.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("Press Solve to find a solution");
        let solution_paragraph = Paragraph::new(solution_text)
            .block(Block::default().borders(Borders::ALL).title("Solution"))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(solution_paragraph, left_chunks[2]);

        // ステータスエリア
        let status_paragraph = Paragraph::new(app.status_message.as_str())
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Left);
        f.render_widget(status_paragraph, left_chunks[3]);

        // デバッグ情報エリア
        let debug_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33), // State構造体
                Constraint::Percentage(33), // CubeStickers構造体
                Constraint::Percentage(34), // CubeDisplay構造体
            ])
            .split(chunks[1]);

        // デバッグ用に詳細な変換を実行
        let (display, cube_stickers) = StateToDisplay::convert_with_stickers(&app.current_state);

        // State構造体のデバッグ出力
        let state_text = format!("State Debug:\n{}", app.current_state);
        let state_paragraph = Paragraph::new(state_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Internal State"))
            .style(Style::default().fg(Color::Cyan))
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(state_paragraph, debug_chunks[0]);

        // CubeStickers構造体のデバッグ出力（スクロール対応）
        let stickers_text = cube_stickers.to_debug_string();
        let stickers_lines: Vec<&str> = stickers_text.lines().collect();
        
        // 表示可能な行数を計算
        let available_height = debug_chunks[1].height.saturating_sub(2) as usize; // ボーダーを除く
        let total_lines = stickers_lines.len();
        let scroll_offset = app.stickers_scroll as usize;
        
        // スクロール位置を調整
        let max_scroll = total_lines.saturating_sub(available_height);
        let actual_scroll = scroll_offset.min(max_scroll);
        
        // 表示する行を選択
        let display_lines = if total_lines > available_height {
            &stickers_lines[actual_scroll..actual_scroll + available_height]
        } else {
            &stickers_lines[..]
        };
        
        let display_text = display_lines.join("\n");
        let title = if total_lines > available_height {
            format!("Cube Stickers ({}/{}) [↑↓ to scroll]", actual_scroll + 1, total_lines)
        } else {
            "Cube Stickers".to_string()
        };
        
        let stickers_paragraph = Paragraph::new(display_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(title))
            .style(Style::default().fg(Color::Yellow));
        f.render_widget(stickers_paragraph, debug_chunks[1]);

        // CubeDisplay構造体のデバッグ出力
        let display_text = display.to_debug_string();
        let display_paragraph = Paragraph::new(display_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Display State"))
            .style(Style::default().fg(Color::Magenta))
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(display_paragraph, debug_chunks[2]);

    } else {
        // 通常モード
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(12),   // キューブ表示エリア
                Constraint::Length(3), // 入力エリア
                Constraint::Length(3), // Solution表示エリア
                Constraint::Length(2), // ステータスエリア
            ])
            .split(f.area());

        // キューブ表示
        let display = StateToDisplay::convert(&app.current_state);
        let cube_widget = CubeNetWidget::new(&display)
            .title(format!("🧩 Rubik's Cube - {}", 
                if app.current_state.is_solved() { "SOLVED!" } else { "Scrambled" }));
        f.render_widget(cube_widget, chunks[0]);

        // 入力エリアとボタンエリア
        let input_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(70), // 入力フィールド
                Constraint::Percentage(30), // Solveボタン
            ])
            .split(chunks[1]);

        // 入力フィールド
        let input_style = if app.selected_element == SelectedElement::InputField {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::Gray)
        };
        let input_paragraph = Paragraph::new(app.input_buffer.as_str())
            .block(Block::default()
                .borders(Borders::ALL)
                .title("Enter scramble (e.g., R U R' F R F')"))
            .style(input_style);
        f.render_widget(input_paragraph, input_chunks[0]);

        // Solveボタン
        let is_solving = app.is_pressing_enter && app.selected_element == SelectedElement::SolveButton;
        let button_text = if is_solving { "Solving..." } else { "Solve" };
        let (button_style, button_title) = if is_solving {
            (Style::default().fg(Color::Yellow).bg(Color::Red), "🔄 SOLVING 🔄".to_string())
        } else if app.selected_element == SelectedElement::SolveButton {
            (Style::default().fg(Color::Black).bg(Color::Green), ">>> Solve <<<".to_string())
        } else {
            (Style::default().fg(Color::White), "Solve".to_string())
        };
        let button_paragraph = Paragraph::new(button_text)
            .block(Block::default().borders(Borders::ALL).title(button_title))
            .style(button_style)
            .alignment(Alignment::Center);
        f.render_widget(button_paragraph, input_chunks[1]);

        // Solution表示エリア
        let solution_text = app.solution.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("Press Solve to find a solution");
        let solution_paragraph = Paragraph::new(solution_text)
            .block(Block::default().borders(Borders::ALL).title("Solution"))
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Left)
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(solution_paragraph, chunks[2]);

        // ステータスエリア
        let status_paragraph = Paragraph::new(app.status_message.as_str())
            .block(Block::default().borders(Borders::ALL).title("Status"))
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Left);
        f.render_widget(status_paragraph, chunks[3]);
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
                Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
                Span::raw(" - Scroll debug info (in debug mode)"),
            ]),
            Line::from(vec![
                Span::styled("r", Style::default().fg(Color::Yellow)),
                Span::raw(" - Reset cube to solved state"),
            ]),
            Line::from(vec![
                Span::styled("Tab", Style::default().fg(Color::Yellow)),
                Span::raw(" - Switch between input field and Solve button"),
            ]),
            Line::from(vec![
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(" - Apply scramble or solve cube"),
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
