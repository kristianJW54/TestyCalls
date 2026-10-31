use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
};
use ratatui::DefaultTerminal;
use std::io;

pub(super) struct TerminalSession {
    pub terminal: DefaultTerminal,
}

impl TerminalSession {
    pub fn new() -> io::Result<Self> {
        let session = Self {
            terminal: ratatui::try_init()?,
        };
        execute!(io::stdout(), EnableMouseCapture)?;
        Ok(session)
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = execute!(io::stdout(), DisableMouseCapture);
        ratatui::restore();
    }
}
