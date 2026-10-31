mod components;
mod input;
mod terminal;
mod view;

use self::{input::Action, view::AppView};
use crate::messages::{AppCommand, AppEvent};
use crossterm::event::{Event, EventStream, KeyEventKind};
use futures::StreamExt;
use ratatui::DefaultTerminal;
use std::{io, time::Duration};
use tokio::sync::mpsc::{Receiver, Sender};

pub(crate) async fn run(
    commands: Sender<AppCommand>,
    updates: Receiver<AppEvent>,
) -> io::Result<()> {
    let mut terminal = terminal::TerminalSession::new()?;
    App::new(commands, updates)
        .run(&mut terminal.terminal)
        .await
}

struct App {
    view: AppView,
    commands: Sender<AppCommand>,
    updates: Receiver<AppEvent>,
}

impl App {
    fn new(commands: Sender<AppCommand>, updates: Receiver<AppEvent>) -> Self {
        Self {
            view: AppView::new(),
            commands,
            updates,
        }
    }

    async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut events = EventStream::new();
        let mut ticks = tokio::time::interval(Duration::from_millis(100));
        loop {
            terminal.draw(|frame| self.view.render(frame))?;
            tokio::select! {
                _ = ticks.tick() => self.view.tick(),
                update = self.updates.recv() => {
                    let Some(update) = update else { return Ok(()); };
                    self.view.apply_event(update);
                }
                event = events.next() => {
                    let Some(event) = event else { return Ok(()); };
                    match event? {
                        Event::Mouse(mouse) => self.view.handle_mouse(mouse),
                        Event::Key(key) if key.kind == KeyEventKind::Press => {
                            if self.view.handle_popup_key(key.code) || self.view.handle_output_key(key.code) {
                                continue;
                            }
                            match input::action(key.code) {
                                Some(Action::Quit) => return Ok(()),
                                Some(Action::Command(command)) => {
                                    self.commands.send(command).await.map_err(io::Error::other)?;
                                }
                                None => {}
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
