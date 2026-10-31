use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Gauge, Padding},
};
use throbber_widgets_tui::{BRAILLE_SIX_DOUBLE, Throbber, ThrobberState};

use crate::messages::RunPhase;
#[derive(Default)]
pub(crate) struct TestInfo {
    pub phase: RunPhase,
    pub completed: usize,
    pub total: Option<usize>,
    throbber: ThrobberState,
}

impl TestInfo {
    fn render_activity(&mut self, frame: &mut Frame, area: Rect, label: &'static str) {
        let widget = Throbber::default()
            .throbber_set(BRAILLE_SIX_DOUBLE)
            .label(label)
            .style(Style::default().fg(Color::Cyan));

        frame.render_stateful_widget(widget, area, &mut self.throbber);
        //
    }

    pub fn tick(&mut self) {
        if matches!(self.phase, RunPhase::Preparing | RunPhase::Running) {
            self.throbber.calc_next();
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        //
        let block = Block::new().padding(Padding::new(1, 1, 1, 1));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let [preparing, running, progress, summary, _remaining] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(inner);

        match self.phase {
            RunPhase::Idle => {
                frame.render_widget("Ready", preparing);
            }

            RunPhase::Preparing => {
                self.render_activity(frame, preparing, "Preparing tests…");
            }

            RunPhase::Running => {
                frame.render_widget("✓ Preparing tests", preparing);
                self.render_activity(frame, running, "Running Tests..");
                self.render_progress(frame, progress);
            }

            RunPhase::Completed => {
                frame.render_widget("✓ Preparing tests", preparing);
                frame.render_widget("✓ Finished running tests", running);
                self.render_progress(frame, progress);
                frame.render_widget("Run completed", summary);
            }

            RunPhase::FailedToRun => {
                frame.render_widget("Unable to finish the run", preparing);
                frame.render_widget("See test output for details", summary);
            }
        }
    }

    fn render_progress(&self, frame: &mut Frame, area: Rect) {
        match self.total {
            Some(total) if total > 0 => {
                let ratio = (self.completed as f64 / total as f64).clamp(0.0, 1.0);

                let gauge = Gauge::default()
                    .gauge_style(Style::default().fg(Color::Cyan))
                    .ratio(ratio)
                    .label(format!("{} / {}", self.completed, total));

                frame.render_widget(gauge, area);
            }

            Some(_) => {
                frame.render_widget("No tests selected", area);
            }

            None => {
                frame.render_widget(
                    format!("{} completed · total unknown", self.completed),
                    area,
                );
            }
        }
    }
}
