use clap::{Parser, Subcommand};

use crate::providers::Provider;

/// The `WeatherCli` struct represents a command-line interface for weather-related operations.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct WeatherCli {
    #[command(subcommand)]
    command: Command,
}

/// Methods for `WeatherCLI` for working with commands
impl WeatherCli {
    /// Gets a reference to the command stored in the `WeatherCli`.
    ///
    /// # Returns
    ///
    /// A reference to the `Command` enum stored in the `WeatherCli`.
    #[allow(dead_code)]
    pub fn get_command(&self) -> &Command {
        &self.command
    }

    /// Takes ownership of the `Command` enum stored in the `WeatherCli`.
    ///
    /// # Returns
    ///
    /// The `Command` enum previously stored in the `WeatherCli`.
    pub fn take_command(self) -> Command {
        self.command
    }
}

/// Enum for CLI commands
#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    /// Get a full list of supported providers
    ProviderList,
    /// Configure a provider with the given credentials
    Configure {
        /// The provider to be configured (Example: 'open-weather', 'weather-api')
        provider: Provider,

        /// API Service URL (Example: Open Weather API - 'https://api.openweathermap.org/data/2.5/weather') (optional)
        #[arg(short, long)]
        url: Option<String>,

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
