use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_base_url: String,
    pub api_key: String,
    pub bind_address: String,
    pub request_timeout_secs: u64,
    pub max_request_size: usize,
    pub cors_allowed_origins: Vec<String>,
    pub log_level: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            api_base_url: env::var("INBESTIA_API_URL")
                .map_err(|_| anyhow::anyhow!("INBESTIA_API_URL environment variable is required"))?,
            api_key: env::var("INBESTIA_API_KEY")
                .map_err(|_| anyhow::anyhow!("INBESTIA_API_KEY environment variable is required"))?,
            bind_address: env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8085".to_string()),
            request_timeout_secs: env::var("REQUEST_TIMEOUT")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .unwrap_or(30),
            max_request_size: env::var("MAX_REQUEST_SIZE")
                .unwrap_or_else(|_| "2097152".to_string()) // 2MB
                .parse()
                .unwrap_or(2 * 1024 * 1024),
            cors_allowed_origins: env::var("CORS_ALLOWED_ORIGINS")
                .unwrap_or_else(|_| "http://127.0.0.1:3001,http://localhost:3001".to_string())
                .split(',')
                .map(|s| s.trim().to_string())
                .collect(),
            log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
        })
    }

    pub fn validate(&self) -> anyhow::Result<()> {
        if self.api_key.is_empty() {
            return Err(anyhow::anyhow!("API key cannot be empty"));
        }

        if !self.api_base_url.starts_with("http") {
            return Err(anyhow::anyhow!("API base URL must start with http:// or https://"));
        }

        Ok(())
    }
}