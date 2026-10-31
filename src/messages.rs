#[derive(Debug, Clone, Copy)]
pub(crate) enum AppCommand {
    RefreshList,
    RunTests,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum RunnerCommand {
    RunDemo,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TestListStatus {
    #[default]
    NotLoaded,
    Preparing,
    Compiling,
    Compiled,
    Stale,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RunPhase {
    #[default]
    Idle,
    Preparing,
    Running,
    Completed,
    FailedToRun,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum OutputKind {
    Results,
    Failures,
    Compiler,
}

impl OutputKind {
    pub fn index(self) -> usize {
        match self {
            Self::Results => 0,
            Self::Failures => 1,
            Self::Compiler => 2,
        }
    }
}

#[derive(Debug)]
pub(crate) enum AppEvent {
    RunStarted,
    ListStatus(TestListStatus),
    Progress { completed: usize, total: usize },
    RunFinished,
    Output { kind: OutputKind, line: String },
}
