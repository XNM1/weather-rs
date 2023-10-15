pub mod models;
pub mod openweather_service;

use async_trait::async_trait;
use color_eyre::Result;
use thiserror::Error;

use models::*;

#[derive(Error, Debug, PartialEq)]
pub enum DateTimeError {
    #[error("Invalid datetime format. Please use a recognized datetime format (e.g., 'MM/DD/YYYY' or 'YYYY-MM-DD hh:mm' or 'YYYY-MM-DD')")]
    ParseError,
}

#[async_trait]
pub trait WeatherApi {
    async fn get_weather_data(&self, address: &str, date: &Option<String>) -> Result<WeatherData>;
}
