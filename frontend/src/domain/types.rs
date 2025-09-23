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
    pub external_api: ExternalApiStatus,
    pub gateway: GatewayStatus,
    pub timestamp: String,
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ExternalApiStatus {
    pub response_time_ms: u64,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GatewayStatus {
    pub status: String,
    pub uptime_secs: u64,
    pub version: String,
}

/// Métricas de la API externa
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ExternalApiMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub database_connections: u32,
    pub cache_hit_ratio: f64,
    pub active_requests: u32,
}

/// Métricas del gateway
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct GatewayMetrics {
    pub active_connections: u32,
    pub cpu: CpuMetrics,
    pub memory: MemoryMetrics,
    pub uptime_secs: u64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct CpuMetrics {
    pub usage_percent: f64,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct MemoryMetrics {
    pub used_mb: u64,
    pub available_mb: u64,
}

/// Respuesta de métricas del sistema
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct SystemMetricsResponse {
    pub status: String,
    pub external_api_metrics: ExternalApiMetrics,
    pub gateway_metrics: GatewayMetrics,
    pub timestamp: String,
    pub trace_id: Option<String>,
}
