use axum::{Json, extract::rejection::JsonRejection};
use serde::{Deserialize, Serialize};

use crate::app::error::AppError;

#[derive(Deserialize, Serialize)]
pub struct EchoBody { email: String }


pub async fn echo(params: Result<Json<EchoBody>, JsonRejection>) -> Result<Json<EchoBody>, AppError> {
    match params {
        Ok(params) => {
            if params.email == "taken@example.com" {
                return Err(AppError::Conflict { message: "email already taken".into() });
            }

            if params.email == "fail@example.com" {
                return Err(AppError::Internal { message: "internal error".into() });
            }
            Ok(params)
        },
        Err(_) => return Err(AppError::BadRequest { message: "invalid json".into() })
    }
}
