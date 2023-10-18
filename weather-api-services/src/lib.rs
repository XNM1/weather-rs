pub mod models;
pub mod openweather_service;
pub mod weatherapi_service;

use anyhow::Result;
use async_trait::async_trait;
use thiserror::Error;

use models::*;

#[derive(Error, Debug)]
pub enum DateTimeError {
    #[error("Invalid datetime format. Please use a recognized datetime format (e.g., 'MM/DD/YYYY' or 'YYYY-MM-DD hh:mm' or 'YYYY-MM-DD')")]
    Parse,
}

#[derive(Error, Debug)]
pub enum WeatherApiError {
    #[error("Failed to create an API client; can be invalid 'url' or 'api_key'")]
    Creation,

    #[error("Failed to send a request to the Weather API; can be invalid 'url' or 'api_key'")]
    Request(#[from] reqwest::Error),

    #[error("Provider server response error '{0}'")]
    Server(String),

    #[error("Can't process the body text from the response")]
    BodyText,
}

#[async_trait]
pub trait WeatherApi {
    async fn get_weather_data(&self, address: &str, date: &Option<String>) -> Result<WeatherData>;
}
