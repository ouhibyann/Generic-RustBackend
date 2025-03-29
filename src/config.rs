// src/config.rs
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct KrakenConfig {
    pub trading_pair: String,
    pub endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct HuobiConfig {
    pub trading_pair: String,
    pub endpoint: String,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub kraken: KrakenConfig,
    pub huobi: HuobiConfig,
}

/// Loads configuration from a file (e.g., config.toml) and environment variables.
pub fn load_config() -> Result<AppConfig, ConfigError> {
    let config = Config::builder()
        .add_source(File::with_name("config").required(false))
        .add_source(Environment::with_prefix("APP").separator("_"))
        .build()?;
    config.try_deserialize()
}
