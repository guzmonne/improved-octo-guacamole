use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::sync::Mutex;

pub mod app;
pub mod config;
pub mod controller;
pub mod db;
pub mod events;
pub mod model;

/// Represents the application state.
#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub tasks: Arc<Mutex<events::Tasks>>,
}

impl AppState {
    /// Creates a new AppState instance.
    pub fn new(db: SqlitePool, tasks: events::Tasks) -> Self {
        Self {
            db,
            tasks: Arc::new(Mutex::new(tasks)),
        }
    }
}
