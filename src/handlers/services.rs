pub mod models;
pub mod openweather_service;
pub mod weatherapi_service;

use async_trait::async_trait;
use narrate::Result;
use thiserror::Error;

use models::*;

#[derive(Error, Debug, PartialEq)]
pub enum DateTimeError {
    #[error("Invalid datetime format. Please use a recognized datetime format (e.g., 'MM/DD/YYYY' or 'YYYY-MM-DD hh:mm' or 'YYYY-MM-DD')")]
    ParseError,
}

#[derive(Error, Debug)]
pub enum WeatherApiError {
    #[error("Failed to create an API client; check url and api key for the API Service in 'weather-rs/config.toml' file")]
    CreationError,

    #[error("Failed to send a request to the Weather API; check url and api key for the API Service in 'weather-rs/config.toml' file")]
    RequestError(#[from] reqwest::Error),
}

#[async_trait]
pub trait WeatherApi {
    async fn get_weather_data(&self, address: &str, date: &Option<String>) -> Result<WeatherData>;
}
