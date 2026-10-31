use super::AppView;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Clear, Padding, Paragraph, Wrap},
};
impl AppView {
    pub(super) fn render_popup(&mut self, frame: &mut Frame) {
        if self.popup == Some("Compiler messages") {
            let bounds = frame.area();
            let width = bounds.width.saturating_sub(4).max(1).min(bounds.width);
            let height = bounds.height.saturating_sub(4).max(1).min(bounds.height);
            let area = Rect::new(
                bounds.x + (bounds.width - width) / 2,
                bounds.y + (bounds.height - height) / 2,
                width,
                height,
            );
            self.output_areas[2] = area;
            frame.render_widget(Clear, area);
            frame.render_widget(
                Block::new().style(Style::default().bg(Color::Rgb(30, 30, 30)).fg(Color::White)),
                area,
            );
            self.outputs[2].render(
                frame,
                area,
                "Compiler messages · stderr · j/k scroll · Esc/c close",
                true,
            );
            if self.outputs[2].lines.is_empty() {
                let inner = Block::bordered().inner(area);
                frame.render_widget(
                    Paragraph::new("No compiler messages yet.")
                        .style(Style::default().fg(Color::Rgb(140, 140, 140))),
                    inner,
                );
            }
            return;
        }
        if let Some(title) = self.popup {
            let text = match title {
                "Commands" => {
                    "r      Refresh list and run sample demo\nq      Quit\nk / j  Move up / down in focused output\nTab    Switch focus (Shift+Tab: previous)\nEnter  Run command (currently demo)\nWheel  Scroll output under the mouse\nc      Inspect compiler messages\nt      Show test list tree\n?      Show commands\n\nPgUp/PgDn: page · Home/End: start/end\nEsc: close popup"
                }
                "Test list tree" => {
                    "No test list loaded yet.\nThe tree will appear here once discovery is connected.\n\nEsc: close"
                }
                _ => {
                    "Refresh is reserved for test-list discovery, which is not connected yet.\n\nEsc: close"
                }
            };
            let bounds = frame.area();
            let width = bounds.width.min(66);
            let height = bounds.height.min(16);
            let area = Rect::new(
                bounds.x + (bounds.width - width) / 2,
                bounds.y + (bounds.height - height) / 2,
                width,
                height,
            );
            frame.render_widget(Clear, area);
            frame.render_widget(
                Paragraph::new(text)
                    .wrap(Wrap { trim: false })
                    .block(
                        Block::bordered()
                            .border_type(BorderType::Rounded)
                            .title(title)
                            .padding(Padding::horizontal(1))
                            .border_style(Style::default().fg(Color::Cyan)),
                    )
                    .style(Style::default().bg(Color::Rgb(30, 30, 30)).fg(Color::White)),
                area,
            );
        }
    }
}
