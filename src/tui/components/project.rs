use crate::project::{Project, ProjectPhase};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Padding, Paragraph},
};
use throbber_widgets_tui::{BRAILLE_SIX_DOUBLE, Throbber, ThrobberState};

#[derive(Default)]
pub(crate) struct ProjectIndicator {
    pub project: Option<Project>,
    throbber: ThrobberState,
}

impl ProjectIndicator {
    pub fn tick(&mut self) {
        if self.project.as_ref().is_none_or(|project| {
            matches!(
                project.phase,
                ProjectPhase::Opening | ProjectPhase::Validating
            )
        }) {
            self.throbber.calc_next();
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let area = Block::new().padding(Padding::horizontal(1)).inner(area);
        let Some(project) = &self.project else {
            frame.render_widget("Opening project…", area);
            return;
        };
        let path = project.selected_path.display().to_string();
        let (label, color) = match project.phase {
            ProjectPhase::Opening => (format!("Opening · {path}"), Color::Cyan),
            ProjectPhase::Validating => (format!("Validating · {path}"), Color::Cyan),
            ProjectPhase::Connected => (format!("✓ Connected · {path}"), Color::Green),
            ProjectPhase::Failed => (
                format!("✗ Cannot connect · {path} · c: details"),
                Color::Red,
            ),
        };
        if matches!(
            project.phase,
            ProjectPhase::Opening | ProjectPhase::Validating
        ) {
            frame.render_stateful_widget(
                Throbber::default()
                    .throbber_set(BRAILLE_SIX_DOUBLE)
                    .label(label)
                    .style(Style::default().fg(color)),
                area,
                &mut self.throbber,
            );
        } else {
            frame.render_widget(
                Paragraph::new(label).style(Style::default().fg(color)),
                area,
            );
        }
    }
}
