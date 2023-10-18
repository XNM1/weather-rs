/// Module that contains structs that represent data from OpenWeather API provider
pub mod openweather_model;
/// Module that contains structs that represent data from Weather API provider
pub mod weatherapi_model;

use serde::Serialize;
use thiserror::Error;

use openweather_model::OpenWeatherData;
use weatherapi_model::WeatherApiData;

use self::weatherapi_model::WeatherApiHistoryData;

/// Represents an error that occurs when there is an issue with parsing JSON response data.
#[derive(Error, Debug)]
pub enum WeatherDataError {
    /// An error indicating a failure to parse JSON response.
    ///
    /// This error occurs when there is an issue with parsing JSON response data,
    /// and the specific `serde_json::Error` is included as a parameter.
    ///
    /// # Parameters
    ///
    /// * `0` - The `serde_json::Error` indicating the specific JSON parsing error.
    #[error("Failed to parse JSON response")]
    JsonParse(#[from] serde_json::Error),
}

/// Represents weather data with temperature, humidity, pressure, wind speed, visibility, and description.
#[derive(Serialize, Debug)]
pub struct WeatherData {
    pub temp: f32,
    pub humidity: u8,
    pub pressure: u16,
    pub wind_speed: f32,
    pub visibility: u16,
    pub description: String,
}

/// Converts data from OpenWeather API to `WeatherData`
impl From<OpenWeatherData> for WeatherData {
    fn from(openweather_data: OpenWeatherData) -> Self {
        let main = openweather_data.main;
        let mut weather = openweather_data.weather;
        let wind = openweather_data.wind;

        WeatherData {
            temp: main.temp,
            humidity: main.humidity,
            pressure: main.pressure,
            wind_speed: wind.speed,
            visibility: openweather_data.visibility,
            description: weather.pop().map_or_else(String::new, |w| w.description),
        }
    }
}

/// Converts data from Weather API to `WeatherData`.
impl From<WeatherApiData> for WeatherData {
    fn from(weatherapi_data: WeatherApiData) -> Self {
        let current = weatherapi_data.current;

        WeatherData {
            temp: current.temp_c,
            humidity: current.humidity,
            pressure: current.pressure_mb as u16,
            wind_speed: km_per_hour_to_m_per_sec(current.wind_kph),
            visibility: km_to_m(current.vis_km),
            description: current.condition.text,
        }
    }
}

/// Converts data for specific date in history from Weather API to `WeatherData`.
impl From<WeatherApiHistoryData> for WeatherData {
    fn from(mut weatherapi_history_data: WeatherApiHistoryData) -> Self {
        let currents = weatherapi_history_data
            .forecast
            .forecastday
            .pop()
            .unwrap()
            .hour;
        let current = currents.get(0).unwrap();

        WeatherData {
            temp: current.temp_c,
            humidity: current.humidity,
            pressure: current.pressure_mb as u16,
            wind_speed: km_per_hour_to_m_per_sec(current.wind_kph),
            visibility: km_to_m(current.vis_km),
            description: current.condition.text.clone(),
        }
    }
}

/// Converts kilometers per hour to meters per second.
fn km_per_hour_to_m_per_sec(km_per_hour: f32) -> f32 {
    km_per_hour * (1000.0 / 3600.0)
}

/// Converts kilometers to meters.
fn km_to_m(km: f32) -> u16 {
    (km * 1000.0) as u16
}

#[cfg(test)]
mod tests {
    use super::{
        weatherapi_model::{HistoryForecast, HistoryForecastDay, WeatherCondition, WeatherCurrent},
        *,
    };
    use openweather_model::*;
    use rstest::*;

    #[fixture]
    fn expected_weather_data() -> WeatherData {
        WeatherData {
            temp: 25.5,
            humidity: 50,
            pressure: 1010,
            wind_speed: 10.0,
            visibility: 10000,
            description: "Partly Cloudy".to_string(),
        }
    }

    #[fixture]
    fn input_open_weather_data() -> OpenWeatherData {
        OpenWeatherData {
            main: WeatherMain {
                temp: 25.5,
                humidity: 50,
                pressure: 1010,
            },
            weather: vec![Weather {
                description: "Partly Cloudy".to_string(),
            }],
            visibility: 10000,
            wind: Wind { speed: 10.0 },
        }
    }

    #[fixture]
    fn input_weather_api_data() -> WeatherApiData {
        WeatherApiData {
            current: WeatherCurrent {
                temp_c: 25.5,
                condition: WeatherCondition {
                    text: "Partly Cloudy".to_string(),
                },
                wind_kph: 36.0,
                pressure_mb: 1010.0,
                humidity: 50,
                vis_km: 10.0,
            },
        }
    }

    #[fixture]
    fn input_weather_history_api_data() -> WeatherApiHistoryData {
        WeatherApiHistoryData {
            forecast: HistoryForecast {
                forecastday: vec![HistoryForecastDay {
                    hour: vec![WeatherCurrent {
                        temp_c: 25.5,
                        condition: WeatherCondition {
                            text: "Partly Cloudy".to_string(),
                        },
                        wind_kph: 36.0,
                        pressure_mb: 1010.0,
                        humidity: 50,
                        vis_km: 10.0,
                    }],
                }],
            },
        }
    }

    #[rstest]
    #[case(input_open_weather_data(), expected_weather_data())]
    fn test_weather_data_conversion_open_weather(
        #[case] input_open_weather_data: OpenWeatherData,
        #[case] expected_weather_data: WeatherData,
    ) {
        let result: WeatherData = input_open_weather_data.into();
        assert_eq!(result.temp, expected_weather_data.temp);
        assert_eq!(result.humidity, expected_weather_data.humidity);
        assert_eq!(result.pressure, expected_weather_data.pressure);
        assert_eq!(result.wind_speed, expected_weather_data.wind_speed);
        assert_eq!(result.visibility, expected_weather_data.visibility);
        assert_eq!(result.description, expected_weather_data.description);
    }

    #[rstest]
    #[case(input_weather_api_data(), expected_weather_data())]
    fn test_weather_data_conversion_weather_api(
        #[case] input_weather_api_data: WeatherApiData,
        #[case] expected_weather_data: WeatherData,
    ) {
        let result: WeatherData = input_weather_api_data.into();
        assert_eq!(result.temp, expected_weather_data.temp);
        assert_eq!(result.humidity, expected_weather_data.humidity);
        assert_eq!(result.pressure, expected_weather_data.pressure);
        assert_eq!(result.wind_speed, expected_weather_data.wind_speed);
        assert_eq!(result.visibility, expected_weather_data.visibility);
        assert_eq!(result.description, expected_weather_data.description);
    }

    #[rstest]
    #[case(input_weather_history_api_data(), expected_weather_data())]
    fn test_weather_data_conversion_weather_api_history(
        #[case] input_weather_api_history_data: WeatherApiHistoryData,
        #[case] expected_weather_data: WeatherData,
    ) {
        let result: WeatherData = input_weather_api_history_data.into();
        assert_eq!(result.temp, expected_weather_data.temp);
        assert_eq!(result.humidity, expected_weather_data.humidity);
        assert_eq!(result.pressure, expected_weather_data.pressure);
        assert_eq!(result.wind_speed, expected_weather_data.wind_speed);
        assert_eq!(result.visibility, expected_weather_data.visibility);
        assert_eq!(result.description, expected_weather_data.description);
    }
}
