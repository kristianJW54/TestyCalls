use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph},
};
use throbber_widgets_tui::{BRAILLE_SIX_DOUBLE, Throbber, ThrobberState};

use crate::messages::TestListStatus;
#[derive(Default)]
pub(crate) struct TestListIndicator {
    pub phase: TestListStatus,
    throbber: ThrobberState,
}

impl TestListIndicator {
    pub fn tick(&mut self) {
        if matches!(
            self.phase,
            TestListStatus::Preparing | TestListStatus::Compiling
        ) {
            self.throbber.calc_next();
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let area = Block::new().padding(Padding::horizontal(1)).inner(area);
        let label = match self.phase {
            TestListStatus::Preparing => Some("Test list: preparing…"),
            TestListStatus::Compiling => Some("Test list: compiling…"),
            _ => None,
        };
        if let Some(label) = label {
            frame.render_stateful_widget(
                Throbber::default()
                    .throbber_set(BRAILLE_SIX_DOUBLE)
                    .label(label)
                    .style(Style::default().fg(Color::Cyan)),
                area,
                &mut self.throbber,
            );
            return;
        }
        let (label, color) = match self.phase {
            TestListStatus::NotLoaded => ("✗ No test list", Color::Rgb(180, 120, 120)),
            TestListStatus::Compiled => ("✓ Test list compiled (demo)", Color::Green),
            TestListStatus::Stale => ("⚠ Test list stale — update needed", Color::Yellow),
            _ => unreachable!(),
        };
        frame.render_widget(
            Paragraph::new(label).style(Style::default().fg(color)),
            area,
        );
    }
}
