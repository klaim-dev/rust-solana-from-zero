use crate::app::error::AppError;
use axum::{Json, extract::Path};
use serde_json::{Value, json};

pub async fn get_user(Path(id): Path<u64>) -> Result<Json<Value>, AppError> {
    crate::domain::validation::validate_user(id)?;
    Ok(Json(json!({
        "id": id
    })))
}
