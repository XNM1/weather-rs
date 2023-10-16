use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ProviderError {
    #[error("Weather provider not found; use the command 'weather-rs provider-list' to get a list of all available providers")]
    ProviderNotFound,

    #[error("Weather provider is not implemented; use the command 'weather-rs provider-list' to get a list of all available providers")]
    ProviderNotImplemented,
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
/// Enum for available providers
pub enum Provider {
    #[default]
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
}
