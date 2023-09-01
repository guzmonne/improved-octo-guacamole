use color_eyre::Result;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

/// Represents a partial fund.
#[derive(Serialize, Deserialize, Debug)]
pub struct PartialFund {
    name: Option<String>,
    fund_manager: Option<i64>,
    year: Option<u16>,
}

/// Enumerates the possible queries for listing funds.
#[derive(Serialize, Deserialize, Debug)]
pub enum ListFundQuery {
    Name(String),
    FundManager(String),
    Year(u16),
    None,
}

/// Represents a fund.
#[derive(Serialize, Deserialize, Clone, FromRow, Debug)]
pub struct Fund {
    id: i64,
    name: String,
    manager: i64,
    year: u16,
}

impl Fund {
    /// Creates a new fund
    pub fn new(id: i64, name: String, manager: i64, year: u16) -> Self {
        Self {
            id,
            name,
            manager,
            year,
        }
    }

    /// Creates a Fund from a partial fund.

    /// Update a fund with a partial fund.
    pub fn update(&mut self, fund: PartialFund) {
        if let Some(name) = fund.name {
            self.name = name;
        }
        if let Some(fund_manager) = fund.fund_manager {
            self.manager = fund_manager;
        }
        if let Some(year) = fund.year {
            self.year = year;
        }
    }
}

pub struct FundRepository<'a> {
    db: &'a SqlitePool,
}

impl<'a> FundRepository<'a> {
    pub fn new(db: &'a SqlitePool) -> Self {
        Self { db }
    }

    /// Get a fund by id.
    pub async fn get(&self, id: i64) -> Result<Fund> {
        tracing::info!("Getting fund with id: {}", id);
        match sqlx::query_as::<_, Fund>("SELECT id, name, manager, year FROM funds WHERE id = ?")
            .bind(id)
            .fetch_optional(self.db)
            .await
        {
            Ok(Some(fund)) => Ok(fund),
            Ok(None) => Err(color_eyre::eyre::eyre!("Fund not found")),
            Err(_) => {
                tracing::error!("Failed to get fund with id: {}", id);
                Err(color_eyre::eyre::eyre!(
                    "Failed to get fund with id: {}",
                    id
                ))
            }
        }
    }

    /// List funds with a query.
    pub async fn list(&self, query: ListFundQuery) -> Result<Vec<Fund>> {
        tracing::info!("Listing funds with query: {:?}", query);
        // let result = sqlx::query("SELECT * FROM funds").fetch_all(db).await?;
        match sqlx::query_as::<_, Fund>("SELECT id, name, manager, year FROM funds")
            .fetch_all(self.db)
            .await
        {
            Ok(funds) => Ok(funds),
            Err(e) => {
                tracing::error!("Failed to list funds: {}", e);
                Err(e.into())
            }
        }
    }

    /// Creates a new fund.
    pub async fn create(&self, name: String, manager: i64, year: u16) -> Result<Fund> {
        tracing::info!("Creating fund: {} {} {}", name, manager, year);
        match sqlx::query_as::<_, Fund>(
            r#"
INSERT INTO funds (name, manager, year) VALUES (?, ?, ?)
RETURNING id, name, manager, year
"#,
        )
        .bind(name.as_str())
        .bind(manager)
        .bind(year)
        .fetch_one(self.db)
        .await
        {
            Ok(fund) => Ok(fund),
            Err(e) => {
                tracing::error!("Failed to create fund: {}", e);
                Err(e.into())
            }
        }
    }

    /// Updates a fund.
    pub async fn update(&self, id: i64, partial: PartialFund) -> Result<Fund> {
        tracing::info!("Getting fund: {}", id);

        let mut fund = self.get(id).await?;
        fund.update(partial);

        tracing::info!("Storing changes with: {:?}", fund);
        self.store(fund).await
    }

    /// Stores a fund.
    async fn store(&self, fund: Fund) -> Result<Fund> {
        tracing::info!("Storing fund: {:?}", fund);
        match sqlx::query(
            r#"
INSERT INTO funds (name, manager, year) VALUES (?, ?, ?)")
ON CONFLICT(id) DO UPDATE SET name = excluded.name, manager = excluded.manager, year = excluded.year
"#,
        )
        .bind(fund.name.as_str())
        .bind(fund.manager)
        .bind(fund.year)
        .execute(self.db)
        .await
        {
            Ok(_) => Ok(fund),
            Err(e) => {
                tracing::error!("Failed to store fund: {}", e);
                Err(e.into())
            }
        }
    }
}
