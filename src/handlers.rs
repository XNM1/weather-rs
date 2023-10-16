mod services;
mod views;

use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use narrate::{colored::*, Result};

use self::services::WeatherApi;
use self::services::{
    openweather_service::OpenWeatherApiService, weatherapi_service::WeatherApiService,
};
use crate::providers::{Provider, ProviderError};

pub fn provider_list_handler(selected_provider: &Provider) {
    for provider in Provider::get_all_variants() {
        let provider_str = match provider {
            Provider::OpenWeather => format!(" {} (implemented)", provider).green().to_string(),
            Provider::WeatherApi => format!(" {} (implemented)", provider).green().to_string(),
            Provider::AccuWeather => format!(" {} (not implemented)", provider).red().to_string(),
            Provider::AerisWeather => format!(" {} (not implemented)", provider).red().to_string(),
        };
        println!("{}", provider_str);
    }
}

pub async fn get_weather_info(
    address: &str,
    date: &Option<String>,
    json: &bool,
    provider: &Option<Provider>,
) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner} Fetching...")?);
    pb.enable_steady_tick(Duration::from_millis(100));

    let client = reqwest::Client::new();
    let weather_api: Result<Box<dyn WeatherApi>> = match provider {
        Some(Provider::OpenWeather) => Ok(Box::new(OpenWeatherApiService::new(
            client,
            "https://api.openweathermap.org/data/2.5/weather".to_string(),
            "example_api_key".to_string(),
        )?)),
        Some(Provider::WeatherApi) => Ok(Box::new(WeatherApiService::new(
            client,
            "https://api.weatherapi.com/v1".to_string(),
            "example_api_key".to_string(),
        )?)),
        Some(Provider::AccuWeather) => Err(ProviderError::ProviderNotImplemented.into()),
        Some(Provider::AerisWeather) => Err(ProviderError::ProviderNotImplemented.into()),
        None => Err(ProviderError::ProviderNotImplemented.into()),
    };
    let weather_data = weather_api?.get_weather_data(address, date).await?;

    pb.finish_and_clear();

    if *json {
        views::json_terminal_view(weather_data)?;
    } else {
        views::table_terminal_view(weather_data);
    }

    Ok(())
}

pub fn configure_provider(provider: &Provider, url: &str, api_key: &str) {}
