use super::*;
use std::time::Duration;

#[tokio::test]
async fn commands_and_events_cross_the_bridge_and_shutdown_completes() {
    let (app_tx, app_rx) = mpsc::channel(32);
    let (update_tx, mut update_rx) = mpsc::channel(128);
    let (runner_tx, mut runner_rx) = mpsc::channel(32);
    let (event_tx, event_rx) = mpsc::channel(128);
    let coordinator = Coordinator {
        project: Project::new(std::env::current_dir().unwrap()),
        app: tokio::spawn(async move {
            app_tx.send(AppCommand::RefreshList).await.unwrap();
            assert!(matches!(update_rx.recv().await, Some(AppEvent::RunStarted)));
            Ok(())
        }),
        app_commands: app_rx,
        app_updates: update_tx,
        runner_commands: runner_tx,
        runner_events: event_rx,
        runner: tokio::spawn(async move {
            assert!(matches!(
                runner_rx.recv().await,
                Some(RunnerCommand::RunDemo)
            ));
            event_tx.send(AppEvent::RunStarted).await.unwrap();
            std::future::pending::<()>().await;
        }),
        catalogue: None,
    };
    tokio::time::timeout(Duration::from_secs(2), coordinator.run())
        .await
        .expect("shutdown must not hang")
        .unwrap();
}

#[tokio::test]
async fn app_errors_reach_main_without_waiting_for_idle_runner() {
    let (_app_tx, app_rx) = mpsc::channel(32);
    let (update_tx, _update_rx) = mpsc::channel(128);
    let (runner_tx, _runner_rx) = mpsc::channel(32);
    let (_event_tx, event_rx) = mpsc::channel(128);
    let coordinator = Coordinator {
        project: Project::new(std::env::current_dir().unwrap()),
        app: tokio::spawn(async { Err(io::Error::other("terminal failed")) }),
        app_commands: app_rx,
        app_updates: update_tx,
        runner_commands: runner_tx,
        runner_events: event_rx,
        runner: tokio::spawn(std::future::pending()),
        catalogue: None,
    };

    let result = tokio::time::timeout(Duration::from_secs(2), coordinator.run())
        .await
        .expect("shutdown must not hang");
    assert_eq!(result.unwrap_err().to_string(), "terminal failed");
}
