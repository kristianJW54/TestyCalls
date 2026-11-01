use crate::{
    messages::{AppCommand, AppEvent, RunnerCommand},
    project::Project,
    runner::ProcessRunner,
    test_tree::Catalogue,
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
    project: Project,
    app: JoinHandle<io::Result<()>>,
    app_commands: Receiver<AppCommand>,
    app_updates: Sender<AppEvent>,
    runner_commands: Sender<RunnerCommand>,
    runner_events: Receiver<AppEvent>,
    runner: JoinHandle<()>,
    catalogue: Option<Catalogue>,
}

impl Coordinator {
    pub fn new(project: Project) -> Self {
        let (app_tx, app_rx) = mpsc::channel(32);
        let (update_tx, update_rx) = mpsc::channel(128);
        let (runner_tx, runner_rx) = mpsc::channel(32);
        let (event_tx, event_rx) = mpsc::channel(128);
        runner_tx
            .try_send(RunnerCommand::OpenProject(project.clone()))
            .expect("new runner channel has capacity");
        Self {
            project,
            app: tokio::spawn(tui::run(app_tx, update_rx)),
            app_commands: app_rx,
            app_updates: update_tx,
            runner_commands: runner_tx,
            runner_events: event_rx,
            runner: tokio::spawn(ProcessRunner::new(runner_rx, event_tx).run()),
            catalogue: None,
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
        if let AppEvent::Output {
            kind: crate::messages::OutputKind::Results,
            line,
        } = &event
        {
            if let Some(summary) = crate::runner::summary::TestSummary::parse(line) {
                let _ = self.app_updates.send(AppEvent::Summary(summary)).await;
            }
        }
        if let AppEvent::Project(project) = &event {
            self.project = project.clone();
        }
        let _ = self.app_updates.send(event).await;
        Ok(())
    }

    pub(crate) async fn build_and_refresh_catalogue(&mut self) -> io::Result<()> {
        // Request the metadata commands to be run from catalogue
        let fake_stdout = [0u8; 10];
        // Run them
        // Add packages and targets
        if let Some(catalogue) = &mut self.catalogue {
            catalogue.build_package_and_targets(&fake_stdout)?;
        }

        // Iterate the package and target and request the test list command from catalogue
        // Send to runner to execute and return
        // Use catalogue.package[i].target[j].build_tree(data..)?;

        // TODO
        todo!()
    }

    // async fn startup(&mut self) -> io::Result<()> {
    //     let metadata = self.open_and_validate_project().await?;
    //     self.build_catalogue(metadata).await
    // }
    //
    // async fn refresh_catalogue(&mut self) -> io::Result<()> {
    //     let metadata = self.request_metadata().await?;
    //     self.build_catalogue(metadata).await
    // }
}
