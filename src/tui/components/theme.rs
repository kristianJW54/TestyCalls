use ratatui::{
    style::{Color, Style},
    symbols::merge::MergeStrategy,
    widgets::{Block, BorderType},
};

pub(crate) fn panel(title: &str) -> Block<'_> {
    Block::bordered()
        .merge_borders(MergeStrategy::Exact)
        .border_type(BorderType::Plain)
        .border_style(Style::default().fg(Color::Rgb(110, 110, 110)))
        .title_style(Style::default().fg(Color::Rgb(210, 210, 210)))
        .title(title)
}
