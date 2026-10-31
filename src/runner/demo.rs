use crate::messages::{AppEvent, TestListStatus};
use std::time::{Duration, Instant};

#[derive(Default)]
pub(super) struct DemoRun {
    started: Option<Instant>,
    emitted_preparing: bool,
    emitted_compiling: bool,
    last_completed: Option<usize>,
}

impl DemoRun {
    pub fn start(&mut self) {
        *self = Self {
            started: Some(Instant::now()),
            ..Self::default()
        };
    }

    pub fn is_active(&self) -> bool {
        self.started.is_some()
    }

    pub fn advance(&mut self) -> Vec<AppEvent> {
        let Some(started) = self.started else {
            return Vec::new();
        };
        self.advance_at(started.elapsed())
    }

    fn advance_at(&mut self, elapsed: Duration) -> Vec<AppEvent> {
        let mut events = Vec::new();
        if !self.emitted_preparing {
            events.push(AppEvent::RunStarted);
            events.push(AppEvent::ListStatus(TestListStatus::Preparing));
            self.emitted_preparing = true;
        }
        if elapsed >= Duration::from_secs(1) && !self.emitted_compiling {
            events.push(AppEvent::ListStatus(TestListStatus::Compiling));
            self.emitted_compiling = true;
        }
        if elapsed >= Duration::from_secs(3) {
            let completed = ((elapsed.as_millis() - 3000) / 400).min(10) as usize;
            if self.last_completed.is_none() {
                events.push(AppEvent::ListStatus(TestListStatus::Compiled));
            }
            if self.last_completed != Some(completed) {
                events.push(AppEvent::Progress {
                    completed,
                    total: 10,
                });
                self.last_completed = Some(completed);
            }
            if completed == 10 {
                events.push(AppEvent::RunFinished);
                self.started = None;
            }
        }
        events
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn phases_are_emitted_once_and_demo_can_restart() {
        let mut demo = DemoRun::default();
        demo.start();
        assert!(matches!(
            demo.advance_at(Duration::ZERO)[0],
            AppEvent::RunStarted
        ));
        assert!(demo.advance_at(Duration::from_millis(100)).is_empty());
        assert!(matches!(
            demo.advance_at(Duration::from_secs(1))[0],
            AppEvent::ListStatus(TestListStatus::Compiling)
        ));
        assert!(matches!(
            demo.advance_at(Duration::from_secs(7)).last(),
            Some(AppEvent::RunFinished)
        ));
        assert!(!demo.is_active());
        demo.start();
        assert!(matches!(
            demo.advance_at(Duration::ZERO)[0],
            AppEvent::RunStarted
        ));
    }
}
