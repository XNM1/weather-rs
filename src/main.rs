mod cli_parser;
mod handlers;

use clap::Parser;
use cli_parser::Command;
use cli_parser::WeatherCli;
use color_eyre::Result;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .install()?;

    let weather_cli = WeatherCli::parse();
    match weather_cli.get_command() {
        Command::ProviderList => {
            handlers::provider_list_handler(&cli_parser::Provider::OpenWeather)
        }
        Command::Configure { provider } => {}
        Command::SelectProvider { provider } => {}
        Command::Get {
            address,
            date,
            json,
            provider,
        } => {
            handlers::get_weather_info(address, date, json, &cli_parser::Provider::OpenWeather)
                .await?
        }
    }

    Ok(())
}
