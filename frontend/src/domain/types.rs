use serde::Deserialize;

/// Información de la API
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ApiInfo {
    pub name: String,
    pub version: String,
    pub description: String,
    pub data_mode: String,
    pub authentication: Authentication,
    pub endpoints: Vec<Endpoint>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Authentication {
    #[serde(rename = "type")]
    pub auth_type: String,
    pub methods: Vec<AuthMethod>,
    pub note: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AuthMethod {
    pub name: String,
    pub location: String,
    pub format: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Endpoint {
    pub description: String,
    pub method: String,
    pub path: String,
    pub requires_auth: bool,
}

/// Estado de salud de la API
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ApiHealth {
    pub status: String,
    pub inbestia_api: String,
    pub gateway: String,
    pub timestamp: String,
    pub trace_id: Option<String>,
    pub error: Option<String>,
}

/// Métricas del sistema de inBestia
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub database_connections: u32,
    pub cache_hit_ratio: f64,
    pub active_requests: u32,
}

/// Respuesta de métricas del sistema
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SystemMetricsResponse {
    pub status: String,
    pub data: SystemMetrics,
    pub timestamp: String,
    pub trace_id: Option<String>,
    pub error: Option<String>,
}
