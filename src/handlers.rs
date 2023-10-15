mod services;
mod views;

use std::thread;
use std::time::Duration;

use color_eyre::{owo_colors::OwoColorize, Result};
use indicatif::{ProgressBar, ProgressStyle};

use self::services::openweather_service::OpenWeatherApi;
use self::services::WeatherApi;
use crate::cli_parser::{Provider, ProviderError};

pub fn provider_list_handler(selected_provider: &Provider) {
    for provider in Provider::get_all_variants() {
        let mut provider_str = match provider {
            Provider::OpenWeather => format!("*{} (selected)", provider).green().to_string(),
            Provider::WeatherApi => format!(" {} (not implemented)", provider).red().to_string(),
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
    provider: &Provider,
) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner} Fetching...")?);
    pb.enable_steady_tick(Duration::from_millis(100));

    let client = reqwest::Client::new();
    let weather_api: Result<Box<dyn WeatherApi>> = match provider {
        Provider::OpenWeather => Ok(Box::new(OpenWeatherApi::new(
            client,
            "https://api.openweathermap.org/data/2.5/weather".to_string(),
            "example_api_key".to_string(),
        ))),
        Provider::WeatherApi => Err(ProviderError::ProviderNotImplemented.into()),
        Provider::AccuWeather => Err(ProviderError::ProviderNotImplemented.into()),
        Provider::AerisWeather => Err(ProviderError::ProviderNotImplemented.into()),
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
