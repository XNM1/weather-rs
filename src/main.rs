mod cli_parser;
mod config;
mod handlers;
mod providers;

use clap::Parser;
use narrate::{report, ExitCode, Result};

use cli_parser::{Command, WeatherCli};
use providers::Provider;

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
    match weather_cli.get_command() {
        Command::ProviderList => {
            handlers::provider_list_handler(&Provider::OpenWeather);
        }
        Command::Configure {
            provider,
            url,
            api_key,
        } => {
            handlers::configure_provider(provider, url, api_key);
        }
        Command::SelectProvider { provider } => {}
        Command::Get {
            address,
            date,
            json,
            provider,
        } => {
            handlers::get_weather_info(address, date, json, provider).await?;
        }
    }

    Ok(())
}
