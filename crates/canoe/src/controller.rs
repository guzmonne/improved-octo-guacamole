use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::model::{FilterFundQuery, Fund, FundRepository, PartialFund};
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct CreateFund {
    name: String,
    manager: i64,
    start_year: u16,
}

#[derive(Serialize, Deserialize)]
pub struct ListFundQuery {
    filter: Option<String>,
    value: Option<String>,
}

pub struct FundController;

impl FundController {
    /// Handle GET /funds
    pub async fn list(
        State(state): State<Arc<AppState>>,
        Query(query): Query<ListFundQuery>,
    ) -> (StatusCode, Json<Vec<Fund>>) {
        let filter = if query.filter.is_some() && query.value.is_some() {
            match query.filter {
                Some(ref filter) if filter == "name" => {
                    FilterFundQuery::Name(query.value.clone().unwrap())
                }
                Some(ref filter) if filter == "manager" => {
                    FilterFundQuery::Manager(query.value.clone().unwrap())
                }
                Some(ref filter) if filter == "start_year" => {
                    FilterFundQuery::StartYear(query.value.clone().unwrap().parse().unwrap())
                }
                _ => FilterFundQuery::None,
            }
        } else {
            FilterFundQuery::None
        };
        match FundRepository::new(&state.db).list(filter).await {
            Ok(funds) => (StatusCode::OK, Json(funds)),
            Err(e) => {
                tracing::error!("Failed to list funds with error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
            }
        }
    }

    /// Handle POST /funds
    pub async fn create(
        State(state): State<Arc<AppState>>,
        Json(body): Json<CreateFund>,
    ) -> (StatusCode, Json<Option<Fund>>) {
        match FundRepository::new(&state.db)
            .create(body.name, body.manager, body.start_year)
            .await
        {
            Ok(fund) => (StatusCode::CREATED, Json(Some(fund))),
            Err(e) => {
                tracing::error!("Failed to create fund with error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
            }
        }
    }

    /// Handle GET /funds/:id
    pub async fn read(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i64>,
    ) -> (StatusCode, Json<Option<Fund>>) {
        match FundRepository::new(&state.db).get(id).await {
            Ok(fund) => (StatusCode::OK, Json(Some(fund))),
            Err(e) => {
                tracing::error!("Failed to get fund with error: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
            }
        }
    }

    /// Handle PUT /funds/:id
    pub async fn update(
        State(state): State<Arc<AppState>>,
        Path(id): Path<i64>,
        Json(partial_fund): Json<PartialFund>,
    ) -> (StatusCode, Json<Option<Fund>>) {
        match FundRepository::new(&state.db)
            .update(id, partial_fund)
            .await
        {
            Ok(fund) => (StatusCode::OK, Json(Some(fund))),
            Err(e) => {
                tracing::error!("Failed to get fund: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
            }
        }
    }
}
