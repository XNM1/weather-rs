/// The `cli_parser` module handles the parsing of command-line arguments and options for the weather-rs application.
mod cli_parser;
/// The `config` module defines data structures for handling configuration settings in the weather-rs application.
mod config;
/// The `handlers` module contains functions that handle various commands and operations in the weather-rs application.
mod handlers;
/// The `providers` module defines enum for weather data providers implementations for the weather-rs application.
mod providers;
/// The `views` module contains functions responsible for displaying weather data in different output views,
/// such as table view and JSON view, in the weather-rs application.
mod views;

use clap::Parser;
use config::MainConfig;
use narrate::anyhow::Result;
use narrate::{colored::Colorize, report, ExitCode};

use cli_parser::{Command, WeatherCli};
use providers::{Provider, NOT_IMPLEMENTED_PROVIDERS};

/// The name of the application.
const APP_NAME: &str = "weather-rs";

/// The name of the configuration file.
const CONFIG_NAME: &str = "config";

/// Main function of the weather-rs application.
///
/// This is the main function of the weather-rs application. It initializes the application, runs the main logic,
/// and handles any errors that may occur during execution.
#[tokio::main]
async fn main() {
    let result = entry_point().await;

    if let Err(ref err) = result {
        report::anyhow_err_full(err);
        std::process::exit(err.exit_code());
    } else {
        std::process::exit(0);
    }
}

/// The entry point of the weather-rs application.
///
/// This function serves as the entry point of the application and is responsible for initializing the application,
/// parsing command-line arguments, and executing the appropriate command handler based on the parsed command.
///
/// # Returns
///
/// A `Result` indicating the success or failure of the application's main logic.
async fn entry_point() -> Result<()> {
    let weather_cli = WeatherCli::parse();
    let mut config: MainConfig = confy::load(APP_NAME, CONFIG_NAME)?;

    match weather_cli.take_command() {
        Command::ProviderList => {
            let selected_provider = config.selected_provider;
            let configured_providers = vec![
                if config.open_weather.is_some() {
                    Some(&Provider::OpenWeather)
                } else {
                    None
                },
                if config.weather_api.is_some() {
                    Some(&Provider::WeatherApi)
                } else {
                    None
                },
                if config.accu_weather.is_some() {
                    Some(&Provider::AccuWeather)
                } else {
                    None
                },
                if config.aeris_weather.is_some() {
                    Some(&Provider::AerisWeather)
                } else {
                    None
                },
            ]
            .into_iter()
            .flatten()
            .collect();
            let not_implemented_providers = NOT_IMPLEMENTED_PROVIDERS.to_vec();

            handlers::provider_list_handler(
                &selected_provider,
                configured_providers,
                not_implemented_providers,
            );
        }
        Command::Configure {
            provider,
            url,
            api_key,
        } => {
            handlers::configure_provider(&mut config, &provider, url, api_key);

            confy::store(APP_NAME, CONFIG_NAME, config)?;

            println!(
                "Provider '{}' was successfully configured",
                &provider.to_string().green()
            );
        }
        Command::SelectProvider { provider } => {
            handlers::select_provider(&mut config, provider.clone());

            confy::store(APP_NAME, CONFIG_NAME, config)?;

            println!(
                "Provider '{}' was successfully selected",
                provider.to_string().green()
            );
        }
        Command::Get {
            address,
            date,
            json,
            provider,
        } => {
            let provider = if let Some(provider) = provider {
                provider
            } else {
                config.selected_provider.clone()
            };

            handlers::get_weather_info(&address, &date, json, &provider, config).await?;
        }
    }

    Ok(())
}
