use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
};

use super::panel;
#[derive(Default)]
pub(crate) struct OutputPanel {
    pub lines: Vec<String>,
    offset: usize,
    visible_rows: usize,
}

impl OutputPanel {
    fn max_offset(&self) -> usize {
        self.lines.len().saturating_sub(self.visible_rows.max(1))
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyCode) -> bool {
        use crossterm::event::KeyCode;
        let page = self.visible_rows.max(1);
        self.offset = match key {
            KeyCode::Up => self.offset.saturating_sub(1),
            KeyCode::Down => self.offset.saturating_add(1),
            KeyCode::PageUp => self.offset.saturating_sub(page),
            KeyCode::PageDown => self.offset.saturating_add(page),
            KeyCode::Home => 0,
            KeyCode::End => self.max_offset(),
            _ => return false,
        }
        .min(self.max_offset());
        true
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, title: &str, focused: bool) {
        let block = panel(title).border_style(Style::default().fg(if focused {
            Color::Cyan
        } else {
            Color::Rgb(60, 60, 60)
        }));
        let inner = block.inner(area);
        self.visible_rows = usize::from(inner.height);
        self.offset = self.offset.min(self.max_offset());
        frame.render_widget(block, area);
        let lines: Vec<ratatui::text::Line> = self
            .lines
            .iter()
            .skip(self.offset)
            .take(self.visible_rows)
            .map(|line| line.as_str().into())
            .collect();
        frame.render_widget(Paragraph::new(lines), inner);
    }
}

#[cfg(test)]
mod output_tests {
    use super::*;
    use crossterm::event::KeyCode;

    #[test]
    fn scrolling_is_bounded_and_independent() {
        let mut first = OutputPanel {
            lines: (0..20).map(|n| n.to_string()).collect(),
            visible_rows: 5,
            ..Default::default()
        };
        let second = OutputPanel::default();
        first.handle_key(KeyCode::PageDown);
        assert_eq!(first.offset, 5);
        first.handle_key(KeyCode::End);
        first.handle_key(KeyCode::Down);
        assert_eq!(first.offset, 15);
        assert_eq!(second.offset, 0);
        first.handle_key(KeyCode::Home);
        first.handle_key(KeyCode::Up);
        assert_eq!(first.offset, 0);
    }
}
