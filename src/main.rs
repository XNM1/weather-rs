mod cli_parser;
mod handlers;

use clap::Parser;
use cli_parser::Command;
use cli_parser::WeatherCLI;

fn main() {
    let weather_cli = WeatherCLI::parse();
    match weather_cli.get_command() {
        Command::ProviderList => handlers::provider_list_handler(),
        Command::Configure { provider } => {}
        Command::SelectProvider { provider } => {}
        Command::Get {
            address,
            date,
            provider,
        } => {}
    }
}
