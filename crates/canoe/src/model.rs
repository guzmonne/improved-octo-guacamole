use serde::{Deserialize, Serialize};

/// Represents a partial fund.
#[derive(Serialize, Deserialize)]
pub struct PartialFund {
    name: Option<String>,
    fund_manager: Option<u64>,
    year: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ListFundQuery {
    Name(String),
    FundManager(String),
    Year(u16),
    None,
}

/// Represents a fund.
#[derive(Serialize, Deserialize)]
pub struct Fund {
    id: u64,
    name: String,
    fund_manager: u64,
    year: u16,
}

impl Fund {
    /// Get a fund by id.
    pub async fn get(id: u64) -> Option<Self> {
        tracing::info!("Getting fund with id: {}", id);
        Some(Self {
            id,
            name: "Fund 1".to_string(),
            fund_manager: 1,
            year: 2020,
        })
    }

    /// List funds with a query.
    pub async fn list(query: ListFundQuery) -> Vec<Self> {
        tracing::info!("Listing funds with query: {:?}", query);
        vec![Self {
            id: 1,
            name: "Fund 1".to_string(),
            fund_manager: 1,
            year: 2020,
        }]
    }

    /// Update a fund with a partial fund.
    pub async fn update(&mut self, fund: PartialFund) {
        if let Some(name) = fund.name {
            self.name = name;
        }
        if let Some(fund_manager) = fund.fund_manager {
            self.fund_manager = fund_manager;
        }
        if let Some(year) = fund.year {
            self.year = year;
        }
    }
}
