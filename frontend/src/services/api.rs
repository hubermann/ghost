use gloo_net::http::Request;
use crate::domain::types::{ApiInfo, ApiHealth, SystemMetricsResponse};
use crate::config::AppConfig;

/// Obtiene información de la API
pub async fn fetch_info() -> Result<ApiInfo, String> {
    Request::get(&AppConfig::info_url())
        .send().await.map_err(|e| e.to_string())?
        .json::<ApiInfo>().await.map_err(|e| e.to_string())
}

/// Verifica el estado de salud de la API de inBestia
pub async fn check_api_health() -> Result<ApiHealth, String> {
    let response = Request::get(&AppConfig::health_url())
        .send().await.map_err(|e| e.to_string())?;

    let text = response.text().await.map_err(|e| e.to_string())?;

    // El endpoint /health devuelve texto plano, lo convertimos a estructura
    let status = if text.contains("funcionando correctamente") { "healthy" } else { "unhealthy" };

    Ok(ApiHealth {
        status: status.to_string(),
        external_api: crate::domain::types::ExternalApiStatus {
            response_time_ms: 0, // No disponible desde este endpoint
            status: if status == "healthy" { "available".to_string() } else { "unavailable".to_string() },
        },
        gateway: crate::domain::types::GatewayStatus {
            status: status.to_string(),
            version: "1.0.0".to_string(), // Valor por defecto
            uptime_secs: 0, // No disponible desde este endpoint
        },
        timestamp: chrono::Utc::now().to_rfc3339(),
        trace_id: None,
    })
}

/// Obtiene las métricas del sistema de inBestia
pub async fn fetch_system_metrics() -> Result<SystemMetricsResponse, String> {
    let response = Request::get(&AppConfig::metrics_url())
        .header("Authorization", &AppConfig::API_KEY)
        .send().await.map_err(|e| e.to_string())?;

    // La API devuelve métricas simples, las adaptamos al formato esperado
    let metrics: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

    Ok(SystemMetricsResponse {
        status: "success".to_string(),
        external_api_metrics: crate::domain::types::ExternalApiMetrics {
            cpu_usage: metrics["cpu_usage"].as_f64().unwrap_or(0.0),
            memory_usage: metrics["memory_usage"].as_f64().unwrap_or(0.0),
            database_connections: metrics["database_connections"].as_i64().unwrap_or(0) as u32,
            cache_hit_ratio: metrics["cache_hit_ratio"].as_f64().unwrap_or(0.0),
            active_requests: metrics["active_requests"].as_i64().unwrap_or(0) as u32,
        },
        gateway_metrics: crate::domain::types::GatewayMetrics {
            active_connections: 0, // No disponible desde este endpoint
            cpu: crate::domain::types::CpuMetrics {
                usage_percent: 0.0, // No disponible desde este endpoint
            },
            memory: crate::domain::types::MemoryMetrics {
                used_mb: 0, // No disponible desde este endpoint
                available_mb: 0, // No disponible desde este endpoint
            },
            uptime_secs: 0, // No disponible desde este endpoint
        },
        timestamp: chrono::Utc::now().to_rfc3339(),
        trace_id: None,
    })
}
