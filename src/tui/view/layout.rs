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
        let large_title = area.width >= 64 && area.height >= 36;
        let [header, project_area, body, footer] = Layout::vertical([
            Constraint::Length(if large_title { 4 } else { 1 }),
            Constraint::Length(4),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .areas(area);

        let [test_info, output_area] = Layout::vertical([
            Constraint::Length(
                (if body.width >= 64 && body.height >= 24 {
                    12
                } else {
                    8
                })
                .min(body.height / 2),
            ),
            Constraint::Min(6),
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
