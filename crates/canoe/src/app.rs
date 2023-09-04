use axum::{routing, Router};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_http::{classify::StatusInRangeAsFailures, trace::TraceLayer};

use crate::controller::FundController;

pub async fn run(
    db: SqlitePool,
    tasks: crate::events::Tasks,
    addr: SocketAddr,
) -> color_eyre::eyre::Result<()> {
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
        .with_state(crate::AppState::new(db, tasks))
        // Configure HTTP tracing
        .layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
        ));
    // Run with hyper
    tracing::debug!("listening on {}", addr);
    match axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(color_eyre::Report::new(e)),
    }
}
