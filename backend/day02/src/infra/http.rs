use crate::app::handlers::fallback::fallback_404;
use crate::app::handlers::search::search;
use crate::app::handlers::users::get_user;
use axum::Router;
use axum::routing::{get, post};

use crate::app::handlers::healthz::healthz;
use crate::app::handlers::echo::echo;
pub fn build_router() -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/users/{id}", get(get_user))
        .route("/search", get(search))
        .route("/echo", post(echo))
        .fallback(fallback_404)
}
