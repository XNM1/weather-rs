use clap::{Parser, Subcommand};

use crate::providers::Provider;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// Struct for CLI commands & arguments parsing
pub struct WeatherCli {
    #[command(subcommand)]
    command: Command,
}

impl WeatherCli {
    #[allow(dead_code)]
    pub fn get_command(&self) -> &Command {
        &self.command
    }

    pub fn take_command(self) -> Command {
        self.command
    }
}

#[derive(Subcommand, Debug, PartialEq)]
/// Enum for CLI commands
pub enum Command {
    /// Get a full list of supported providers
    ProviderList,
    /// Configure a provider with the given credentials
    Configure {
        /// The provider to be configured
        provider: Provider,

        /// API Service URL (Example: Open Weather API - 'https://api.openweathermap.org/data/2.5/weather' or 'https://api.openweathermap.org/data/3.0/onecall')
        url: String,

        /// The API key for a service provider
        api_key: String,
    },
    /// Select an available provider
    SelectProvider {
        /// The provider to be selected
        provider: Provider,
    },
    /// Get weather information
    Get {
        /// The address for which weather information is requested
        address: String,

        /// Date for specific weather information (optional)
        #[arg(short, long)]
        date: Option<String>,

        /// Get weather data in JSON format flag (optional)
        #[arg(short, long)]
        json: bool,

        /// Provider for weather data (optional)
        #[arg(short, long)]
        provider: Option<Provider>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    fn test_get_command() {
        let command = Command::ProviderList;
        let weather_cli = WeatherCli { command };

        let result = weather_cli.get_command();

        assert_eq!(result, &Command::ProviderList);
    }

    #[rstest]
    fn test_take_command() {
        let command = Command::ProviderList;
        let weather_cli = WeatherCli { command };

        let result = weather_cli.take_command();

        assert_eq!(result, Command::ProviderList);
    }
}
