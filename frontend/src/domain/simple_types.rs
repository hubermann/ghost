use serde::Deserialize;

/// Métricas del sistema - Coincide EXACTAMENTE con la respuesta de InBestia API
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SimpleSystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub database_connections: u32,
    pub cache_hit_ratio: f64,
    pub active_requests: u32,
}

/// Estado de un proveedor de datos - Coincide EXACTAMENTE con la respuesta de InBestia API
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ProviderStatus {
    pub name: String,
    #[serde(rename = "type_code")]
    pub type_code: String,
    pub available: bool,
    pub active: bool,
    pub rate_limit_remaining: Option<u32>,
    pub rate_limit_reset: Option<String>,
    pub response_time_ms: u64,
}

/// Estado de salud simplificado
#[derive(Debug, Clone, PartialEq)]
pub struct SimpleHealthStatus {
    pub status: String,
    pub message: String,
}

/// Información de la API - Usa la estructura existente que ya funciona
pub use crate::domain::types::ApiInfo;