use std::{sync::Arc, time::Duration};

use color_eyre::eyre::Result;
use queues::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use tokio::sync::Mutex;

use crate::model::FundRepository;

/// Represents a serializable event.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    pub name: String,
    pub payload: String,
}

impl Event {
    /// Creates a new event.
    pub fn new(name: String, payload: String) -> Self {
        Self { name, payload }
    }
}

/// Represents a queue of events.
#[derive(Debug, Clone, Default)]
pub struct Tasks {
    queue: Arc<Mutex<Queue<Event>>>,
}

impl Tasks {
    /// Creates a new Tasks instance.
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(Queue::new())),
        }
    }
    /// Pushes a new event to the queue.
    pub async fn emit(&mut self, event: String, payload: String) -> Result<()> {
        tracing::info!("Emitting event: {}", event);
        let mut queue = self.queue.lock().await;
        match queue.add(Event::new(event, payload)) {
            Ok(_) => Ok(()),
            Err(e) => Err(color_eyre::eyre::eyre!(
                "Failed to push event to queue: {}",
                e
            )),
        }
    }
}

pub async fn init() -> Result<Tasks> {
    tracing::info!("Starting event loop");
    let tasks = Tasks::new();

    let queue = tasks.queue.clone();
    tokio::spawn(async move {
        tracing::info!("Connecting to database");
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = sqlx::SqlitePool::connect(&database_url)
            .await
            .expect("Failed to connect to db");
        let interval = Duration::from_millis(100);
        loop {
            {
                let mut queue = queue.lock().await;
                if let Ok(event) = queue.remove() {
                    match event.name.as_str() {
                        "fund_created" => {
                            tracing::info!("Processing fund_created");
                            check_duplicates(&db, &event, &mut queue)
                                .await
                                .expect("Failed to check duplicates");
                        }
                        "fund_duplicate" => {
                            tracing::info!("Processing fund_duplicate");
                        }
                        _ => {
                            tracing::info!("Unknown event: {}", event.name);
                        }
                    }
                    tracing::info!("Event: {:?}", event);
                }
            }
            // Sleep for interval seconds
            tokio::time::sleep(interval).await;
        }
    });

    Ok(tasks)
}

#[derive(Debug, Serialize, Deserialize)]
struct FundCreatedPayload {
    id: i64,
}

#[derive(Debug, FromRow)]
struct DuplicateCount {
    count: i64,
}

/// Checks if the fund has already been assigned to a company with the same name or alias.
async fn check_duplicates(
    db: &sqlx::SqlitePool,
    event: &Event,
    queue: &mut Queue<Event>,
) -> Result<()> {
    let payload: FundCreatedPayload = serde_json::from_str(&event.payload)?;
    let fund = FundRepository::new(db).get(payload.id).await?;

    // SELECT count(*) FROM funds WHERE name = "FooBarBax" AND manager = 1 AND id != 50;
    let duplicate_fund_name: DuplicateCount = sqlx::query_as::<_, DuplicateCount>(
        r#"
SELECT count(*) as count FROM funds WHERE name = ? AND manager = ? AND id != ?
"#,
    )
    .bind(fund.name.as_str())
    .bind(fund.manager)
    .bind(fund.id)
    .fetch_one(db)
    .await?;

    // WITH company_funds AS (
    //   SELECT fund_id FROM funds WHERE manager = 1
    // )
    // SELECT count(*) FROM aliases WHERE alias = "FooBarBax" AND fund_id IN (SELECT fund_id FROM company_funds)
    let duplicate_alias: DuplicateCount = sqlx::query_as::<_, DuplicateCount>(
        r#"
WITH company_funds AS (
    SELECT fund_id FROM funds WHERE manager = ?
)
SELECT count(*) as count FROM aliases WHERE alias = ? AND fund_id IN (SELECT fund_id FROM company_funds)
"#,
    )
    .bind(fund.manager)
    .bind(fund.name.to_string())
    .fetch_one(db)
    .await?;

    if duplicate_fund_name.count > 0 || duplicate_alias.count > 0 {
        tracing::info!("Duplicate found for fund: {}", &fund.name);
        queue
            .add(Event::new(
                "fund_duplicate".to_string(),
                serde_json::to_string(&fund)?,
            ))
            .expect("Failed to add event to queue");
    }

    Ok(())
}
