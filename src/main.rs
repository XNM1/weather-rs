mod cli_parser;
mod config;
mod handlers;
mod providers;

use clap::Parser;
use config::config_model::MainConfig;
use narrate::{colored::Colorize, report, ExitCode, Result};

use cli_parser::{Command, WeatherCli};
use providers::{Provider, NOT_IMPLEMENTED_PROVIDERS};

const APP_NAME: &str = "weather-rs";
const CONFIG_NAME: &str = "config";

#[tokio::main]
async fn main() {
    let result = entry_point().await;

    if let Err(ref err) = result {
        report::err_full(err);
        std::process::exit(err.exit_code());
    } else {
        std::process::exit(0);
    }
}

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
