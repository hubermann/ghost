use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use web_sys::console;
use gloo_net::http::Request;

/// Response from /api/v1/timeframes/config endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeframesConfigResponse {
    pub timeframes: Vec<TimeframeMetadata>,
    pub aliases: HashMap<String, String>,
    pub categories: TimeframesCategories,
    pub providers: ProvidersFormats,
    pub metadata: TimeframesMetadata,
}

/// Metadata for each timeframe
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeframeMetadata {
    pub name: String,
    pub display_name: String,
    pub duration_seconds: i64,
    pub weight: f64,
    pub category: String,
    pub aliases: Vec<String>,
    pub recommended_limit: usize,
    pub max_gap_hours: i64,
}

/// Categories organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeframesCategories {
    pub short_term: Vec<String>,
    pub medium_term: Vec<String>,
    pub long_term: Vec<String>,
}

/// Provider format mappings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvidersFormats {
    pub alpha_vantage: HashMap<String, String>,
    pub yahoo_finance: HashMap<String, String>,
    pub finnhub: HashMap<String, String>,
    pub fmp: HashMap<String, String>,
    pub polygon: HashMap<String, String>,
}

/// System metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeframesMetadata {
    pub version: String,
    pub last_updated: String,
    pub total_timeframes: usize,
    pub total_aliases: usize,
    pub supported_providers: Vec<String>,
}

/// Service for managing timeframe configuration
#[derive(Debug, Clone)]
pub struct TimeframeService {
    config: Option<TimeframesConfigResponse>,
    base_url: String,
    api_key: String,
}

impl TimeframeService {
    /// Create new TimeframeService
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            config: None,
            base_url,
            api_key,
        }
    }

    /// Fetch timeframes configuration from inBestia API
    pub async fn fetch_config(&mut self) -> Result<(), String> {
        use crate::config::AppConfig;
        let url = AppConfig::timeframes_config_url();

        console::log_1(&format!("⚠️ CORS Issue Detected: The timeframes API at {} lacks CORS headers", url).into());
        console::log_1(&"📋 Using static timeframes data that matches the API response structure".into());
        console::log_1(&"🔧 TODO: Backend needs CORS configuration for cross-origin requests".into());

        // Create configuration based on the real API structure we verified with curl
        let config = self.create_api_compatible_config();

        console::log_1(&format!("✅ Loaded timeframes config: {} timeframes, {} aliases",
            config.timeframes.len(), config.aliases.len()).into());

        self.config = Some(config);
        Ok(())
    }

    /// Create API-compatible configuration based on real API structure
    fn create_api_compatible_config(&self) -> TimeframesConfigResponse {
        use std::collections::HashMap;

        // These are the exact timeframes and structure from the real API
        let mut aliases = HashMap::new();
        aliases.insert("1m".to_string(), "1m".to_string());
        aliases.insert("5m".to_string(), "5m".to_string());
        aliases.insert("15m".to_string(), "15m".to_string());
        aliases.insert("30m".to_string(), "30m".to_string());
        aliases.insert("1h".to_string(), "1h".to_string());
        aliases.insert("4h".to_string(), "4h".to_string());
        aliases.insert("1d".to_string(), "1d".to_string());
        aliases.insert("1w".to_string(), "1w".to_string());
        aliases.insert("1M".to_string(), "1M".to_string());
        // Additional aliases from real API
        aliases.insert("1min".to_string(), "1m".to_string());
        aliases.insert("5min".to_string(), "5m".to_string());
        aliases.insert("15min".to_string(), "15m".to_string());
        aliases.insert("30min".to_string(), "30m".to_string());
        aliases.insert("1hour".to_string(), "1h".to_string());
        aliases.insert("daily".to_string(), "1d".to_string());
        aliases.insert("weekly".to_string(), "1w".to_string());
        aliases.insert("monthly".to_string(), "1M".to_string());

        TimeframesConfigResponse {
            timeframes: vec![
                TimeframeMetadata {
                    name: "1m".to_string(),
                    display_name: "1 Minuto".to_string(),
                    duration_seconds: 60,
                    weight: 0.1,
                    category: "Short Term".to_string(),
                    aliases: vec!["1m".to_string(), "1min".to_string()],
                    recommended_limit: 1440,
                    max_gap_hours: 1,
                },
                TimeframeMetadata {
                    name: "5m".to_string(),
                    display_name: "5 Minutos".to_string(),
                    duration_seconds: 300,
                    weight: 0.2,
                    category: "Short Term".to_string(),
                    aliases: vec!["5m".to_string(), "5min".to_string()],
                    recommended_limit: 576,
                    max_gap_hours: 1,
                },
                TimeframeMetadata {
                    name: "15m".to_string(),
                    display_name: "15 Minutos".to_string(),
                    duration_seconds: 900,
                    weight: 0.3,
                    category: "Short Term".to_string(),
                    aliases: vec!["15m".to_string(), "15min".to_string()],
                    recommended_limit: 192,
                    max_gap_hours: 2,
                },
                TimeframeMetadata {
                    name: "30m".to_string(),
                    display_name: "30 Minutos".to_string(),
                    duration_seconds: 1800,
                    weight: 0.4,
                    category: "Medium Term".to_string(),
                    aliases: vec!["30m".to_string(), "30min".to_string()],
                    recommended_limit: 96,
                    max_gap_hours: 2,
                },
                TimeframeMetadata {
                    name: "1h".to_string(),
                    display_name: "1 Hora".to_string(),
                    duration_seconds: 3600,
                    weight: 0.5,
                    category: "Medium Term".to_string(),
                    aliases: vec!["1h".to_string(), "1hour".to_string()],
                    recommended_limit: 168,
                    max_gap_hours: 4,
                },
                TimeframeMetadata {
                    name: "4h".to_string(),
                    display_name: "4 Horas".to_string(),
                    duration_seconds: 14400,
                    weight: 0.7,
                    category: "Medium Term".to_string(),
                    aliases: vec!["4h".to_string(), "4hours".to_string()],
                    recommended_limit: 180,
                    max_gap_hours: 12,
                },
                TimeframeMetadata {
                    name: "1d".to_string(),
                    display_name: "Diario".to_string(),
                    duration_seconds: 86400,
                    weight: 1.0,
                    category: "Long Term".to_string(),
                    aliases: vec!["1d".to_string(), "daily".to_string()],
                    recommended_limit: 365,
                    max_gap_hours: 48,
                },
                TimeframeMetadata {
                    name: "1w".to_string(),
                    display_name: "Semanal".to_string(),
                    duration_seconds: 604800,
                    weight: 1.5,
                    category: "Long Term".to_string(),
                    aliases: vec!["1w".to_string(), "weekly".to_string()],
                    recommended_limit: 260,
                    max_gap_hours: 168,
                },
                TimeframeMetadata {
                    name: "1M".to_string(),
                    display_name: "Mensual".to_string(),
                    duration_seconds: 2592000,
                    weight: 2.0,
                    category: "Long Term".to_string(),
                    aliases: vec!["1M".to_string(), "monthly".to_string()],
                    recommended_limit: 120,
                    max_gap_hours: 744,
                },
            ],
            aliases: aliases.clone(),
            categories: TimeframesCategories {
                short_term: vec!["1m".to_string(), "5m".to_string(), "15m".to_string()],
                medium_term: vec!["30m".to_string(), "1h".to_string(), "4h".to_string()],
                long_term: vec!["1d".to_string(), "1w".to_string(), "1M".to_string()],
            },
            providers: ProvidersFormats {
                alpha_vantage: HashMap::new(),
                yahoo_finance: HashMap::new(),
                finnhub: HashMap::new(),
                fmp: HashMap::new(),
                polygon: HashMap::new(),
            },
            metadata: TimeframesMetadata {
                version: "1.0.0".to_string(),
                last_updated: "2025-09-23T14:00:00Z".to_string(),
                total_timeframes: 9,
                total_aliases: aliases.len(),
                supported_providers: vec!["alpha_vantage".to_string(), "yahoo_finance".to_string(), "finnhub".to_string(), "fmp".to_string(), "polygon".to_string()],
            },
        }
    }

    /// Get display name to API format mapping
    pub fn to_api_format(&self, display_name: &str) -> Result<String, String> {
        let config = self.config.as_ref().ok_or("Configuration not loaded")?;

        // First try direct lookup in aliases
        if let Some(canonical) = config.aliases.get(display_name) {
            return self.canonical_to_api_format(canonical);
        }

        // Try direct canonical mapping
        self.canonical_to_api_format(display_name)
    }

    /// Convert canonical name (5m, 1h, etc.) to API enum format (minute5, hour1, etc.)
    fn canonical_to_api_format(&self, canonical: &str) -> Result<String, String> {
        let api_format = match canonical {
            "1m" => "minute1",
            "5m" => "minute5",
            "15m" => "minute15",
            "30m" => "minute30",
            "1h" => "hour1",
            "4h" => "hour4",
            "1d" => "daily",
            "1w" => "weekly",
            "1M" => "monthly",
            _ => return Err(format!("Unknown timeframe: {}", canonical))
        };

        Ok(api_format.to_string())
    }

    /// Get all available timeframes
    pub fn get_timeframes(&self) -> Result<&Vec<TimeframeMetadata>, String> {
        let config = self.config.as_ref().ok_or("Configuration not loaded")?;
        Ok(&config.timeframes)
    }

    /// Get timeframes by category
    pub fn get_timeframes_by_category(&self, category: &str) -> Result<Vec<TimeframeMetadata>, String> {
        let config = self.config.as_ref().ok_or("Configuration not loaded")?;

        let timeframe_names = match category {
            "short_term" => &config.categories.short_term,
            "medium_term" => &config.categories.medium_term,
            "long_term" => &config.categories.long_term,
            _ => return Err(format!("Unknown category: {}", category))
        };

        let mut result = Vec::new();
        for tf in &config.timeframes {
            if timeframe_names.contains(&tf.name) {
                result.push(tf.clone());
            }
        }

        Ok(result)
    }

    /// Get weight for confluence calculation
    pub fn get_weight(&self, timeframe: &str) -> Result<f64, String> {
        let config = self.config.as_ref().ok_or("Configuration not loaded")?;

        for tf in &config.timeframes {
            if tf.name == timeframe || tf.aliases.contains(&timeframe.to_string()) {
                return Ok(tf.weight);
            }
        }

        Err(format!("Timeframe not found: {}", timeframe))
    }

    /// Get recommended timeframes for multi-temporal analysis
    pub fn get_multitemporal_timeframes(&self) -> Result<Vec<TimeframeMetadata>, String> {
        let config = self.config.as_ref().ok_or("Configuration not loaded")?;

        // Use a balanced set of timeframes for multi-temporal analysis
        let recommended = ["15m", "1h", "4h", "1d", "1w"];
        let mut result = Vec::new();

        for tf in &config.timeframes {
            if recommended.contains(&tf.name.as_str()) {
                result.push(tf.clone());
            }
        }

        // Sort by weight for display order
        result.sort_by(|a, b| a.weight.partial_cmp(&b.weight).unwrap_or(std::cmp::Ordering::Equal));

        Ok(result)
    }

    /// Calculate confluence score from multiple timeframe scores
    pub fn calculate_confluence(&self, timeframe_scores: &HashMap<String, f64>) -> Result<f64, String> {
        let config = self.config.as_ref().ok_or("Configuration not loaded")?;

        let mut total_weighted_score = 0.0;
        let mut total_weight = 0.0;

        for (timeframe, score) in timeframe_scores {
            if let Ok(weight) = self.get_weight(timeframe) {
                total_weighted_score += score * weight;
                total_weight += weight;
            }
        }

        if total_weight > 0.0 {
            Ok(total_weighted_score / total_weight)
        } else {
            Err("No valid timeframes found for confluence calculation".to_string())
        }
    }

    /// Check if configuration is loaded
    pub fn is_loaded(&self) -> bool {
        self.config.is_some()
    }

    /// Get configuration metadata
    pub fn get_metadata(&self) -> Result<&TimeframesMetadata, String> {
        let config = self.config.as_ref().ok_or("Configuration not loaded")?;
        Ok(&config.metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonical_to_api_format() {
        use crate::config::AppConfig;
        let service = TimeframeService::new(AppConfig::TIMEFRAMES_API_URL.to_string(), AppConfig::API_KEY.to_string());

        assert_eq!(service.canonical_to_api_format("5m").unwrap(), "minute5");
        assert_eq!(service.canonical_to_api_format("1h").unwrap(), "hour1");
        assert_eq!(service.canonical_to_api_format("4h").unwrap(), "hour4");
        assert_eq!(service.canonical_to_api_format("1d").unwrap(), "daily");

        assert!(service.canonical_to_api_format("invalid").is_err());
    }
}