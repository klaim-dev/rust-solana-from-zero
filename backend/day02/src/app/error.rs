use crate::domain::error::DomainError;
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    Unprocessable,
    Internal,
    TooManyRequests,
}

#[derive(Debug, Serialize, PartialEq)]
struct ErrorBody {
    code: ErrorCode,
    message: String,
}
#[allow(dead_code)]
#[derive(Debug)]
pub enum AppError {
    BadRequest { message: String },
    Unprocessable { message: String },
    NotFound { message: String },
    Conflict { message: String },
    Internal { message: String },
    Unauthorized,
    Forbidden,
    TooManyRequests,
}

impl AppError {
    fn status(&self) -> StatusCode {
        match self {
            Self::BadRequest { .. } => StatusCode::BAD_REQUEST,
            Self::Unprocessable { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::NotFound { .. } => StatusCode::NOT_FOUND,
            Self::Conflict { .. } => StatusCode::CONFLICT,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            Self::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> ErrorCode {
        match self {
            Self::BadRequest { .. } => ErrorCode::BadRequest,
            Self::Unprocessable { .. } => ErrorCode::Unprocessable,
            Self::NotFound { .. } => ErrorCode::NotFound,
            Self::Conflict { .. } => ErrorCode::Conflict,
            Self::Unauthorized => ErrorCode::Unauthorized,
            Self::Forbidden => ErrorCode::Forbidden,
            Self::TooManyRequests => ErrorCode::TooManyRequests,
            Self::Internal { .. } => ErrorCode::Internal,
        }
    }

    fn message(&self) -> String {
        match self {
            Self::BadRequest { message }
            | Self::Conflict { message }
            | Self::Internal { message }
            | Self::NotFound { message }
            | Self::Unprocessable { message } => message.clone(),

            Self::Forbidden => "forbidden".to_string(),
            Self::TooManyRequests => "too_many_requests".to_string(),
            Self::Unauthorized => "unauthorized".to_string(),
        }
    }
}

impl From<DomainError> for AppError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::Validation { field: _, message } => AppError::Unprocessable { message },
            DomainError::NotFound { entity: _, message } => AppError::NotFound { message },
            DomainError::Conflict { field: _, message } => AppError::Conflict { message },
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status();
        let code = self.code();
        let message = self.message();
        (status, Json(ErrorBody { code, message })).into_response()
    }
}

#[cfg(test)]
mod tests {
    use axum::body::to_bytes;

    use super::*;

    async fn assert_error_mapping(
        err: AppError,
        expected_status: StatusCode,
        expected_code: &str,
        expected_message: &str,
    ) {
        let res = err.into_response();
        let (parts, body) = res.into_parts();
        assert_eq!(parts.status, expected_status);

        let body_bytes = to_bytes(body, usize::MAX).await.unwrap();
        let value: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
        assert_eq!(value["code"], expected_code);
        assert_eq!(value["message"], expected_message);
    }

    #[tokio::test]
    async fn app_error_bad_request_mapping() {
        assert_error_mapping(
            AppError::BadRequest {
                message: "invalid name".into(),
            },
            StatusCode::BAD_REQUEST,
            "bad_request",
            "invalid name",
        )
        .await;
    }

    #[tokio::test]
    async fn app_error_reserved_code_unauthorized() {
        assert_error_mapping(
            AppError::Unauthorized,
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "unauthorized",
        )
        .await;
    }

    #[tokio::test]
    async fn app_error_reserved_code_forbidden() {
        assert_error_mapping(
            AppError::Forbidden,
            StatusCode::FORBIDDEN,
            "forbidden",
            "forbidden",
        )
        .await;
    }

    #[tokio::test]
    async fn app_error_reserved_code_too_many_requests() {
        assert_error_mapping(
            AppError::TooManyRequests,
            StatusCode::TOO_MANY_REQUESTS,
            "too_many_requests",
            "too_many_requests",
        )
        .await;
    }
}
