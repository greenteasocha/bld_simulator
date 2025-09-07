use ratatui::{prelude::*, widgets::*};
use super::{CubeDisplay, Face};

pub struct CubeNetWidget<'a> {
    cube: &'a CubeDisplay,
    title: Option<String>,
    show_borders: bool,
}

impl<'a> CubeNetWidget<'a> {
    pub fn new(cube: &'a CubeDisplay) -> Self {
        Self {
            cube,
            title: None,
            show_borders: true,
        }
    }

    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn borders(mut self, show: bool) -> Self {
        self.show_borders = show;
        self
    }

    /// キューブの展開図をテキストで生成
    #[allow(dead_code)]
    fn generate_cube_net(&self) -> Vec<String> {
        let mut lines = Vec::new();

        // 上面を描画
        if let Some(up_face) = self.cube.get_face(&Face::Up) {
            lines.push("    ┌─────┐".to_string());
            for row in 0..3 {
                let mut line = "    │".to_string();
                for col in 0..3 {
                    let color = up_face.get_cell(row, col);
                    line.push(color.to_char());
                }
                line.push_str("│");
                lines.push(line);
            }
            lines.push("┌───┼─────┼───┬───┐".to_string());
        }

        // 中段（左、前、右、後）を描画
        for row in 0..3 {
            let mut line = String::new();
            
            // 左面
            line.push('│');
            if let Some(left_face) = self.cube.get_face(&Face::Left) {
                for col in 0..3 {
                    let color = left_face.get_cell(row, col);
                    line.push(color.to_char());
                }
            }
            
            line.push('│');
            
            // 前面
            if let Some(front_face) = self.cube.get_face(&Face::Front) {
                for col in 0..3 {
                    let color = front_face.get_cell(row, col);
                    line.push(color.to_char());
                }
            }
            
            line.push('│');
            
            // 右面
            if let Some(right_face) = self.cube.get_face(&Face::Right) {
                for col in 0..3 {
                    let color = right_face.get_cell(row, col);
                    line.push(color.to_char());
                }
            }
            
            line.push('│');
            
            // 後面
            if let Some(back_face) = self.cube.get_face(&Face::Back) {
                for col in 0..3 {
                    let color = back_face.get_cell(row, col);
                    line.push(color.to_char());
                }
            }
            
            line.push('│');
            lines.push(line);
        }

        lines.push("└───┼─────┼───┴───┘".to_string());

        // 下面を描画
        if let Some(down_face) = self.cube.get_face(&Face::Down) {
            for row in 0..3 {
                let mut line = "    │".to_string();
                for col in 0..3 {
                    let color = down_face.get_cell(row, col);
                    line.push(color.to_char());
                }
                line.push_str("│");
                lines.push(line);
            }
            lines.push("    └─────┘".to_string());
        }

        lines
    }

    /// カラー版の展開図を生成（ratatui用）
    fn generate_colored_spans(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // 上面
        if let Some(up_face) = self.cube.get_face(&Face::Up) {
            lines.push(Line::from("    ┌─────┐"));
            for row in 0..3 {
                let mut spans = vec![Span::raw("    │")];
                for col in 0..3 {
                    let color = up_face.get_cell(row, col);
                    spans.push(Span::styled(
                        color.to_char().to_string(),
                        Style::default()
                            .bg(color.to_ratatui_color())
                            .fg(Color::Black)
                    ));
                }
                spans.push(Span::raw("│"));
                lines.push(Line::from(spans));
            }
            lines.push(Line::from("┌───┼─────┼───┬───┐"));
        }

        // 中段
        for row in 0..3 {
            let mut spans = vec![Span::raw("│")];
            
            // 左面
            if let Some(left_face) = self.cube.get_face(&Face::Left) {
                for col in 0..3 {
                    let color = left_face.get_cell(row, col);
                    spans.push(Span::styled(
                        color.to_char().to_string(),
                        Style::default()
                            .bg(color.to_ratatui_color())
                            .fg(Color::Black)
                    ));
                }
            }
            
            spans.push(Span::raw("│"));
            
            // 前面
            if let Some(front_face) = self.cube.get_face(&Face::Front) {
                for col in 0..3 {
                    let color = front_face.get_cell(row, col);
                    spans.push(Span::styled(
                        color.to_char().to_string(),
                        Style::default()
                            .bg(color.to_ratatui_color())
                            .fg(Color::Black)
                    ));
                }
            }
            
            spans.push(Span::raw("│"));
            
            // 右面
            if let Some(right_face) = self.cube.get_face(&Face::Right) {
                for col in 0..3 {
                    let color = right_face.get_cell(row, col);
                    spans.push(Span::styled(
                        color.to_char().to_string(),
                        Style::default()
                            .bg(color.to_ratatui_color())
                            .fg(Color::Black)
                    ));
                }
            }
            
            spans.push(Span::raw("│"));
            
            // 後面
            if let Some(back_face) = self.cube.get_face(&Face::Back) {
                for col in 0..3 {
                    let color = back_face.get_cell(row, col);
                    spans.push(Span::styled(
                        color.to_char().to_string(),
                        Style::default()
                            .bg(color.to_ratatui_color())
                            .fg(Color::Black)
                    ));
                }
            }
            
            spans.push(Span::raw("│"));
            lines.push(Line::from(spans));
        }

        lines.push(Line::from("└───┼─────┼───┴───┘"));

        // 下面
        if let Some(down_face) = self.cube.get_face(&Face::Down) {
            for row in 0..3 {
                let mut spans = vec![Span::raw("    │")];
                for col in 0..3 {
                    let color = down_face.get_cell(row, col);
                    spans.push(Span::styled(
                        color.to_char().to_string(),
                        Style::default()
                            .bg(color.to_ratatui_color())
                            .fg(Color::Black)
                    ));
                }
                spans.push(Span::raw("│"));
                lines.push(Line::from(spans));
            }
            lines.push(Line::from("    └─────┘"));
        }

        lines
    }
}

impl<'a> Widget for CubeNetWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = self.title.as_ref()
            .map(|s| s.clone())
            .unwrap_or_else(|| "🧩 Rubik's Cube".to_string());
        
        let block = if self.show_borders {
            Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
        } else {
            Block::default().title(title)
        };

        let inner = block.inner(area);
        block.render(area, buf);

        // カラー版の展開図を生成して描画
        let lines = self.generate_colored_spans();
        let text = Text::from(lines);
        
        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true });
        
        paragraph.render(inner, buf);
    }
}