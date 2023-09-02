use sqlx::SqlitePool;

pub mod controller;
pub mod db;
pub mod events;
pub mod model;

/// Represents the application state.
pub struct AppState {
    pub db: SqlitePool,
    pub tasks: events::Tasks,
}
