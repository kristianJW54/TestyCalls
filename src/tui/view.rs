mod header;
mod input;
mod layout;
mod popups;
#[cfg(test)]
mod tests;

use self::layout::ScreenLayout;
use super::components::{OutputPanel, TestInfo, TestListIndicator, panel};
use crate::messages::{AppEvent, RunPhase};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

#[derive(Default)]
pub(super) struct AppView {
    test_info: TestInfo,
    test_list_status: TestListIndicator,
    outputs: [OutputPanel; 3],
    focused_output: usize,
    output_areas: [Rect; 3],
    popup: Option<&'static str>,
    project_path: String,
}

impl AppView {
    pub(super) fn new() -> Self {
        Self {
            project_path: std::env::current_dir()
                .map(|path| path.display().to_string())
                .unwrap_or_else(|_| "Working directory unavailable".into()),
            ..Self::default()
        }
    }

    pub(super) fn apply_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::RunStarted => {
                self.test_info = TestInfo::default();
                self.test_info.phase = RunPhase::Preparing;
            }
            AppEvent::ListStatus(status) => self.test_list_status.phase = status,
            AppEvent::Progress { completed, total } => {
                self.test_info.completed = completed;
                self.test_info.total = Some(total);
                self.test_info.phase = RunPhase::Running;
            }
            AppEvent::RunFinished => self.test_info.phase = RunPhase::Completed,
            AppEvent::Output { kind, line } => self.outputs[kind.index()].lines.push(line),
        }
    }

    pub(super) fn tick(&mut self) {
        self.test_info.tick();
        self.test_list_status.tick();
    }

    pub(super) fn render(&mut self, frame: &mut Frame) {
        frame.render_widget(
            Block::new().style(Style::default().bg(Color::Reset).fg(Color::Reset)),
            frame.area(),
        );
        let ScreenLayout {
            header,
            project_area,
            footer,
            test_info,
            test_output,
            failures,
            large_title,
        } = ScreenLayout::new(frame.area());
        self.render_header(frame, header, large_title);
        self.render_project(frame, project_area);
        self.test_info.render(frame, test_info);
        self.output_areas = [test_output, failures, Rect::default()];
        for (index, (title, area)) in [
            ("Test results · stdout", test_output),
            ("Failures", failures),
        ]
        .into_iter()
        .enumerate()
        {
            self.outputs[index].render(
                frame,
                area,
                title,
                self.popup.is_none() && self.focused_output == index,
            );
        }

        let commands = "? Commands · c Compiler · r Refresh demo · Enter Run demo · q Quit";
        frame.render_widget(
            Paragraph::new(commands)
                .block(panel("Commands").border_style(Style::default().fg(Color::Rgb(60, 60, 60)))),
            footer,
        );

        self.render_popup(frame);
    }
}
