mod coordinator;
mod messages;
mod project;
mod runner;
mod test_tree;
mod tui;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut args = std::env::args_os().skip(1);
    let selected = match args.next() {
        Some(argument) if argument == "--help" || argument == "-h" => {
            println!("Usage: testycalls [PROJECT_DIRECTORY]\nDefaults to the current directory.");
            return Ok(());
        }
        Some(argument) => std::path::PathBuf::from(argument),
        None => std::env::current_dir()?,
    };
    if args.next().is_some() {
        return Err(std::io::Error::other(
            "Usage: testycalls [PROJECT_DIRECTORY]",
        ));
    }
    let selected = std::path::absolute(selected)?;
    coordinator::Coordinator::new(project::Project::new(selected))
        .run()
        .await
}
