#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    // Initialize color error reports
    color_eyre::install()?;

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Read environment variables
    let config = canoe::config::read()?;

    // Initialize database
    let db = canoe::db::init().await?;

    // Initialize the tasks queue
    let tasks = canoe::events::init().await?;

    // Run main function
    canoe::app::run(db, tasks, config.addr).await?;

    Ok(())
}
