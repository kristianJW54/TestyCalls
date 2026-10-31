use super::AppView;
use crossterm::event::{KeyCode, MouseEvent, MouseEventKind};
impl AppView {
    pub(in crate::tui) fn handle_popup_key(&mut self, key: KeyCode) -> bool {
        if self.popup == Some("Compiler messages") {
            match key {
                KeyCode::Esc | KeyCode::Char('c') => self.popup = None,
                KeyCode::Char('q') => return false,
                KeyCode::Char('j') => {
                    self.outputs[2].handle_key(KeyCode::Down);
                }
                KeyCode::Char('k') => {
                    self.outputs[2].handle_key(KeyCode::Up);
                }
                _ => {
                    self.outputs[2].handle_key(key);
                }
            }
            return true;
        }
        if self.popup.is_some() {
            if matches!(key, KeyCode::Esc | KeyCode::Char('?') | KeyCode::Enter) {
                self.popup = None;
            }
            return key != KeyCode::Char('q');
        }
        match key {
            KeyCode::Char('?') => self.popup = Some("Commands"),
            KeyCode::Char('t') => self.popup = Some("Test list tree"),
            KeyCode::Char('c') => self.popup = Some("Compiler messages"),
            _ => return false,
        }
        true
    }

    pub(in crate::tui) fn handle_mouse(&mut self, event: MouseEvent) {
        let key = match event.kind {
            MouseEventKind::ScrollUp => KeyCode::Up,
            MouseEventKind::ScrollDown => KeyCode::Down,
            _ => return,
        };
        if self.popup == Some("Compiler messages") {
            if self.output_areas[2].contains((event.column, event.row).into()) {
                self.outputs[2].handle_key(key);
            }
            return;
        }
        if self.popup.is_some() {
            return;
        }
        if let Some(index) = self
            .output_areas
            .iter()
            .take(2)
            .position(|area| area.contains((event.column, event.row).into()))
        {
            self.focused_output = index;
            self.outputs[index].handle_key(key);
        }
    }

    pub(in crate::tui) fn handle_output_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Tab | KeyCode::BackTab => self.focused_output = (self.focused_output + 1) % 2,
            KeyCode::Char('k') => return self.outputs[self.focused_output].handle_key(KeyCode::Up),
            KeyCode::Char('j') => {
                return self.outputs[self.focused_output].handle_key(KeyCode::Down);
            }
            _ => return self.outputs[self.focused_output].handle_key(key),
        }
        true
    }
}
