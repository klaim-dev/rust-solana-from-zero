use axum::Router;
use axum::routing::get;

use crate::routes::healthz::healthz;
pub fn build_router() -> Router{
    Router::new()
    .route("/healthz", get(healthz))
}