use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use reqwest::Client;
use std::net::SocketAddr;
use tower::ServiceExt;

async fn create_test_app() -> (Router, String) {
    let api_base = "http://localhost:9999".to_string(); // Mock server
    let api_key = "test-key".to_string();
    let bind_addr = "127.0.0.1:0".parse::<SocketAddr>().unwrap();
    
    let state = crate::AppState {
        api_base: api_base.clone(),
        api_key,
        client: Client::builder().build().unwrap(),
    };

    let app = Router::new()
        .route("/", axum::routing::get(|| async { "Ghost Dashboard API" }))
        .route("/health", axum::routing::get(crate::forward_public))
        .route("/api/v1/info", axum::routing::get(crate::forward_public))
        .with_state(state);

    (app, api_base)
}

#[tokio::test]
async fn test_health_endpoint() {
    let (app, _) = create_test_app().await;
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_info_endpoint() {
    let (app, _) = create_test_app().await;
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/info")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_root_endpoint() {
    let (app, _) = create_test_app().await;
    
    let response = app
        .oneshot(
            Request::builder()
                .uri("/")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
