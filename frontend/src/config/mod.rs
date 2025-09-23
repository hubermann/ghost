/// Centralized configuration for the Ghost frontend application
pub struct AppConfig;

impl AppConfig {
    /// Base URL for the main inBestia API
    pub const API_BASE_URL: &'static str = "http://127.0.0.1:8080";

    /// Base URL for timeframes service (same as main API)
    pub const TIMEFRAMES_API_URL: &'static str = "http://127.0.0.1:8080";

    /// API key for authentication
    pub const API_KEY: &'static str = "inbestia2025key";

    /// Health check endpoint
    pub fn health_url() -> String {
        format!("{}/health", Self::API_BASE_URL)
    }

    /// System info endpoint
    pub fn info_url() -> String {
        format!("{}/api/v1/info", Self::API_BASE_URL)
    }

    /// System metrics endpoint
    pub fn metrics_url() -> String {
        format!("{}/api/v1/metrics/system", Self::API_BASE_URL)
    }

    /// Timeframes configuration endpoint
    pub fn timeframes_config_url() -> String {
        format!("{}/api/v1/timeframes/config", Self::TIMEFRAMES_API_URL)
    }

    /// Analysis endpoint
    pub fn analyze_url() -> String {
        format!("{}/api/v1/analyze", Self::API_BASE_URL)
    }
}