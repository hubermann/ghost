use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use chrono::Utc;
use uuid::Uuid;
use crate::AppState;

/// Health check endpoint específico del gateway
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "ghost-dashboard-gateway",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": Utc::now(),
        "uptime_secs": get_uptime_seconds()
    }))
}

/// Health check que verifica conectividad con la API externa
pub async fn api_health_check(State(state): State<AppState>) -> impl IntoResponse {
    let trace_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now();

    match state.client
        .get(&format!("{}/health", state.api_base))
        .header("Authorization", format!("Bearer {}", state.api_key))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            (StatusCode::OK, Json(json!({
                "status": "healthy",
                "gateway": {
                    "status": "healthy",
                    "version": env!("CARGO_PKG_VERSION"),
                    "uptime_secs": get_uptime_seconds()
                },
                "external_api": {
                    "status": "available",
                    "response_time_ms": 0 // TODO: measure actual response time
                },
                "timestamp": timestamp,
                "trace_id": trace_id
            })))
        }
        Ok(response) => {
            (StatusCode::SERVICE_UNAVAILABLE, Json(json!({
                "status": "degraded",
                "gateway": {
                    "status": "healthy",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "external_api": {
                    "status": "error",
                    "http_status": response.status().as_u16()
                },
                "error": format!("External API returned status: {}", response.status()),
                "timestamp": timestamp,
                "trace_id": trace_id
            })))
        }
        Err(e) => {
            (StatusCode::SERVICE_UNAVAILABLE, Json(json!({
                "status": "unhealthy",
                "gateway": {
                    "status": "healthy",
                    "version": env!("CARGO_PKG_VERSION")
                },
                "external_api": {
                    "status": "unavailable",
                    "error": e.to_string()
                },
                "error": format!("Failed to connect to external API: {}", e),
                "timestamp": timestamp,
                "trace_id": trace_id
            })))
        }
    }
}

/// Endpoint para métricas básicas del sistema
pub async fn system_metrics(State(state): State<AppState>) -> impl IntoResponse {
    let trace_id = Uuid::new_v4().to_string();
    let timestamp = Utc::now();

    // Obtener métricas del sistema operativo
    let system_metrics = get_system_metrics();

    // Intentar obtener métricas de la API externa
    match state.client
        .get(&format!("{}/api/v1/metrics/system", state.api_base))
        .header("Authorization", format!("Bearer {}", state.api_key))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(response) if response.status().is_success() => {
            match response.json::<serde_json::Value>().await {
                Ok(external_metrics) => {
                    Json(json!({
                        "status": "success",
                        "gateway_metrics": system_metrics,
                        "external_api_metrics": external_metrics,
                        "timestamp": timestamp,
                        "trace_id": trace_id
                    }))
                }
                Err(e) => {
                    Json(json!({
                        "status": "partial",
                        "gateway_metrics": system_metrics,
                        "external_api_metrics": null,
                        "error": format!("Failed to parse external metrics: {}", e),
                        "timestamp": timestamp,
                        "trace_id": trace_id
                    }))
                }
            }
        }
        Ok(response) => {
            Json(json!({
                "status": "partial",
                "gateway_metrics": system_metrics,
                "external_api_metrics": null,
                "error": format!("External API returned status: {}", response.status()),
                "timestamp": timestamp,
                "trace_id": trace_id
            }))
        }
        Err(e) => {
            Json(json!({
                "status": "partial",
                "gateway_metrics": system_metrics,
                "external_api_metrics": null,
                "error": format!("Failed to connect to external API: {}", e),
                "timestamp": timestamp,
                "trace_id": trace_id
            }))
        }
    }
}

/// Obtener métricas básicas del sistema
fn get_system_metrics() -> serde_json::Value {
    json!({
        "memory": {
            "used_mb": get_memory_usage_mb(),
            "available_mb": get_available_memory_mb()
        },
        "cpu": {
            "usage_percent": get_cpu_usage_percent()
        },
        "uptime_secs": get_uptime_seconds(),
        "active_connections": get_active_connections_count()
    })
}

// Implementaciones básicas - en producción usarías librerías como `sysinfo`
fn get_uptime_seconds() -> u64 {
    static START_TIME: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    let start = START_TIME.get_or_init(|| std::time::Instant::now());
    start.elapsed().as_secs()
}

fn get_memory_usage_mb() -> u64 {
    // Placeholder - implementar con sysinfo en producción
    0
}

fn get_available_memory_mb() -> u64 {
    // Placeholder - implementar con sysinfo en producción
    0
}

fn get_cpu_usage_percent() -> f64 {
    // Placeholder - implementar con sysinfo en producción
    0.0
}

fn get_active_connections_count() -> u32 {
    // Placeholder - implementar contador real
    0
}