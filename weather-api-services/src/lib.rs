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
    /// An error indicating an invalid datetime format.
    ///
    /// This error occurs when there is an issue with parsing date and time data in an unrecognized format,
    /// and the specific invalid datetime string is included as a parameter.
    ///
    /// # Parameters
    ///
    /// * `0` - A string representing the invalid datetime string that caused the error.
    #[error("Invalid datetime - {0}. Please use a recognized datetime format (e.g., 'MM/DD/YYYY' or 'YYYY-MM-DD hh:mm' or 'YYYY-MM-DD')")]
    Parse(String),
}

/// Represents errors related to a weather API service and its operations.
#[derive(Error, Debug)]
pub enum WeatherApiError {
    /// Represents an error when creating an API client, which can occur due to invalid 'url' or 'api_key'.
    #[error("Failed to create an API client; can be invalid 'url' or 'api_key'")]
    Creation,

    /// Represents an error when sending a request to the weather API provider.
    ///
    /// This error occurs when there is a failure in creating an API client,
    /// which can be due to invalid 'url' or 'api_key'
    ///
    /// # Parameters
    ///
    /// * `0` - The `reqwest::Error` indicating the specific request error.
    /// * `1` - A string representing the name of the service provider causing the error.
    #[error(
        "Failed to send a request to the service provider {0}; can be invalid 'url' or 'api_key'"
    )]
    Request(reqwest::Error, String),

    /// Represents an error with the provider server's response when an error occurs on the provider side, including a custom error message.
    ///
    /// This error occurs when the provider server responds with an error message, and
    /// the custom error message is included as a parameter.
    ///
    /// # Parameters
    ///
    /// * `0` - A string representing the custom error message from the provider server.
    #[error("Provider server response error '{0}'")]
    Server(String),

    /// Represents an error when processing the body text from the response.
    ///
    /// # Parameters
    ///
    /// * `0` - The `reqwest::Error` indicating the specific error while processing the body text.
    #[error("Can't process the body text from the response")]
    BodyText(reqwest::Error),

    /// Represents an error when the provider doesn't support a specific feature.
    ///
    /// # Parameters
    ///
    /// * `0` - A string representing the name of the unsupported feature.
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
