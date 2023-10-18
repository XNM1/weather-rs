use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::providers::Provider;

/// Represents errors related to configuration.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// An error indicating a failure to read configuration for a specific service.
    ///
    /// # Parameters
    ///
    /// * `0` - A string representing the name of the service for which configuration reading failed.
    #[error("Failed to read configuration for '{0}' service; check url and api key for the API Service in 'weather-rs/config.toml' file in your config directory")]
    ProviderConfig(String),
}

/// Represents the main configuration for the weather application.
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MainConfig {
    /// The selected weather data provider.
    pub selected_provider: Provider,
    /// Configuration for the OpenWeather service.
    pub open_weather: Option<ProviderConfig>,
    /// Configuration for the WeatherAPI service.
    pub weather_api: Option<ProviderConfig>,
    /// Configuration for the AccuWeather service.
    pub accu_weather: Option<ProviderConfig>,
    /// Configuration for the AerisWeather service.
    pub aeris_weather: Option<ProviderConfig>,
}

/// Represents the configuration for a weather data provider.
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProviderConfig {
    /// The URL for the service.
    pub url: String,
    /// The API key required for authentication with the service.
    pub api_key: String,
}
