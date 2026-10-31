use super::AppView;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Padding, Paragraph},
};
use tui_big_text::{BigText, PixelSize};
impl AppView {
    pub(super) fn render_header(&self, frame: &mut Frame, header: Rect, large_title: bool) {
        let title_style = Style::default()
            .fg(Color::Rgb(210, 210, 210))
            .add_modifier(Modifier::BOLD);
        let title_area = Block::new().padding(Padding::horizontal(1)).inner(header);
        if large_title {
            let title = BigText::builder()
                .pixel_size(PixelSize::Quadrant)
                .style(title_style)
                .lines(vec!["TestyCalls".into()])
                .build();
            frame.render_widget(title, title_area);
        } else {
            frame.render_widget(Paragraph::new("TestyCalls").style(title_style), title_area);
        }
    }
    pub(super) fn render_project(&mut self, frame: &mut Frame, project_area: Rect) {
        let project_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Rgb(60, 60, 60)));
        let project_inner = project_block.inner(project_area);
        frame.render_widget(project_block, project_area);
        let [path_area, list_status] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(1)]).areas(project_inner);
        let display_path = self.project_path.replace('\\', " / ");
        frame.render_widget(
            Paragraph::new(format!("Project  ·  {display_path}"))
                .style(Style::default().fg(Color::Rgb(140, 140, 140)))
                .block(Block::new().padding(Padding::horizontal(1))),
            path_area,
        );
        self.test_list_status.render(frame, list_status);
    }
}
