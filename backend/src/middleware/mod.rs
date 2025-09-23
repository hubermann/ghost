use axum::{
    extract::Request,
    http::{HeaderValue, Method, StatusCode},
    middleware::Next,
    response::Response,
};
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, warn};

pub fn cors_layer(allowed_origins: Vec<String>) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any)
        .max_age(Duration::from_secs(3600));

    if allowed_origins.iter().any(|origin| origin == "*") {
        cors = cors.allow_origin(Any);
    } else {
        for origin in allowed_origins {
            if let Ok(header_value) = HeaderValue::from_str(&origin) {
                cors = cors.allow_origin(header_value);
            }
        }
    }

    cors
}

/// Middleware para logging detallado de requests
pub async fn request_logger(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    let start = std::time::Instant::now();

    info!(
        method = %method,
        uri = %uri,
        "request started"
    );

    let response = next.run(req).await;
    let duration = start.elapsed();

    let status = response.status();
    if status.is_success() {
        info!(
            method = %method,
            uri = %uri,
            status = %status,
            duration_ms = %duration.as_millis(),
            "request completed successfully"
        );
    } else {
        warn!(
            method = %method,
            uri = %uri,
            status = %status,
            duration_ms = %duration.as_millis(),
            "request completed with error status"
        );
    }

    response
}

/// Middleware de seguridad básica
pub async fn security_headers(req: Request, next: Next) -> Response {
    let mut response = next.run(req).await;

    let headers = response.headers_mut();
    headers.insert("X-Content-Type-Options", HeaderValue::from_static("nosniff"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert("X-XSS-Protection", HeaderValue::from_static("1; mode=block"));
    headers.insert(
        "Strict-Transport-Security",
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    );

    response
}

/// Middleware para limitar el tamaño del body
pub async fn request_size_limit(req: Request, next: Next) -> Response {
    if let Some(content_length) = req.headers().get("content-length") {
        if let Ok(length_str) = content_length.to_str() {
            if let Ok(length) = length_str.parse::<usize>() {
                const MAX_SIZE: usize = 2 * 1024 * 1024; // 2MB
                if length > MAX_SIZE {
                    return Response::builder()
                        .status(StatusCode::PAYLOAD_TOO_LARGE)
                        .body("Request body too large".into())
                        .unwrap();
                }
            }
        }
    }

    next.run(req).await
}