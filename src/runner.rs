mod demo;

use crate::messages::{AppEvent, RunnerCommand};
use tokio::sync::mpsc::{Receiver, Sender};

pub(crate) struct ProcessRunner {
    commands: Receiver<RunnerCommand>,
    events: Sender<AppEvent>,
}

impl ProcessRunner {
    pub fn new(commands: Receiver<RunnerCommand>, events: Sender<AppEvent>) -> Self {
        Self { commands, events }
    }

    pub async fn run(mut self) {
        let mut demo = demo::DemoRun::default();
        let mut ticks = tokio::time::interval(std::time::Duration::from_millis(100));
        loop {
            tokio::select! {
                command = self.commands.recv() => {
                    match command {
                        Some(RunnerCommand::RunDemo) => demo.start(),
                        None => break,
                    }
                }
                _ = ticks.tick(), if demo.is_active() => {
                    for event in demo.advance() {
                        if self.events.send(event).await.is_err() { return; }
                    }
                }
            }
        }
    }
}
