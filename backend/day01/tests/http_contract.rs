use serde_json::Value;
use axum::{body::Body, http::{Request, StatusCode}};
use day01::app::build_router;
use tower::ServiceExt;

#[tokio::test]
async fn check_healthz() {
    let app = build_router();
    let req = Request::builder()
    .method("GET")
    .uri("/healthz")
    .body(Body::default())
    .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    let (parts, body) = response.into_parts();

    let status = parts.status;
    let ct = parts.headers.get("content-type").unwrap().to_str().unwrap();

    let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    assert_eq!(status, StatusCode::OK);
    assert!(ct.starts_with("application/json"));
    assert_eq!(json["ok"], true );
}

#[tokio::test]
async fn path_nope() {
    let app = build_router();
    let req = Request::builder()
    .method("GET")
    .uri("/nope")
    .body(Body::default())
    .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    let (parts, _body) = response.into_parts();

    assert_eq!(parts.status, StatusCode::NOT_FOUND);
}