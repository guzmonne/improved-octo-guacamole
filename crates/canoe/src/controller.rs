use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};

use crate::model::{Fund, FundRepository, ListFundQuery, PartialFund};
use crate::AppState;

#[derive(Serialize, Deserialize)]
pub struct UpdateFund {
    id: i64,
    fund: PartialFund,
}

#[derive(Serialize, Deserialize)]
pub struct CreateFund {
    name: String,
    manager: i64,
    start_year: u16,
}

pub struct FundController;

impl FundController {
    /// Handle GET /funds
    pub async fn list(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Vec<Fund>>) {
        (
            StatusCode::OK,
            Json(
                match FundRepository::new(&state.db)
                    .list(ListFundQuery::None)
                    .await
                {
                    Ok(funds) => funds,
                    Err(e) => {
                        tracing::error!("Failed to list funds with error: {}", e);
                        vec![]
                    }
                },
            ),
        )
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
        Json(body): Json<UpdateFund>,
    ) -> (StatusCode, Json<Option<Fund>>) {
        match FundRepository::new(&state.db)
            .update(body.id, body.fund)
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
