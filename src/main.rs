mod coordinator;
mod messages;
mod runner;
mod test_tree;
mod tui;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    coordinator::Coordinator::new().run().await
}
