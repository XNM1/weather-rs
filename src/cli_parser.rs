mod errors;

use anyhow::Result;
use clap::{Parser, Subcommand};
use errors::ProviderError;
use std::str::FromStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
/// A quick and easy CLI tool for fetching weather data from various providers
pub struct WeatherCLI {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
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

#[derive(Clone)]
enum Provider {
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
            _ => Err(ProviderError::NoProvider),
        }
    }
}
