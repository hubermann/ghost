use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use reqwest::Client;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing::{error, info};
use uuid::Uuid;

mod config;
mod handlers;

use config::Config;

#[derive(Clone)]
pub struct AppState {
    pub api_base: String,
    pub api_key: String,
    pub client: Client,
    pub config: Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // Cargar y validar configuración
    let config = Config::from_env()?;
    config.validate()?;

    // Configurar logging con el nivel especificado
    tracing_subscriber::fmt()
        .with_env_filter(&config.log_level)
        .compact()
        .init();

    let bind_addr: SocketAddr = config.bind_address.parse()?;

    let state = AppState {
        api_base: config.api_base_url.clone(),
        api_key: config.api_key.clone(),
        client: Client::builder()
            .timeout(std::time::Duration::from_secs(config.request_timeout_secs))
            .build()?,
        config: config.clone(),
    };

    let app = Router::new()
        // Endpoints públicos
        .route("/", get(|| async { "Ghost Dashboard API" }))
        .route("/health", get(handlers::health_check))
        .route("/api/health", get(handlers::api_health_check))
        .route("/api/metrics/system", get(handlers::system_metrics))
        .route("/api/v1/info", get(forward_public))
        // Endpoints que requieren autenticación
        .route("/api/analyze", post(forward_auth))
        .route("/api/timeframes/config", get(forward_auth))
        .route("/api/v1/analyze", post(forward_auth))
        .route("/api/v1/historical", post(forward_auth))
        .route("/api/v1/indicators", post(forward_auth))
        .route("/api/v1/compare", post(forward_auth))
        .route("/api/v1/providers/status", get(forward_auth))
        .route("/api/v1/metrics/system", get(forward_auth))
        .route("/api/v1/metrics/reconciliation", get(forward_auth))
        .route("/api/v1/metrics/data_quality", get(forward_auth))
        .route("/api/v1/timeframes/config", get(forward_auth))
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

    // Log detallado de la request entrante
    info!(
        trace_id = %trace_id,
        method = %method,
        path = %path_q,
        url = %url,
        with_auth = %with_auth,
        "proxy request started"
    );

    let body = to_bytes(req.into_body(), s.config.max_request_size).await.unwrap_or_default();

    // Log del body para requests POST/PUT
    if !body.is_empty() {
        if let Ok(body_str) = std::str::from_utf8(&body) {
            info!(
                trace_id = %trace_id,
                body = %body_str,
                "request body"
            );
        }
    }

    let mut r = s.client.request(method.clone(), &url);
    if with_auth {
        r = r.header("Authorization", format!("Bearer {}", s.api_key));
    }
    // Agregar Content-Type header para peticiones POST
    if method == axum::http::Method::POST {
        r = r.header("Content-Type", "application/json");
    }
    let res = r.body(body.to_vec()).send().await;

    match res {
        Ok(up) => {
            let status = up.status();
            let bytes = up.bytes().await.unwrap_or_default();

            // Log de la respuesta
            if let Ok(response_str) = std::str::from_utf8(&bytes) {
                info!(
                    trace_id = %trace_id,
                    status = %status,
                    response = %response_str,
                    "proxy response received"
                );
            }

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

