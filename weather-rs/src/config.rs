use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
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
    /// * `1` - A string representing the relative path to the configuration file in the default system configuration directory.
    #[error("Failed to read configuration for '{0}' service; check url and api key for the API Service in '{1}' file in your config directory")]
    ProviderConfig(String, String),
}

/// Represents the main configuration for the weather application.
#[derive(Serialize, Deserialize, SmartDefault, Debug, PartialEq)]
pub struct MainConfig {
    /// The selected weather data provider.
    pub selected_provider: Provider,
    /// Configuration for the OpenWeather service.
    #[default(ProviderConfig { url: "https://api.openweathermap.org/data/2.5/weather".to_owned(), api_key: None })]
    pub open_weather: ProviderConfig,
    /// Configuration for the WeatherAPI service.
    #[default(ProviderConfig { url: "https://api.weatherapi.com/v1".to_owned(), api_key: None })]
    pub weather_api: ProviderConfig,
    /// Configuration for the AccuWeather service.
    #[default(ProviderConfig { url: "http://dataservice.accuweather.com/currentconditions/v1".to_owned(), api_key: None })]
    pub accu_weather: ProviderConfig,
    /// Configuration for the AerisWeather service.
    #[default(ProviderConfig { url: "https://api.aerisapi.com/conditions".to_owned(), api_key: None })]
    pub aeris_weather: ProviderConfig,
}

/// Represents the configuration for a weather data provider.
#[derive(Serialize, Deserialize, Debug, SmartDefault, PartialEq)]
pub struct ProviderConfig {
    /// The URL for the service.
    pub url: String,
    /// The API key required for authentication with the service.
    pub api_key: Option<String>,
}
