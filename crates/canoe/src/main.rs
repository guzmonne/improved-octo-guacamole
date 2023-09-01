use axum::{http::StatusCode, routing, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

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

#[derive(Serialize, Deserialize)]
struct Fund {
    id: u64,
    name: String,
    fund_manager: u64,
    year: u16,
}

#[derive(Serialize, Deserialize, Debug)]
enum ListFundQuery {
    Name(String),
    FundManager(String),
    Year(u16),
    None,
}

impl Fund {
    pub async fn get(id: u64) -> Option<Self> {
        tracing::info!("Getting fund with id: {}", id);
        Some(Self {
            id,
            name: "Fund 1".to_string(),
            fund_manager: 1,
            year: 2020,
        })
    }

    pub async fn list(query: ListFundQuery) -> Vec<Self> {
        tracing::info!("Listing funds with query: {:?}", query);
        vec![Self {
            id: 1,
            name: "Fund 1".to_string(),
            fund_manager: 1,
            year: 2020,
        }]
    }

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

#[derive(Serialize, Deserialize)]
struct PartialFund {
    name: Option<String>,
    fund_manager: Option<u64>,
    year: Option<u16>,
}

#[derive(Serialize, Deserialize)]
struct UpdateFund {
    id: u64,
    fund: PartialFund,
}

struct FundController;

impl FundController {
    pub async fn list() -> (StatusCode, Json<Vec<Fund>>) {
        (StatusCode::OK, Json(Fund::list(ListFundQuery::None).await))
    }

    pub async fn update(Json(body): Json<UpdateFund>) -> (StatusCode, Json<Fund>) {
        let mut fund = Fund::get(body.id).await.unwrap();
        fund.update(body.fund).await;
        (StatusCode::OK, Json(fund))
    }
}
