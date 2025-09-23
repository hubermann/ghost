use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalysisRequest {
    pub symbol: String,
    pub timeframe: String,
    pub include_fundamental: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct AnalysisResponse {
    pub symbol: String,
    pub timeframe: String,
    pub score: f64,
    pub technical_score: f64,
    pub fundamental_score: Option<f64>,
    pub trend_score: f64,
    pub momentum_score: f64,
    pub volatility_score: f64,
    pub volume_score: f64,
    pub suggested_operation: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TimeframeConfig {
    pub name: String,
    pub display_name: String,
    pub duration_seconds: u32,
    pub weight: f32,
    pub category: String,
    pub aliases: Vec<String>,
    pub recommended_limit: u32,
    pub max_gap_hours: u32,
}

impl TimeframeConfig {
    /// Convierte el timeframe de la API al formato esperado por el endpoint de análisis
    /// La API de inBestia espera: minute1, minute5, minute15, minute30, hour1, hour4, daily, weekly, monthly
    pub fn to_analysis_format(&self) -> String {
        match self.name.as_str() {
            "1m" => "minute1".to_string(),
            "5m" => "minute5".to_string(),
            "15m" => "minute15".to_string(),
            "30m" => "minute30".to_string(),
            "1h" => "hour1".to_string(),
            "4h" => "hour4".to_string(),
            "1d" => "daily".to_string(),
            "1w" => "weekly".to_string(),
            "1M" => "monthly".to_string(),
            _ => self.name.clone(), // Fallback al nombre original
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TimeframesConfigResponse {
    pub timeframes: Vec<TimeframeConfig>,
    pub aliases: std::collections::HashMap<String, String>,
    pub categories: std::collections::HashMap<String, Vec<String>>,
    pub providers: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    pub metadata: TimeframesMetadata,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct TimeframesMetadata {
    pub version: String,
    pub last_updated: String,
    pub total_timeframes: u32,
    pub total_aliases: u32,
    pub supported_providers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SymbolErrorResponse {
    pub message: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AnalysisState {
    Idle,
    Loading,
    Success(AnalysisResponse),
    Error(String),
    SymbolNotFound(SymbolErrorResponse),
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeframesState {
    Loading,
    Loaded(Vec<TimeframeConfig>),
    Error(String),
}
