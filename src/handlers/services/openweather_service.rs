use async_trait::async_trait;
use color_eyre::Result;
use dateparser::parse as parse_datetime_from_str;
use reqwest::Client;
use std::collections::HashMap;

use super::*;
use models::WeatherDataError;
use openweather_model::OpenWeatherData;

#[derive(Error, Debug)]
pub enum OpenWeatherError {
    #[error("Failed to send a request to the Open Weather API")]
    RequestError(#[from] reqwest::Error),
}

pub struct OpenWeatherApi {
    url: String,
    api_key: String,
    client: Client,
}

impl OpenWeatherApi {
    pub fn new(client: Client, url: String, api_key: String) -> Self {
        OpenWeatherApi {
            client,
            url,
            api_key,
        }
    }

    #[allow(dead_code)]
    pub fn get_url(&self) -> &str {
        &self.url
    }
}

#[async_trait]
impl WeatherApi for OpenWeatherApi {
    async fn get_weather_data(&self, address: &str, date: &Option<String>) -> Result<WeatherData> {
        let mut params = HashMap::new();

        params.insert("q", address.to_owned());
        params.insert("units", "metric".to_owned());
        params.insert("appid", self.api_key.to_owned());
        if let Some(date) = date {
            let timestamp = parse_datetime_from_str(date)
                .map_err(|_| DateTimeError::ParseError)?
                .timestamp();
            params.insert("dt", timestamp.to_string());
        }

        let client = &self.client;
        let url = &self.url;

        let response = client
            .get(url)
            .query(&params)
            .send()
            .await
            .map_err(OpenWeatherError::RequestError)?;

        let openweather_data: OpenWeatherData = serde_json::from_str(
            &response
                .text()
                .await
                .map_err(OpenWeatherError::RequestError)?,
        )
        .map_err(WeatherDataError::JsonParseError)?;

        let weather_data = openweather_data.into();

        Ok(weather_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    mod tests_openweatherapi_struct {
        use super::*;

        #[rstest]
        #[case(
            "https://api.openweathermap.org",
            "my_openweather_api_key",
            "https://api.openweathermap.org"
        )]
        #[case("https://example.com", "my_example_api_key", "https://example.com")]
        fn test_openweather_api_creation(
            #[case] url: &str,
            #[case] api_key: &str,
            #[case] expected_url: &str,
        ) {
            let client = Client::new();
            let api = OpenWeatherApi::new(client, url.to_string(), api_key.to_string());

            assert_eq!(api.url, expected_url);
            assert_eq!(api.api_key, api_key);
        }

        #[rstest]
        #[case(
            "https://api.openweathermap.org",
            "my_openweather_api_key",
            "https://api.openweathermap.org"
        )]
        fn test_get_url_method(
            #[case] url: &str,
            #[case] api_key: &str,
            #[case] expected_url: &str,
        ) {
            let client = Client::new();
            let api = OpenWeatherApi::new(client, url.to_string(), api_key.to_string());

            assert_eq!(api.get_url(), expected_url);
        }

        #[rstest]
        #[case("", "", "")]
        fn test_openweather_api_with_empty_url_and_api_key(
            #[case] url: &str,
            #[case] api_key: &str,
            #[case] expected_url: &str,
        ) {
            let client = Client::new();
            let api = OpenWeatherApi::new(client, url.to_string(), api_key.to_string());

            assert_eq!(api.get_url(), expected_url);
        }
    }

    mod tests_get_weather_data {
        use super::*;
        use serde_json::json;

        #[allow(clippy::too_many_arguments)]
        fn mock_openweather_server(
            address: &str,
            date: Option<&str>,
            temp: f32,
            humidity: u8,
            pressure: u32,
            wind_speed: f32,
            visibility: u32,
            description: &str,
            api_key: &str,
        ) -> (mockito::ServerGuard, Vec<mockito::Mock>) {
            let mock_response = json!(
                {
                    "main": {"temp": temp, "humidity": humidity, "pressure": pressure},
                    "wind": {"speed": wind_speed},
                    "visibility": visibility,
                    "weather": [{"description": description}]
                }
            );

            let mut mock_server = mockito::Server::new();

            let mock_endpoint_with_date = mock_server
                .mock("GET", "/data/2.5/weather")
                .match_query(mockito::Matcher::UrlEncoded("q".into(), address.into()))
                .match_query(mockito::Matcher::UrlEncoded(
                    "units".into(),
                    "metric".into(),
                ))
                .match_query(mockito::Matcher::UrlEncoded("appid".into(), api_key.into()))
                .match_query(mockito::Matcher::UrlEncoded(
                    "dt".into(),
                    date.unwrap_or_default().into(),
                ))
                .with_status(200)
                .with_header("content-type", "text/json")
                .with_body(mock_response.to_string())
                .create();

            let mock_endpoint_wihtout_date = mock_server
                .mock("GET", "/data/2.5/weather")
                .match_query(mockito::Matcher::UrlEncoded("q".into(), address.into()))
                .match_query(mockito::Matcher::UrlEncoded(
                    "units".into(),
                    "metric".into(),
                ))
                .match_query(mockito::Matcher::UrlEncoded("appid".into(), api_key.into()))
                .with_status(200)
                .with_header("content-type", "text/json")
                .with_body(mock_response.to_string())
                .create();

            let mock_endpoints = vec![mock_endpoint_with_date, mock_endpoint_wihtout_date];

            (mock_server, mock_endpoints)
        }

        #[rstest]
        #[case(
            "CityName",
            None,
            200.0,
            50,
            1013,
            5.0,
            10000,
            "Cloudy",
            "hdx19j9qjcsd90jmwc123fg"
        )]
        #[case(
            "AnotherCity",
            Some("2023-10-15"),
            22.0,
            60,
            1005,
            12.0,
            8000,
            "Rainy",
            "fo2mjeqjssdj0jmlc123fg"
        )]
        #[case(
            "ThirdCity",
            Some("2023-10-16"),
            25.0,
            70,
            1010,
            8.0,
            12000,
            "Sunny",
            "a1b2c3d4e5f6g7h8"
        )]
        #[case(
            "FourthCity",
            None,
            18.5,
            45,
            1015,
            6.5,
            9500,
            "Partly Cloudy",
            "xyz987pqr321lmn456"
        )]
        #[case(
            "FifthCity",
            Some("2023-10-17"),
            30.5,
            80,
            1002,
            15.0,
            6000,
            "Stormy",
            "abc123def456ghi789"
        )]
        #[tokio::test]
        #[allow(clippy::too_many_arguments)]
        async fn test_get_weather_data(
            #[case] address: &str,
            #[case] date: Option<&str>,
            #[case] temp: f32,
            #[case] humidity: u8,
            #[case] pressure: u32,
            #[case] wind_speed: f32,
            #[case] visibility: u32,
            #[case] description: &str,
            #[case] api_key: &str,
        ) {
            let (mock_server, _) = mock_openweather_server(
                address,
                date,
                temp,
                humidity,
                pressure,
                wind_speed,
                visibility,
                description,
                api_key,
            );

            let url = mock_server.url();
            let client = Client::new();
            let api = OpenWeatherApi::new(
                client,
                url.to_string() + "/data/2.5/weather",
                api_key.to_string(),
            );

            let result = api
                .get_weather_data(address, &date.map(|d| d.to_string()))
                .await
                .unwrap();

            assert_eq!(result.temp, temp);
            assert_eq!(result.humidity, humidity);
            assert_eq!(result.pressure, pressure);
            assert_eq!(result.wind_speed, wind_speed);
            assert_eq!(result.visibility, visibility);
            assert_eq!(result.description, description);
        }

        #[rstest]
        #[case(
            "AnotherCity",
            Some("InvalidDate"),
            22.0,
            60,
            1005,
            12.0,
            8000,
            "Rainy",
            "fo2mjeqjssdj0jmlc123fg"
        )]
        #[tokio::test]
        #[allow(clippy::too_many_arguments)]
        async fn test_get_weather_data_date_parse_error(
            #[case] address: &str,
            #[case] date: Option<&str>,
            #[case] temp: f32,
            #[case] humidity: u8,
            #[case] pressure: u32,
            #[case] wind_speed: f32,
            #[case] visibility: u32,
            #[case] description: &str,
            #[case] api_key: &str,
        ) {
            let (mock_server, _) = mock_openweather_server(
                address,
                date,
                temp,
                humidity,
                pressure,
                wind_speed,
                visibility,
                description,
                api_key,
            );

            let url = mock_server.url();
            let client = Client::new();
            let api = OpenWeatherApi::new(
                client,
                url.to_string() + "/data/2.5/weather",
                api_key.to_string(),
            );

            let result: DateTimeError = api
                .get_weather_data(address, &date.map(|d| d.to_string()))
                .await
                .unwrap_err()
                .downcast()
                .unwrap();

            assert_eq!(result, DateTimeError::ParseError);
        }

        #[rstest]
        #[tokio::test]
        async fn test_get_weather_data_request_error() {
            let address = "SomeCity";
            let api_key = "123";

            let url = "http://invalid-url";
            let client = Client::new();
            let api = OpenWeatherApi::new(
                client,
                url.to_string() + "/data/2.5/weather",
                api_key.to_string(),
            );

            let result: OpenWeatherError = api
                .get_weather_data(address, &None)
                .await
                .unwrap_err()
                .downcast()
                .unwrap();

            assert!(matches!(result, OpenWeatherError::RequestError(_)));
        }

        #[rstest]
        #[tokio::test]
        async fn test_get_weather_data_json_parse_error() {
            let address = "SomeCity";
            let api_key = "123";
            let mut mock_server = mockito::Server::new();
            mock_server
                .mock("GET", "/data/2.5/weather")
                .match_query(mockito::Matcher::UrlEncoded("q".into(), address.into()))
                .match_query(mockito::Matcher::UrlEncoded(
                    "units".into(),
                    "metric".into(),
                ))
                .match_query(mockito::Matcher::UrlEncoded("appid".into(), api_key.into()))
                .with_status(200)
                .with_body("invalid json")
                .create();

            let url = mock_server.url();
            let client = Client::new();
            let api = OpenWeatherApi::new(
                client,
                url.to_string() + "/data/2.5/weather",
                api_key.to_string(),
            );

            let result: WeatherDataError = api
                .get_weather_data(address, &None)
                .await
                .unwrap_err()
                .downcast()
                .unwrap();

            assert!(matches!(result, WeatherDataError::JsonParseError(_)));
        }
    }
}
