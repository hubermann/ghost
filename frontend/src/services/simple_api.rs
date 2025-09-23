use gloo_net::http::Request;
use crate::domain::simple_types::{SimpleSystemMetrics, ProviderStatus, SimpleHealthStatus, ApiInfo};
use crate::config::AppConfig;

/// Obtiene métricas del sistema DIRECTAMENTE de InBestia API
/// Respuesta coincide exactamente con el formato de la API real
pub async fn get_system_metrics() -> Result<SimpleSystemMetrics, String> {
    Request::get(&AppConfig::metrics_url())
        .header("Authorization", &AppConfig::API_KEY)
        .send().await.map_err(|e| format!("Network error: {}", e))?
        .json::<SimpleSystemMetrics>().await.map_err(|e| format!("JSON parse error: {}", e))
}

/// Obtiene estado de proveedores DIRECTAMENTE de InBestia API
/// Respuesta es un array directo de ProviderStatus
pub async fn get_providers_status() -> Result<Vec<ProviderStatus>, String> {
    Request::get(&format!("{}/api/v1/providers/status", AppConfig::API_BASE_URL))
        .header("Authorization", &AppConfig::API_KEY)
        .send().await.map_err(|e| format!("Network error: {}", e))?
        .json::<Vec<ProviderStatus>>().await.map_err(|e| format!("JSON parse error: {}", e))
}

/// Obtiene estado de salud DIRECTAMENTE de InBestia API
/// Convierte respuesta de texto plano a estructura simple
pub async fn get_health_status() -> Result<SimpleHealthStatus, String> {
    let response = Request::get(&AppConfig::health_url())
        .send().await.map_err(|e| format!("Network error: {}", e))?;

    let text = response.text().await.map_err(|e| format!("Text parse error: {}", e))?;

    Ok(SimpleHealthStatus {
        status: if text.contains("funcionando correctamente") {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
        message: text,
    })
}

/// Obtiene información de la API DIRECTAMENTE de InBestia API
/// Ya funciona correctamente, lo reutilizamos
pub async fn get_api_info() -> Result<ApiInfo, String> {
    Request::get(&AppConfig::info_url())
        .send().await.map_err(|e| format!("Network error: {}", e))?
        .json::<ApiInfo>().await.map_err(|e| format!("JSON parse error: {}", e))
}