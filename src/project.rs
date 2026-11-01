use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum ProjectPhase {
    #[default]
    Opening,
    Validating,
    Connected,
    Failed,
}

#[derive(Debug, Clone)]
pub(crate) struct Project {
    pub selected_path: PathBuf,
    pub workspace_root: Option<PathBuf>,
    pub phase: ProjectPhase,
    pub error: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl Project {
    pub fn new(selected_path: PathBuf) -> Self {
        Self {
            selected_path,
            workspace_root: None,
            phase: ProjectPhase::Opening,
            error: None,
            metadata: None,
        }
    }
}
