use crate::{messages::RunPhase, runner::summary::TestSummary};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Gauge, Padding, Paragraph},
};
use throbber_widgets_tui::{BRAILLE_SIX_DOUBLE, Throbber, ThrobberState};

#[derive(Default)]
pub(crate) struct TestInfo {
    pub phase: RunPhase,
    pub completed: usize,
    pub total: Option<usize>,
    pub summary: Option<TestSummary>,
    throbber: ThrobberState,
}

impl TestInfo {
    pub fn tick(&mut self) {
        if matches!(self.phase, RunPhase::Preparing | RunPhase::Running) {
            self.throbber.calc_next();
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let compact = area.width < 64 || area.height < 12;
        let padding = if compact {
            Padding::horizontal(1)
        } else {
            Padding::new(2, 2, 1, 1)
        };
        let inner = Block::new().padding(padding).inner(area);
        let [overview, progress_band, details, _rest] = Layout::vertical([
            Constraint::Length(if compact { 3 } else { 6 }),
            Constraint::Length(3),
            Constraint::Length(1),
            Constraint::Min(0),
        ])
        .areas(inner);
        let [activity, metrics] =
            Layout::horizontal([Constraint::Percentage(55), Constraint::Percentage(45)])
                .spacing(1)
                .areas(overview);
        self.render_chain(frame, activity);
        let [_, progress, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(1),
            Constraint::Fill(1),
        ])
        .areas(progress_band);
        let ratio = self.total.filter(|total| *total > 0).map_or(0.0, |total| {
            (self.completed as f64 / total as f64).clamp(0.0, 1.0)
        });
        let label = match self.total {
            Some(total) => format!("{} / {total}   ·   {:.0}%", self.completed, ratio * 100.0),
            None => "Waiting for tests".into(),
        };
        let color = if self
            .summary
            .as_ref()
            .is_some_and(|summary| summary.failed > 0)
        {
            Color::Rgb(235, 120, 120)
        } else {
            Color::Rgb(100, 195, 170)
        };
        frame.render_widget(
            Gauge::default()
                .use_unicode(true)
                .ratio(ratio)
                .label(label)
                .gauge_style(Style::default().fg(color).bg(Color::Rgb(45, 50, 55)))
                .style(Style::default().add_modifier(Modifier::BOLD)),
            progress,
        );

        let values = match &self.summary {
            Some(summary) => [
                summary.passed.to_string(),
                summary.failed.to_string(),
                format!("{:.2}s", summary.seconds),
            ],
            None => ["—".into(), "—".into(), "—".into()],
        };
        let rows = Layout::vertical([Constraint::Fill(1); 3]).split(metrics);
        for (index, (label, color)) in [
            ("Passed", Color::Rgb(125, 240, 175)),
            ("Failed", Color::Rgb(255, 145, 150)),
            ("Finished in", Color::Rgb(155, 205, 255)),
        ]
        .into_iter()
        .enumerate()
        {
            let background = if self.summary.is_none() {
                Color::Rgb(40, 43, 47)
            } else {
                [
                    Color::Rgb(30, 55, 44),
                    Color::Rgb(57, 36, 41),
                    Color::Rgb(33, 44, 58),
                ][index]
            };
            // Terminal glyphs occupy whole rows. A one-row card avoids the
            // uneven top/bottom padding of a two-row card.
            let card_area = Rect::new(
                rows[index].x,
                rows[index].y,
                rows[index].width,
                rows[index].height.min(1),
            );
            let card = Block::new().style(Style::default().bg(background));
            frame.render_widget(card, card_area);
            let content = Block::new()
                .padding(Padding::horizontal(1))
                .inner(card_area);
            let [label_area, value_area] =
                Layout::horizontal([Constraint::Percentage(45), Constraint::Percentage(55)])
                    .areas(content);
            let label = if compact && index == 2 { "Time" } else { label };
            frame.render_widget(
                Paragraph::new(label.to_uppercase())
                    .alignment(Alignment::Left)
                    .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
                label_area,
            );
            frame.render_widget(
                Paragraph::new(values[index].as_str())
                    .alignment(Alignment::Right)
                    .style(Style::default().fg(color).add_modifier(Modifier::BOLD)),
                value_area,
            );
        }
        self.render_summary(frame, details);
        if self.phase == RunPhase::FailedToRun {
            frame.render_widget("Unable to finish · c for compiler details", details);
        }
    }

    pub fn render_summary(&self, frame: &mut Frame, area: Rect) {
        let text = match &self.summary {
            Some(summary) if area.width < 76 => format!(
                "{} ignored · {} measured · {} filtered",
                summary.ignored, summary.measured, summary.filtered,
            ),
            Some(summary) => format!(
                "{} passed · {} failed · {:.2}s  |  {} ignored · {} measured · {} filtered out",
                summary.passed,
                summary.failed,
                summary.seconds,
                summary.ignored,
                summary.measured,
                summary.filtered,
            ),
            None => "Waiting for results".into(),
        };
        frame.render_widget(
            Paragraph::new(text).style(Style::default().fg(Color::Rgb(155, 160, 170))),
            area,
        );
    }

    fn render_chain(&mut self, frame: &mut Frame, area: Rect) {
        let compact = area.height < 5;
        let cells = Layout::vertical([Constraint::Length(1); 5]).split(area);
        let current = match self.phase {
            RunPhase::Idle | RunPhase::Preparing => 0,
            RunPhase::Running => 1,
            RunPhase::Completed | RunPhase::FailedToRun => 2,
        };
        for (index, label) in ["Preparing tests", "Running tests", "Completed"]
            .into_iter()
            .enumerate()
        {
            let row = if compact { index } else { index * 2 };
            if row >= usize::from(area.height) {
                continue;
            }
            let completed_label = match self.total {
                Some(total) => format!("Completed {total} Tests"),
                None => "Completed".into(),
            };
            let label = if index == 2 {
                completed_label.as_str()
            } else if area.width < 22 {
                ["Prepare", "Run"][index]
            } else {
                label
            };
            if index < 2 && !compact {
                frame.render_widget(
                    Paragraph::new("│").style(Style::default().fg(Color::Rgb(65, 75, 80))),
                    cells[index * 2 + 1],
                );
            }
            if index == current && matches!(self.phase, RunPhase::Preparing | RunPhase::Running) {
                frame.render_stateful_widget(
                    Throbber::default()
                        .throbber_set(BRAILLE_SIX_DOUBLE)
                        .label(label)
                        .style(Style::default().fg(Color::Cyan)),
                    Rect::new(area.x, area.y.saturating_add(row as u16), area.width, 1),
                    &mut self.throbber,
                );
            } else {
                let done = self.phase == RunPhase::Completed || index < current;
                let text = if index == 2 && self.phase == RunPhase::FailedToRun {
                    "✗ Stopped".into()
                } else {
                    format!(
                        "{} {label}",
                        if done {
                            "✓"
                        } else if compact && index == 2 {
                            "└"
                        } else if compact {
                            "├"
                        } else {
                            "·"
                        }
                    )
                };
                frame.render_widget(
                    Paragraph::new(text).style(Style::default().fg(if done {
                        Color::Rgb(100, 195, 170)
                    } else {
                        Color::Rgb(110, 115, 125)
                    })),
                    Rect::new(area.x, area.y.saturating_add(row as u16), area.width, 1),
                );
            }
        }
    }
}
