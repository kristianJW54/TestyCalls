use crate::{
    messages::{AppCommand, AppEvent, RunnerCommand},
    runner::ProcessRunner,
    tui,
};
use std::io;
#[cfg(test)]
mod tests;
use tokio::{
    sync::mpsc::{self, Receiver, Sender},
    task::JoinHandle,
};

pub(crate) struct Coordinator {
    app: JoinHandle<io::Result<()>>,
    app_commands: Receiver<AppCommand>,
    app_updates: Sender<AppEvent>,
    runner_commands: Sender<RunnerCommand>,
    runner_events: Receiver<AppEvent>,
    runner: JoinHandle<()>,
}

impl Coordinator {
    pub fn new() -> Self {
        let (app_tx, app_rx) = mpsc::channel(32);
        let (update_tx, update_rx) = mpsc::channel(128);
        let (runner_tx, runner_rx) = mpsc::channel(32);
        let (event_tx, event_rx) = mpsc::channel(128);
        Self {
            app: tokio::spawn(tui::run(app_tx, update_rx)),
            app_commands: app_rx,
            app_updates: update_tx,
            runner_commands: runner_tx,
            runner_events: event_rx,
            runner: tokio::spawn(ProcessRunner::new(runner_rx, event_tx).run()),
        }
    }

    pub async fn run(mut self) -> io::Result<()> {
        let result = self.event_loop().await;
        self.runner.abort();
        let _ = self.runner.await;
        drop(self.app_updates);
        if !self.app.is_finished() {
            let _ = self.app.await;
        }
        result
    }

    async fn event_loop(&mut self) -> io::Result<()> {
        let mut commands_open = true;
        loop {
            tokio::select! {
                result = &mut self.app => return result.map_err(io::Error::other)?,
                command = self.app_commands.recv(), if commands_open => {
                    match command {
                        Some(command) => self.handle_command(command).await?,
                        None => commands_open = false,
                    }
                }
                event = self.runner_events.recv() => {
                    let Some(event) = event else {
                        return Err(io::Error::other("Runner stopped unexpectedly"));
                    };
                    self.handle_runner_event(event).await?;
                }
            }
        }
    }

    pub(crate) async fn handle_command(&mut self, command: AppCommand) -> io::Result<()> {
        let runner_command = match command {
            AppCommand::RefreshList => RunnerCommand::RunDemo,
            AppCommand::RunTests => RunnerCommand::RunDemo,
        };
        self.runner_commands
            .send(runner_command)
            .await
            .map_err(io::Error::other)
    }

    pub(crate) async fn handle_runner_event(&mut self, event: AppEvent) -> io::Result<()> {
        let _ = self.app_updates.send(event).await;
        Ok(())
    }
}
