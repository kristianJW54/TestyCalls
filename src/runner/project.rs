use crate::{
    messages::AppEvent,
    project::{Project, ProjectPhase},
};
use std::{io, path::Path, time::Duration};
use tokio::{process::Command, sync::mpsc::Sender};

pub(super) async fn open(mut project: Project, events: &Sender<AppEvent>) {
    if events
        .send(AppEvent::Project(project.clone()))
        .await
        .is_err()
    {
        return;
    }
    tokio::time::sleep(Duration::from_millis(350)).await;
    let result = async {
        let path = tokio::fs::canonicalize(&project.selected_path).await?;
        if !path.is_dir() {
            return Err(io::Error::other("Select a project directory, not a file"));
        }
        project.selected_path = path;
        project.phase = ProjectPhase::Validating;
        let _ = events.send(AppEvent::Project(project.clone())).await;
        let (metadata, _) = tokio::join!(
            metadata(&project.selected_path),
            tokio::time::sleep(Duration::from_millis(350))
        );
        let metadata = metadata?;
        let root = metadata
            .get("workspace_root")
            .and_then(|value| value.as_str())
            .ok_or_else(|| io::Error::other("Cargo metadata did not include a workspace root"))?;
        project.workspace_root = Some(root.into());
        project.metadata = Some(metadata);
        Ok::<(), io::Error>(())
    }
    .await;
    match result {
        Ok(()) => project.phase = ProjectPhase::Connected,
        Err(error) => {
            project.phase = ProjectPhase::Failed;
            project.error = Some(error.to_string());
        }
    }
    let _ = events.send(AppEvent::Project(project)).await;
}

async fn metadata(path: &Path) -> io::Result<serde_json::Value> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .current_dir(path)
        .kill_on_drop(true)
        .output()
        .await?;
    if !output.status.success() {
        return Err(io::Error::other(
            String::from_utf8_lossy(&output.stderr).trim().to_owned(),
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(io::Error::other)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn opens_external_workspace_member_and_keeps_both_paths() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "testycalls-project-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(root.join("member/src")).unwrap();
        std::fs::write(
            root.join("Cargo.toml"),
            "[workspace]\nmembers = [\"member\"]\nresolver = \"2\"\n",
        )
        .unwrap();
        std::fs::write(
            root.join("member/Cargo.toml"),
            "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )
        .unwrap();
        std::fs::write(root.join("member/src/lib.rs"), "").unwrap();
        let (sender, mut receiver) = tokio::sync::mpsc::channel(8);
        open(Project::new(root.join("member")), &sender).await;
        let mut projects = Vec::new();
        while let Ok(AppEvent::Project(project)) = receiver.try_recv() {
            projects.push(project);
        }
        std::fs::remove_dir_all(&root).unwrap();
        assert_eq!(
            projects
                .iter()
                .map(|project| project.phase)
                .collect::<Vec<_>>(),
            [
                ProjectPhase::Opening,
                ProjectPhase::Validating,
                ProjectPhase::Connected
            ]
        );
        let project = projects.last().unwrap();
        assert!(project.selected_path.ends_with("member"));
        assert!(!project.workspace_root.as_ref().unwrap().ends_with("member"));
        assert_eq!(
            project.metadata.as_ref().unwrap()["packages"][0]["name"],
            "fixture"
        );
    }

    #[tokio::test]
    async fn nonexistent_path_reports_failure() {
        let path = std::env::temp_dir().join(format!("testycalls-missing-{}", std::process::id()));
        let (sender, mut receiver) = tokio::sync::mpsc::channel(8);
        open(Project::new(path), &sender).await;
        assert!(matches!(
            receiver.recv().await,
            Some(AppEvent::Project(Project {
                phase: ProjectPhase::Opening,
                ..
            }))
        ));
        let Some(AppEvent::Project(project)) = receiver.recv().await else {
            panic!("missing failure");
        };
        assert_eq!(project.phase, ProjectPhase::Failed);
        assert!(project.error.is_some());
    }
}
