use axum::{routing, Router};
use std::net::SocketAddr;

use canoe::controller::FundController;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    // Initialize color error reports
    color_eyre::install()?;

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Run main function
    run().await?;

    Ok(())
}

async fn run() -> color_eyre::eyre::Result<()> {
    // Controller
    let app = Router::new()
        // `GET /funds`: returns a list of funds filtered by `name`, `fund_manager`, or `year`.
        .route("/funds", routing::get(FundController::list))
        // `PUT /funds/:id`: updates all attributes of a fund.
        .route("/funds/:id", routing::put(FundController::update));

    // Run with hyper
    let addr = SocketAddr::from(([127, 0, 0, 1], 2908));
    tracing::debug!("listening on {}", addr);
    match axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(color_eyre::Report::new(e)),
    }
}
