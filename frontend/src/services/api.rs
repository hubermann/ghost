use gloo_net::http::Request;
use crate::domain::types::{ApiInfo, ApiHealth, SystemMetricsResponse};

/// Obtiene información de la API
pub async fn fetch_info() -> Result<ApiInfo, String> {
    Request::get("http://127.0.0.1:8085/api/v1/info")
        .send().await.map_err(|e| e.to_string())?
        .json::<ApiInfo>().await.map_err(|e| e.to_string())
}

/// Verifica el estado de salud de la API de inBestia
pub async fn check_api_health() -> Result<ApiHealth, String> {
    Request::get("http://127.0.0.1:8085/api/health")
        .send().await.map_err(|e| e.to_string())?
        .json::<ApiHealth>().await.map_err(|e| e.to_string())
}

/// Obtiene las métricas del sistema de inBestia
pub async fn fetch_system_metrics() -> Result<SystemMetricsResponse, String> {
    Request::get("http://127.0.0.1:8085/api/metrics/system")
        .send().await.map_err(|e| e.to_string())?
        .json::<SystemMetricsResponse>().await.map_err(|e| e.to_string())
}
