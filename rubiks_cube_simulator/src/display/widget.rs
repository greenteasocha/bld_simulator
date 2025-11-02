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

    /// カラー版の展開図を生成（ratatui用）
    /// レイアウト: 上面(W)を前面(G)の上に配置
    fn generate_colored_spans(&self) -> Vec<Line<'static>> {
        let mut lines = Vec::new();

        // 各ステッカーを2文字幅で表示して罫線と幅を合わせる
        let sticker_width = 2;
        let face_width = sticker_width * 3; // 1面の幅 = 6文字

        // 上面 (前面の上に配置)
        // 左面(6文字) + 罫線(1文字) = 7文字分のパディング
        let left_padding = face_width + 1;
        
        if let Some(up_face) = self.cube.get_face(&Face::Up) {
            // 上部罫線
            lines.push(Line::from(format!("{}┌{}┐", 
                " ".repeat(left_padding), 
                "─".repeat(face_width)
            )));
            
            for row in 0..3 {
                let mut spans = vec![
                    Span::raw(" ".repeat(left_padding)),
                    Span::raw("│")
                ];
                for col in 0..3 {
                    let color = up_face.get_cell(row, col);
                    let char_display = format!("{:width$}", color.to_char(), width = sticker_width);
                    spans.push(Span::styled(
                        char_display,
                        Style::default()
                            .bg(color.to_ratatui_color())
                            .fg(Color::Black)
                    ));
                }
                spans.push(Span::raw("│"));
                lines.push(Line::from(spans));
            }
            
            // 中段への接続罫線
            lines.push(Line::from(format!(
                "┌{}┼{}┼{}┬{}┐",
                "─".repeat(face_width),
                "─".repeat(face_width),
                "─".repeat(face_width),
                "─".repeat(face_width)
            )));
        }

        // 中段（左、前、右、後）
        for row in 0..3 {
            let mut spans = vec![Span::raw("│")];
            
            // 左面
            if let Some(left_face) = self.cube.get_face(&Face::Left) {
                for col in 0..3 {
                    let color = left_face.get_cell(row, col);
                    let char_display = format!("{:width$}", color.to_char(), width = sticker_width);
                    spans.push(Span::styled(
                        char_display,
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
                    let char_display = format!("{:width$}", color.to_char(), width = sticker_width);
                    spans.push(Span::styled(
                        char_display,
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
                    let char_display = format!("{:width$}", color.to_char(), width = sticker_width);
                    spans.push(Span::styled(
                        char_display,
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
                    let char_display = format!("{:width$}", color.to_char(), width = sticker_width);
                    spans.push(Span::styled(
                        char_display,
                        Style::default()
                            .bg(color.to_ratatui_color())
                            .fg(Color::Black)
                    ));
                }
            }
            
            spans.push(Span::raw("│"));
            lines.push(Line::from(spans));
        }

        // 下段への接続罫線
        lines.push(Line::from(format!(
            "└{}┼{}┼{}┴{}┘",
            "─".repeat(face_width),
            "─".repeat(face_width),
            "─".repeat(face_width),
            "─".repeat(face_width)
        )));

        // 下面 (前面の下に配置)
        if let Some(down_face) = self.cube.get_face(&Face::Down) {
            for row in 0..3 {
                let mut spans = vec![
                    Span::raw(" ".repeat(left_padding)),
                    Span::raw("│")
                ];
                for col in 0..3 {
                    let color = down_face.get_cell(row, col);
                    let char_display = format!("{:width$}", color.to_char(), width = sticker_width);
                    spans.push(Span::styled(
                        char_display,
                        Style::default()
                            .bg(color.to_ratatui_color())
                            .fg(Color::Black)
                    ));
                }
                spans.push(Span::raw("│"));
                lines.push(Line::from(spans));
            }
            
            // 下部罫線
            lines.push(Line::from(format!("{}└{}┘", 
                " ".repeat(left_padding),
                "─".repeat(face_width)
            )));
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
