use axum::{
    Json,
    extract::{Query, rejection::QueryRejection},
};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::app::error::AppError;
#[derive(Deserialize)]
pub struct SearchParams {
    pub limit: u32,
    pub q: String,
}

pub async fn search(
    params: Result<Query<SearchParams>, QueryRejection>,
) -> Result<Json<Value>, AppError> {
    match params {
        Ok(params) => {
            crate::domain::validation::validate_query(&params.q)?;
            crate::domain::validation::validate_limit(params.limit)?;
            Ok(Json(json!({"limit": 10, "q": "hi"})))
        }
        Err(_) => {
            return Err(AppError::BadRequest {
                message: "failed to parse query".into(),
            });
        }
    }
}
