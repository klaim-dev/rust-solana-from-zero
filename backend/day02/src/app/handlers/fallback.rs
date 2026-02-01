use crate::app::error::AppError;

pub async fn fallback_404() -> AppError {
    AppError::NotFound {
        message: "route not found".into(),
    }
}
