use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::model::{Fund, ListFundQuery, PartialFund};

#[derive(Serialize, Deserialize)]
pub struct UpdateFund {
    id: u64,
    fund: PartialFund,
}

pub struct FundController;

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
