use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use day02::infra::http::build_router;
use serde_json::Value;
use tower::ServiceExt;

async fn request_json(method: &str, path: &str, body: Body) -> (StatusCode, Value, Option<String>) {
    let app = build_router();
    let req = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .body(body)
        .unwrap();

    let response = app.oneshot(req).await.unwrap();
    let (parts, body) = response.into_parts();
    let status = parts.status;
    let content_type = parts
        .headers
        .get("content-type")
        .and_then(|value| value.to_str().ok())
        .map(|value| value.to_string());

    let body_bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    let json: Value = serde_json::from_slice(&body_bytes).unwrap();

    (status, json, content_type)
}

fn assert_error(value: &Value, status: StatusCode, expected_status: StatusCode, code: &str, message: &str) {
    assert_eq!(status, expected_status);
    assert_eq!(value["code"], code);
    assert_eq!(value["message"], message);
}

#[tokio::test]
async fn check_healthz() {
    let (status, json, content_type) = request_json("GET", "/healthz", Body::empty()).await;

    assert_eq!(status, StatusCode::OK);
    assert!(content_type.unwrap_or_default().starts_with("application/json"));
    assert_eq!(json["ok"], true);
}

#[tokio::test]
async fn path_nope() {
    let (status, value, _) = request_json("GET", "/nope", Body::empty()).await;

    assert_error(
        &value,
        status,
        StatusCode::NOT_FOUND,
        "not_found",
        "route not found",
    );
}

#[tokio::test]
async fn get_user_happy_path() {
    let (status, value, _) = request_json("GET", "/users/1", Body::empty()).await;

    assert_eq!(value["id"], 1);
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn get_user_fail_path() {
    let (status, value, _) = request_json("GET", "/users/0", Body::empty()).await;

    assert_error(
        &value,
        status,
        StatusCode::NOT_FOUND,
        "not_found",
        "user not found",
    );
}

#[tokio::test]
async fn search_happy_path() {
    let (status, value, _) = request_json("GET", "/search?limit=10&q=hi", Body::empty()).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(value["limit"], 10);
    assert_eq!(value["q"], "hi");
}

#[tokio::test]
async fn search_bad_request_path() {
    let (status, value, _) = request_json("GET", "/search?limit=ABC&q=hi", Body::empty()).await;

    assert_error(
        &value,
        status,
        StatusCode::BAD_REQUEST,
        "bad_request",
        "failed to parse query",
    );
}

#[tokio::test]
async fn search_unprocessable_path() {
    let (status, value, _) = request_json("GET", "/search?limit=0&q=hi", Body::empty()).await;

    assert_error(
        &value,
        status,
        StatusCode::UNPROCESSABLE_ENTITY,
        "unprocessable",
        "limit must be 1..=100",
    );
}

#[tokio::test]
async fn search_echo_happy_path() {
    let body = Body::from(r#"{"email":"a@example.com"}"#);
    let (status, value, _) = request_json("POST", "/echo", body).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(value["email"], "a@example.com");
}

#[tokio::test]
async fn search_echo_invalid_json() {
    let body = Body::from(r#"{"email":"a@example.com""#);
    let (status, value, _) = request_json("POST", "/echo", body).await;

    assert_error(
        &value,
        status,
        StatusCode::BAD_REQUEST,
        "bad_request",
        "invalid json",
    );
}

#[tokio::test]
async fn search_echo_conflict() {
    let body = Body::from(r#"{"email":"taken@example.com"}"#);
    let (status, value, _) = request_json("POST", "/echo", body).await;

    assert_error(
        &value,
        status,
        StatusCode::CONFLICT,
        "conflict",
        "email already taken",
    );
}

#[tokio::test]
async fn search_echo_internal() {
    let body = Body::from(r#"{"email":"fail@example.com"}"#);
    let (status, value, _) = request_json("POST", "/echo", body).await;

    assert_error(
        &value,
        status,
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal",
        "internal error",
    );
}
