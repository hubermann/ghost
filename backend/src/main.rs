use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
    Json,
};
use reqwest::Client;
use serde_json::json;
use std::{env, net::SocketAddr};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone)]
struct AppState {
    api_base: String,
    api_key: String,
    client: Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .compact()
        .init();

    let api_base = env::var("INBESTIA_API_URL")?;
    let api_key = env::var("INBESTIA_API_KEY")?;
    let bind_addr: SocketAddr = env::var("BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8081".into())
        .parse()?;

    let state = AppState {
        api_base,
        api_key,
        client: Client::builder().build()?,
    };

    let app = Router::new()
        // públicos
        .route("/", get(|| async { "Ghost Dashboard API" }))
        .route("/health", get(health_check))
        .route("/api/health", get(api_health_check))
        .route("/api/metrics/system", get(system_metrics))
        .route("/api/v1/info", get(forward_public))
        // autenticados
        .route("/api/v1/analyze", post(forward_auth))
        .route("/api/v1/historical", post(forward_auth))
        .route("/api/v1/indicators", post(forward_auth))
        .route("/api/v1/compare", post(forward_auth))
        .route("/api/v1/providers/status", get(forward_auth))
        .route("/api/v1/metrics/system", get(forward_auth))
        .route("/api/v1/metrics/reconciliation", get(forward_auth))
        .route("/api/v1/metrics/data_quality", get(forward_auth))
        .with_state(state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    info!("gateway on http://{bind_addr}");
    axum::serve(tokio::net::TcpListener::bind(bind_addr).await?, app).await?;
    Ok(())
}

async fn forward_public(State(s): State<AppState>, req: Request<Body>) -> impl IntoResponse {
    proxy(s, req, false).await
}

async fn forward_auth(State(s): State<AppState>, req: Request<Body>) -> impl IntoResponse {
    proxy(s, req, true).await
}

async fn proxy(s: AppState, req: Request<Body>, with_auth: bool) -> impl IntoResponse {
    use axum::body::to_bytes;
    let trace_id = Uuid::new_v4().to_string();
    
    let path_q = req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("/");
    let url = format!("{}{}", s.api_base, path_q);
    let method = req.method().clone();
    let body = to_bytes(req.into_body(), 2 * 1024 * 1024).await.unwrap_or_default();

    let mut r = s.client.request(method, &url);
    if with_auth {
        r = r.header("Authorization", format!("Bearer {}", s.api_key));
    }
    let res = r.body(body.to_vec()).send().await;

    match res {
        Ok(up) => {
            let status = up.status();
            let bytes = up.bytes().await.unwrap_or_default();
            (status, bytes).into_response()
        }
        Err(e) => {
            error!(error=?e, trace_id=%trace_id, "upstream error");
            let error_response = serde_json::json!({
                "error": {
                    "code": "UPSTREAM_TIMEOUT",
                    "message": "Servicio no disponible",
                    "trace_id": trace_id
                }
            });
            (
                StatusCode::BAD_GATEWAY,
                axum::Json(error_response),
            )
                .into_response()
        }
    }
}

// Obtener métricas del sistema de inBestia
async fn system_metrics(State(state): State<AppState>) -> impl IntoResponse {
    let trace_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now();
    
    // Intentar obtener métricas de la API de inBestia
    match state.client
        .get(&format!("{}/api/v1/metrics/system", state.api_base))
        .header("Authorization", format!("Bearer {}", state.api_key))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await 
    {
        Ok(response) if response.status().is_success() => {
            match response.json::<serde_json::Value>().await {
                Ok(metrics) => {
                    info!(trace_id=%trace_id, "system metrics retrieved successfully");
                    Json(json!({
                        "status": "success",
                        "data": metrics,
                        "timestamp": timestamp,
                        "trace_id": trace_id
                    }))
                }
                Err(e) => {
                    error!(trace_id=%trace_id, error=%e, "failed to parse metrics response");
                    Json(json!({
                        "status": "error",
                        "error": format!("Failed to parse metrics: {}", e),
                        "timestamp": timestamp,
                        "trace_id": trace_id
                    }))
                }
            }
        }
        Ok(response) => {
            error!(trace_id=%trace_id, status=%response.status(), "metrics endpoint returned error status");
            Json(json!({
                "status": "error",
                "error": format!("Metrics endpoint returned status: {}", response.status()),
                "timestamp": timestamp,
                "trace_id": trace_id
            }))
        }
        Err(e) => {
            error!(trace_id=%trace_id, error=%e, "system metrics request failed");
            Json(json!({
                "status": "error",
                "error": format!("Connection failed: {}", e),
                "timestamp": timestamp,
                "trace_id": trace_id
            }))
        }
    }
}

// Health check básico del gateway
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "ghost-dashboard-gateway",
        "timestamp": Utc::now()
    }))
}

// Health check de la API de inBestia
async fn api_health_check(State(state): State<AppState>) -> impl IntoResponse {
    let trace_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now();
    
    // Intentar conectar a la API de inBestia
    match state.client
        .get(&format!("{}/health", state.api_base))
        .header("Authorization", format!("Bearer {}", state.api_key))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await 
    {
        Ok(response) if response.status().is_success() => {
            info!(trace_id=%trace_id, "inbestia api health check successful");
            Json(json!({
                "status": "healthy",
                "inbestia_api": "available",
                "gateway": "healthy",
                "timestamp": timestamp,
                "trace_id": trace_id
            }))
        }
        Ok(response) => {
            error!(trace_id=%trace_id, status=%response.status(), "inbestia api returned error status");
            Json(json!({
                "status": "unhealthy",
                "inbestia_api": "error",
                "gateway": "healthy",
                "error": format!("API returned status: {}", response.status()),
                "timestamp": timestamp,
                "trace_id": trace_id
            }))
        }
        Err(e) => {
            error!(trace_id=%trace_id, error=%e, "inbestia api health check failed");
            Json(json!({
                "status": "unhealthy",
                "inbestia_api": "unavailable",
                "gateway": "healthy",
                "error": format!("Connection failed: {}", e),
                "timestamp": timestamp,
                "trace_id": trace_id
            }))
        }
    }
}
