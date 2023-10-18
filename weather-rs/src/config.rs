use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::providers::Provider;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read configuration for '{0}' service; check url and api key for the API Service in 'weather-rs/config.toml' file in your config directory")]
    ProviderConfig(String),
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MainConfig {
    pub selected_provider: Provider,
    pub open_weather: Option<ProviderConfig>,
    pub weather_api: Option<ProviderConfig>,
    pub accu_weather: Option<ProviderConfig>,
    pub aeris_weather: Option<ProviderConfig>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProviderConfig {
    pub url: String,
    pub api_key: String,
}
