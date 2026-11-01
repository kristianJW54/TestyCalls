mod header;
mod input;
mod layout;
mod popups;
#[cfg(test)]
mod tests;

use self::layout::ScreenLayout;
use super::components::{OutputPanel, ProjectIndicator, TestInfo, TestListIndicator, panel};
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
    project: ProjectIndicator,
}

impl AppView {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn apply_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::Summary(summary) => self.test_info.summary = Some(summary),
            AppEvent::Project(project) => {
                if let Some(error) = &project.error {
                    self.outputs[2]
                        .lines
                        .extend(error.lines().map(str::to_owned));
                }
                self.project.project = Some(project);
            }
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
        self.project.tick();
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

        frame.render_widget(
            Paragraph::new("? Commands · c Compiler · r Refresh demo · Enter Run demo · q Quit")
                .block(panel("Commands").border_style(Style::default().fg(Color::Rgb(60, 60, 60)))),
            footer,
        );

        self.render_popup(frame);
    }
}
