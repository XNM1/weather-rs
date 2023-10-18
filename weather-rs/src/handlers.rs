use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use narrate::anyhow::Result;
use narrate::colored::Colorize;

use crate::config::{ConfigError, MainConfig, ProviderConfig};
use crate::providers::{Provider, ProviderError};
use crate::views;
use weather_api_services::WeatherApi;
use weather_api_services::{
    openweather_service::OpenWeatherApiService, weatherapi_service::WeatherApiService,
};

pub fn provider_list_handler(
    selected_provider: &Provider,
    configured_providers: Vec<&Provider>,
    not_implemented_providers: Vec<&Provider>,
) {
    println!("Current status of providers: ");

    for provider in Provider::get_all_variants() {
        let provider_str = if not_implemented_providers.contains(&&provider) {
            format!("{} (not implemented)", provider).red()
        } else if configured_providers.contains(&&provider) {
            format!("{} (configured)", provider).green()
        } else {
            format!("{} (not configured)", provider).yellow()
        };

        if &provider == selected_provider {
            println!("*{} (selected)", provider_str);
        } else {
            println!(" {}", provider_str);
        }
    }

    println!("\nCurrently supported providers is\n\tOpen Weather ({}; example url: '{}'),\n\tWeather API ({}; example url: '{}')", "v2".blue(), "https://api.openweathermap.org/data/2.5/weather".green(), "v1".blue(), "https://api.weatherapi.com/v1".green());
}

pub async fn get_weather_info(
    address: &str,
    date: &Option<String>,
    json: bool,
    provider: &Provider,
    config: MainConfig,
) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner().template("{spinner} Fetching...")?);
    pb.enable_steady_tick(Duration::from_millis(100));

    let client = reqwest::Client::new();
    let weather_api: Result<Box<dyn WeatherApi>> = match provider {
        Provider::OpenWeather => {
            let open_weather_config = config
                .open_weather
                .ok_or(ConfigError::ProviderConfig(provider.to_string()))?;

            Ok(Box::new(OpenWeatherApiService::new(
                client,
                open_weather_config.url,
                open_weather_config.api_key,
            )?))
        }
        Provider::WeatherApi => {
            let weather_api_config = config
                .weather_api
                .ok_or(ConfigError::ProviderConfig(provider.to_string()))?;

            Ok(Box::new(WeatherApiService::new(
                client,
                weather_api_config.url,
                weather_api_config.api_key,
            )?))
        }
        Provider::AccuWeather => Err(ProviderError::ProviderNotImplemented.into()),
        Provider::AerisWeather => Err(ProviderError::ProviderNotImplemented.into()),
    };
    let weather_data = weather_api?.get_weather_data(address, date).await?;

    pb.finish_and_clear();

    if json {
        views::json_terminal_view(weather_data)?;
    } else {
        views::table_terminal_view(weather_data);
    }

    Ok(())
}

pub fn configure_provider(cfg: &mut MainConfig, provider: &Provider, url: String, api_key: String) {
    let provider_config: Option<ProviderConfig> = Some(ProviderConfig { url, api_key });

    match provider {
        Provider::OpenWeather => cfg.open_weather = provider_config,
        Provider::WeatherApi => cfg.weather_api = provider_config,
        Provider::AccuWeather => cfg.accu_weather = provider_config,
        Provider::AerisWeather => cfg.aeris_weather = provider_config,
    }
}

pub fn select_provider(cfg: &mut MainConfig, provider: Provider) {
    cfg.selected_provider = provider;
}
