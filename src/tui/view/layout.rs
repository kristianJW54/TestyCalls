use ratatui::layout::{Constraint, Layout, Rect};

pub(super) struct ScreenLayout {
    pub header: Rect,
    pub project_area: Rect,
    pub footer: Rect,
    pub test_info: Rect,
    pub test_output: Rect,
    pub failures: Rect,
    pub large_title: bool,
}

impl ScreenLayout {
    pub fn new(area: Rect) -> Self {
        let large_title = area.width >= 42 && area.height >= 26;
        let [header, project_area, body, footer] = Layout::vertical([
            Constraint::Length(if large_title { 4 } else { 1 }),
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(area);

        // Give test info a quarter of the body (at least eight rows when
        // space permits). Share the remaining height between output streams.
        let [test_info, output_area] = Layout::vertical([
            Constraint::Length((body.height / 4).max(6)),
            Constraint::Min(9),
        ])
        .areas(body);
        let results_area = output_area;
        let [test_output, failures] = if results_area.width >= 80 {
            Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).areas(results_area)
        } else {
            Layout::vertical([Constraint::Fill(2), Constraint::Fill(1)]).areas(results_area)
        };

        Self {
            header,
            project_area,
            footer,
            test_info,
            test_output,
            failures,
            large_title,
        }
    }
}
