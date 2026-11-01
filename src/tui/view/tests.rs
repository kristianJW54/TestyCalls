use super::*;
use crate::messages::{OutputKind, TestListStatus};
use crossterm::event::KeyCode;
use ratatui::{Terminal, backend::TestBackend};

#[test]
fn shrinking_width_preserves_room_for_output() {
    for height in [24, 30, 40] {
        let mut previous_height = u16::MAX;
        for width in [120, 80, 64, 60, 40] {
            let layout = layout::ScreenLayout::new(ratatui::layout::Rect::new(0, 0, width, height));
            assert!(layout.test_info.height <= previous_height);
            previous_height = layout.test_info.height;
            assert!(layout.test_output.height >= 5);
            assert!(layout.failures.height >= 3);
        }
    }
}

#[test]
fn compact_metrics_stay_beside_the_phase_chain() {
    let mut view = AppView::new();
    view.apply_event(AppEvent::Progress {
        completed: 10,
        total: 10,
    });
    view.apply_event(AppEvent::Summary(crate::runner::summary::TestSummary {
        passed: 8,
        failed: 2,
        ignored: 0,
        measured: 0,
        filtered: 3,
        seconds: 4.0,
    }));
    view.apply_event(AppEvent::RunFinished);
    let mut terminal = Terminal::new(TestBackend::new(60, 30)).unwrap();
    terminal.draw(|frame| view.render(frame)).unwrap();
    let layout = layout::ScreenLayout::new(ratatui::layout::Rect::new(0, 0, 60, 30));
    let buffer = terminal.backend().buffer();
    for (offset, label, value) in [(0, "PASSED", "8"), (1, "FAILED", "2"), (2, "TIME", "4.00s")] {
        let row: String = (32..59)
            .map(|x| buffer[(x, layout.test_info.y + offset)].symbol())
            .collect();
        assert!(row.contains(label), "missing {label}: {row}");
        assert!(row.contains(value), "missing {value}: {row}");
    }
}

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
