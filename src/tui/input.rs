use crate::messages::AppCommand;
use crossterm::event::KeyCode;

pub(super) enum Action {
    Quit,
    Command(AppCommand),
}

pub(super) fn action(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('r') => Some(Action::Command(AppCommand::RefreshList)),
        KeyCode::Enter => Some(Action::Command(AppCommand::RunTests)),
        _ => None,
    }
}
