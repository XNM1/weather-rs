pub mod openweather_model;

use serde::Serialize;
use thiserror::Error;

use openweather_model::OpenWeatherData;

#[derive(Error, Debug)]
pub enum WeatherDataError {
    #[error("Failed to parse JSON response")]
    JsonParseError(#[from] serde_json::Error),
}

#[derive(Serialize, Debug)]
pub struct WeatherData {
    pub temp: f32,
    pub humidity: u8,
    pub pressure: u32,
    pub wind_speed: f32,
    pub visibility: u32,
    pub description: String,
}

impl From<OpenWeatherData> for WeatherData {
    fn from(open_weather_data: OpenWeatherData) -> Self {
        let main = open_weather_data.main;
        let mut weather = open_weather_data.weather;
        let wind = open_weather_data.wind;

        WeatherData {
            temp: main.temp,
            humidity: main.humidity,
            pressure: main.pressure,
            wind_speed: wind.speed,
            visibility: open_weather_data.visibility,
            description: weather.pop().map_or_else(String::new, |w| w.description),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[rstest]
    #[case(input_open_weather_data(), expected_weather_data())]
    fn test_weather_data_conversion(
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
}
