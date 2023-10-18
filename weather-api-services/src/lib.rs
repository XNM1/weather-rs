/// Module that contains structs that represent data from different providers
pub mod models;
/// Module that contains structs and methods for working with the OpenWeather API
pub mod openweather_service;
/// Module that contains structs and methods for working with the Weather API
pub mod weatherapi_service;

use anyhow::Result;
use async_trait::async_trait;
use thiserror::Error;

use models::*;

/// Represents an error that occurs when there is an issue with parsing date and time data.
#[derive(Error, Debug)]
pub enum DateTimeError {
    #[error("Invalid datetime format. Please use a recognized datetime format (e.g., 'MM/DD/YYYY' or 'YYYY-MM-DD hh:mm' or 'YYYY-MM-DD')")]
    Parse,
}

/// Represents errors related to the some weather api service and its operations.
#[derive(Error, Debug)]
pub enum WeatherApiError {
    /// Represents an error when creating an API client, which can occur due to invalid 'url' or 'api_key'.
    #[error("Failed to create an API client; can be invalid 'url' or 'api_key'")]
    Creation,

    /// Represents an error when sending a request to the Weather API.
    #[error("Failed to send a request to the Weather API; can be invalid 'url' or 'api_key'")]
    Request(#[from] reqwest::Error),

    /// Represents an error with the provider server's response when an error occurs on the provider side, including a custom error message.
    #[error("Provider server response error '{0}'")]
    Server(String),

    /// Represents an error when processing the body text from the response.
    #[error("Can't process the body text from the response")]
    BodyText,

    /// Represents an error when the provider don't support a specific feature.
    #[error("Service provider doesn't support a feature '{0}'")]
    Feature(String),
}

/// The `WeatherApi` trait defines the contract for retrieving weather data for a given address and optional date.
#[async_trait]
pub trait WeatherApi {
    /// Asynchronously retrieves weather data for a specific address and date (if provided).
    ///
    /// # Arguments
    ///
    /// * `address` - A string representing the address for which weather data is requested.
    /// * `date` - An optional string containing the date for historical weather data. Pass `None` for current weather.
    ///
    /// # Returns
    ///
    /// A `Result` containing the retrieved weather data or an error if the request fails.
    async fn get_weather_data(&self, address: &str, date: &Option<String>) -> Result<WeatherData>;
}
