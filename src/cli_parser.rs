mod errors;

use anyhow::Result;
use clap::{Parser, Subcommand};
use errors::ProviderError;
use std::fmt;
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// A quick and easy CLI tool for fetching weather data from various providers
pub struct WeatherCLI {
    #[command(subcommand)]
    command: Command,
}

impl WeatherCLI {
    pub fn get_command(&self) -> &Command {
        &self.command
    }
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Command {
    /// Get a full list of supported providers
    ProviderList,
    /// Configure a provider with the given credentials
    Configure {
        /// The provider to be configured
        provider: Provider,
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

        /// Provider for weather data (optional)
        #[arg(short, long)]
        provider: Option<Provider>,
    },
}

#[derive(Clone, Debug, PartialEq)]
pub enum Provider {
    OpenWeather,
    WeatherApi,
    AccuWeather,
    AerisWeather,
}

impl FromStr for Provider {
    type Err = ProviderError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "open-weather" => Ok(Provider::OpenWeather),
            "weather-api" => Ok(Provider::WeatherApi),
            "accu-weather" => Ok(Provider::AccuWeather),
            "aeris-weather" => Ok(Provider::AerisWeather),
            _ => Err(ProviderError::ProviderNotFound),
        }
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Provider::OpenWeather => write!(f, "open-weather"),
            Provider::WeatherApi => write!(f, "weather-api"),
            Provider::AccuWeather => write!(f, "accu-weather"),
            Provider::AerisWeather => write!(f, "aeris-weather"),
        }
    }
}

impl Provider {
    /// Return all available variants of the Provider enum
    pub fn get_all_variants() -> [Provider; 4] {
        [
            Provider::OpenWeather,
            Provider::WeatherApi,
            Provider::AccuWeather,
            Provider::AerisWeather,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("open-weather", Provider::OpenWeather)]
    #[case("weather-api", Provider::WeatherApi)]
    #[case("accu-weather", Provider::AccuWeather)]
    #[case("aeris-weather", Provider::AerisWeather)]
    fn test_from_str_valid_input(#[case] input: &str, #[case] expected: Provider) {
        let result = Provider::from_str(input);
        assert_eq!(result, Ok(expected));
    }

    #[rstest]
    #[case("invalid-provider")]
    #[case("unknown-provider")]
    fn test_from_str_invalid_input(#[case] input: &str) {
        let result = Provider::from_str(input);
        assert_eq!(result, Err(ProviderError::ProviderNotFound));
    }

    #[rstest]
    #[case(Provider::OpenWeather, "open-weather")]
    #[case(Provider::WeatherApi, "weather-api")]
    #[case(Provider::AccuWeather, "accu-weather")]
    #[case(Provider::AerisWeather, "aeris-weather")]
    fn test_to_string(#[case] input: Provider, #[case] expected: &str) {
        let result = input.to_string();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case([Provider::OpenWeather, Provider::WeatherApi, Provider::AccuWeather, Provider::AerisWeather])]
    fn test_get_all_variants(#[case] expected: [Provider; 4]) {
        let variants = Provider::get_all_variants();
        assert_eq!(variants, expected);
    }

    #[test]
    fn test_get_command() {
        let command = Command::ProviderList;
        let weather_cli = WeatherCLI { command };

        let result = weather_cli.get_command();

        assert_eq!(&Command::ProviderList, result);
    }
}
