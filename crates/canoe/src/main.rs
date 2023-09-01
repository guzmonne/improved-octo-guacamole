use axum::{routing, Router};
use sqlx::SqlitePool;
use std::{net::SocketAddr, sync::Arc};
use tower_http::{classify::StatusInRangeAsFailures, trace::TraceLayer};

use canoe::controller::FundController;
use canoe::db::init;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    // Initialize color error reports
    color_eyre::install()?;

    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Initialize database
    let db = init().await?;

    // Run main function
    let mut host = std::env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("2908".to_string());

    if host == "localhost" {
        host = "127.0.0.1".to_string();
    }

    run(db, &host, &port).await?;

    Ok(())
}

async fn run(db: SqlitePool, host: &str, port: &str) -> color_eyre::eyre::Result<()> {
    // Controller
    let app = Router::new()
        // `GET /funds`: returns a list of funds filtered by `name`, `manager`, or `year`.
        .route("/funds", routing::get(FundController::list))
        // `POST /funds`: creates a new fund.
        .route("/funds", routing::post(FundController::create))
        // `GET /funds/:id`: returns a fund by id.
        .route("/funds/:id", routing::get(FundController::read))
        // `PUT /funds/:id`: updates all attributes of a fund.
        .route("/funds/:id", routing::put(FundController::update))
        // Configure the app state
        .with_state(Arc::new(canoe::AppState { db }))
        // Configure HTTP tracing
        .layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
        ));
    // Run with hyper
    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    tracing::debug!("listening on {}", addr);
    match axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(color_eyre::Report::new(e)),
    }
}
