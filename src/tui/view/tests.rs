use super::*;
use crate::messages::{OutputKind, TestListStatus};
use crossterm::event::KeyCode;
use ratatui::{Terminal, backend::TestBackend};

#[test]
fn events_update_the_correct_display_state() {
    let mut view = AppView::new();
    view.apply_event(AppEvent::RunStarted);
    assert_eq!(view.test_info.phase, RunPhase::Preparing);
    view.apply_event(AppEvent::ListStatus(TestListStatus::Compiling));
    assert_eq!(view.test_list_status.phase, TestListStatus::Compiling);
    view.apply_event(AppEvent::Progress {
        completed: 2,
        total: 10,
    });
    assert_eq!(view.test_info.completed, 2);
    assert_eq!(view.test_info.phase, RunPhase::Running);
    view.apply_event(AppEvent::Output {
        kind: OutputKind::Compiler,
        line: "compiler output".into(),
    });
    assert_eq!(view.outputs[2].lines, ["compiler output"]);
    assert!(view.outputs[0].lines.is_empty());
    view.apply_event(AppEvent::RunFinished);
    assert_eq!(view.test_info.phase, RunPhase::Completed);
}

#[test]
fn layout_and_popups_render_on_wide_narrow_and_tiny_terminals() {
    let mut view = AppView::new();
    for (width, height) in [(120, 40), (60, 30), (10, 5), (1, 1)] {
        let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
        view.popup = None;
        terminal.draw(|frame| view.render(frame)).unwrap();
        if width == 120 {
            assert_eq!(view.output_areas[0].y, view.output_areas[1].y);
            assert!(view.output_areas[0].x < view.output_areas[1].x);
        } else if width == 60 {
            assert_eq!(view.output_areas[0].x, view.output_areas[1].x);
            assert!(view.output_areas[0].y < view.output_areas[1].y);
        }
        for key in ['c', '?', 't'] {
            view.handle_popup_key(KeyCode::Char(key));
            terminal.draw(|frame| view.render(frame)).unwrap();
            assert!(view.handle_popup_key(KeyCode::Esc));
            assert!(view.popup.is_none());
        }
    }
}
