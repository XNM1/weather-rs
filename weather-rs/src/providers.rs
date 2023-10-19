use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use thiserror::Error;

/// An array of providers that are not implemented.
///
/// This array lists the weather data providers that are not implemented in the current version
/// of the application.
pub const NOT_IMPLEMENTED_PROVIDERS: [&Provider; 2] =
    [&Provider::AccuWeather, &Provider::AerisWeather];

/// Represents errors related to weather data providers.
#[derive(Error, Debug)]
pub enum ProviderError {
    /// An error indicating that a weather data provider was not found.
    ///
    /// This error occurs when an attempt is made to use a weather data provider that does not exist
    /// or is not recognized.
    #[error("Weather provider not found; use the command 'weather-rs provider-list' to get a list of all available providers")]
    ProviderNotFound,

    /// An error indicating that a weather data provider is not implemented.
    ///
    /// This error occurs when an attempt is made to use a weather data provider that is not yet
    /// implemented in the current version of the application.
    #[error("Weather provider is not implemented; use the command 'weather-rs provider-list' to get a list of all available providers")]
    ProviderNotImplemented,
}

/// Represents weather data providers available in the application.
///
/// This enum represents the available weather data providers in the application.
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum Provider {
    #[default]
    OpenWeather,
    WeatherApi,
    AccuWeather,
    AerisWeather,
}

impl FromStr for Provider {
    type Err = ProviderError;

    /// Converts a string to a Provider enum variant.
    ///
    /// This method attempts to parse a string and convert it into a Provider enum variant.
    /// It returns a Result containing the parsed variant or a ProviderError if the string
    /// does not match any known providers.
    ///
    /// # Arguments
    ///
    /// * `s` - A string representing the provider name to be parsed.
    ///
    /// # Returns
    ///
    /// A Result containing the parsed Provider variant or a ProviderError if the string is not recognized.
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
    /// Formats the Provider enum variant as a string.
    ///
    /// This method formats a Provider enum variant as a string, which can be useful for
    /// displaying provider names or for configuration purposes.
    ///
    /// # Arguments
    ///
    /// * `self` - The Provider enum variant to be formatted.
    ///
    /// # Returns
    ///
    /// A Result containing the formatted string result.
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
    /// Returns all available variants of the Provider enum.
    ///
    /// This method returns an array containing all available variants of the Provider enum.
    ///
    /// # Returns
    ///
    /// An array containing all available Provider enum variants.
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
        let result = Provider::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case("invalid-provider")]
    #[case("unknown-provider")]
    fn test_from_str_invalid_input(#[case] input: &str) {
        let result = Provider::from_str(input).unwrap_err();
        assert!(matches!(result, ProviderError::ProviderNotFound));
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
