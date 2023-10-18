use anyhow::Result;
use dateparser::parse as parse_datetime_from_str;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;

use super::{models::openweather_model::OpenWeatherErrorData, *};
use models::WeatherDataError;
use openweather_model::OpenWeatherData;

/// Struct that implement the `WeatherApi` trait and interacts with the OpenWeather API.
#[derive(Debug)]
pub struct OpenWeatherApiService {
    url: String,
    api_key: String,
    client: Client,
}

/// `OpenWeatherApiService` constructors and methods
impl OpenWeatherApiService {
    /// Creates a new instance of `OpenWeatherApiService`.
    ///
    /// # Arguments
    ///
    /// * `client` - The HTTP client (reqwest) to use for making requests.
    /// * `url` - The base URL for the OpenWeather API.
    /// * `api_key` - The API key required for authentication.
    ///
    /// # Returns
    ///
    /// A `Result` containing the initialized `OpenWeatherApiService` or an error if initialization fails.
    pub fn new(client: Client, mut url: String, api_key: String) -> Result<Self> {
        if url.is_empty() || api_key.is_empty() {
            return Err(WeatherApiError::Creation.into());
        }

        // url cleaning
        if url.ends_with('/') {
            url.pop();
        }

        Ok(OpenWeatherApiService {
            client,
            url,
            api_key,
        })
    }

    /// Retrieves the URL of the OpenWeather API service.
    ///
    /// # Returns
    ///
    /// A reference to the URL string.
    #[allow(dead_code)]
    pub fn get_url(&self) -> &str {
        &self.url
    }
}

/// An implementation of the `WeatherApi` trait for OpenWeather API service.
#[async_trait]
impl WeatherApi for OpenWeatherApiService {
    /// Asynchronously retrieves weather data for a specific address and date (if provided).
    ///
    /// # Arguments
    ///
    /// * `address` - A string representing the address for which weather data is requested.
    /// * `date` - An optional string containing the date for historical weather data. Pass `None` for current weather.
    ///
    /// # Returns
    ///
    /// A `Result` containing the retrieved weather data or an error if the request fails.
    async fn get_weather_data(&self, address: &str, date: &Option<String>) -> Result<WeatherData> {
        let mut params = HashMap::new();

        params.insert("q", address.to_owned());
        params.insert("units", "metric".to_owned());
        params.insert("appid", self.api_key.to_owned());
        if let Some(date) = date {
            let timestamp = parse_datetime_from_str(date)
                .map_err(|_| DateTimeError::Parse)?
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
            .map_err(WeatherApiError::Request)?;

        let status_code = response.status();

        let response_body = &response
            .text()
            .await
            .map_err(|_| WeatherApiError::BodyText)?;

        if status_code == StatusCode::OK {
            let openweather_data: OpenWeatherData =
                serde_json::from_str(response_body).map_err(WeatherDataError::JsonParse)?;

            let weather_data = openweather_data.into();

            Ok(weather_data)
        } else {
            let weather_error_data: OpenWeatherErrorData =
                serde_json::from_str(response_body).map_err(WeatherDataError::JsonParse)?;

            Err(WeatherApiError::Server(weather_error_data.message).into())
        }
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
            "https://api.openweathermap.org/",
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
            let api =
                OpenWeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

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
            let api =
                OpenWeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

            assert_eq!(api.get_url(), expected_url);
        }

        #[rstest]
        #[case("", "")]
        #[case("", "some key")]
        #[case("some url", "")]
        fn test_openweather_api_with_empty_url_and_api_key(
            #[case] url: &str,
            #[case] api_key: &str,
        ) {
            let client = Client::new();
            let api = OpenWeatherApiService::new(client, url.to_string(), api_key.to_string())
                .unwrap_err()
                .downcast()
                .unwrap();

            assert!(matches!(api, WeatherApiError::Creation));
        }
    }

    mod tests_get_weather_data {
        use super::*;
        use serde_json::json;

        #[allow(clippy::too_many_arguments)]
        fn mock_openweather_server(
            address: &str,
            temp: f32,
            humidity: u8,
            pressure: u16,
            wind_speed: f32,
            visibility: u16,
            description: &str,
            api_key: &str,
        ) -> (mockito::ServerGuard, mockito::Mock) {
            let mock_response = json!(
                {
                    "main": {"temp": temp, "humidity": humidity, "pressure": pressure},
                    "wind": {"speed": wind_speed},
                    "visibility": visibility,
                    "weather": [{"description": description}]
                }
            );

            let mut mock_server = mockito::Server::new();

            let mock_endpoint = mock_server
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

            (mock_server, mock_endpoint)
        }

        #[rstest]
        #[case("CityName", 200.0, 50, 1013, 5.0, 10000, "Cloudy")]
        #[case("FourthCity", 18.5, 45, 1015, 6.5, 9500, "Partly Cloudy")]
        #[tokio::test]
        #[allow(clippy::too_many_arguments)]
        async fn test_get_weather_data(
            #[case] address: &str,
            #[case] temp: f32,
            #[case] humidity: u8,
            #[case] pressure: u16,
            #[case] wind_speed: f32,
            #[case] visibility: u16,
            #[case] description: &str,
        ) {
            let api_key = "SomeApiKey";
            let (mock_server, mock_endpoint) = mock_openweather_server(
                address,
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
            let api = OpenWeatherApiService::new(
                client,
                url.to_string() + "/data/2.5/weather",
                api_key.to_string(),
            )
            .unwrap();

            let result = api.get_weather_data(address, &None).await.unwrap();

            mock_endpoint.assert();
            assert_eq!(result.temp, temp);
            assert_eq!(result.humidity, humidity);
            assert_eq!(result.pressure, pressure);
            assert_eq!(result.wind_speed, wind_speed);
            assert_eq!(result.visibility, visibility);
            assert_eq!(result.description, description);
        }

        #[allow(clippy::too_many_arguments)]
        fn mock_openweather_history_server(
            address: &str,
            date: &str,
            temp: f32,
            humidity: u8,
            pressure: u16,
            wind_speed: f32,
            visibility: u16,
            description: &str,
            api_key: &str,
        ) -> (mockito::ServerGuard, mockito::Mock) {
            let mock_response = json!(
                {
                    "main": {"temp": temp, "humidity": humidity, "pressure": pressure},
                    "wind": {"speed": wind_speed},
                    "visibility": visibility,
                    "weather": [{"description": description}]
                }
            );

            let mut mock_server = mockito::Server::new();

            let mock_endpoint = mock_server
                .mock("GET", "/data/2.5/weather")
                .match_query(mockito::Matcher::UrlEncoded("q".into(), address.into()))
                .match_query(mockito::Matcher::UrlEncoded(
                    "units".into(),
                    "metric".into(),
                ))
                .match_query(mockito::Matcher::UrlEncoded("appid".into(), api_key.into()))
                .match_query(mockito::Matcher::UrlEncoded("dt".into(), date.into()))
                .with_status(200)
                .with_header("content-type", "text/json")
                .with_body(mock_response.to_string())
                .create();

            (mock_server, mock_endpoint)
        }

        #[rstest]
        #[case("AnotherCity", "2023-10-15 00:00", 22.0, 60, 1005, 12.0, 8000, "Rainy")]
        #[case("ThirdCity", "2023-10-16 00:00", 25.0, 70, 1010, 8.0, 12000, "Sunny")]
        #[case("FifthCity", "2023-10-17 00:00", 30.5, 80, 1002, 15.0, 6000, "Stormy")]
        #[tokio::test]
        #[allow(clippy::too_many_arguments)]
        async fn test_get_weather_data_with_date(
            #[case] address: &str,
            #[case] date: &str,
            #[case] temp: f32,
            #[case] humidity: u8,
            #[case] pressure: u16,
            #[case] wind_speed: f32,
            #[case] visibility: u16,
            #[case] description: &str,
        ) {
            let api_key = "SomeApiKey";
            let (mock_server, mock_endpoint) = mock_openweather_history_server(
                address,
                &parse_datetime_from_str(date)
                    .unwrap()
                    .timestamp()
                    .to_string(),
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
            let api = OpenWeatherApiService::new(
                client,
                url.to_string() + "/data/2.5/weather",
                api_key.to_string(),
            )
            .unwrap();

            let result = api
                .get_weather_data(address, &Some(date.to_owned()))
                .await
                .unwrap();

            mock_endpoint.assert();
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
            "Rainy"
        )]
        #[tokio::test]
        #[allow(clippy::too_many_arguments)]
        async fn test_get_weather_data_date_parse_error(
            #[case] address: &str,
            #[case] date: Option<&str>,
            #[case] temp: f32,
            #[case] humidity: u8,
            #[case] pressure: u16,
            #[case] wind_speed: f32,
            #[case] visibility: u16,
            #[case] description: &str,
        ) {
            let api_key = "SomeApiKey";
            let (mock_server, _) = mock_openweather_history_server(
                address,
                date.unwrap_or_default(),
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
            let api = OpenWeatherApiService::new(
                client,
                url.to_string() + "/data/2.5/weather",
                api_key.to_string(),
            )
            .unwrap();

            let result: DateTimeError = api
                .get_weather_data(address, &date.map(|d| d.to_string()))
                .await
                .unwrap_err()
                .downcast()
                .unwrap();

            assert!(matches!(result, DateTimeError::Parse));
        }

        #[rstest]
        #[tokio::test]
        async fn test_get_weather_data_request_error() {
            let address = "SomeCity";
            let api_key = "123";

            let url = "http://invalid-url";
            let client = Client::new();
            let api = OpenWeatherApiService::new(
                client,
                url.to_string() + "/data/2.5/weather",
                api_key.to_string(),
            )
            .unwrap();

            let result: WeatherApiError = api
                .get_weather_data(address, &None)
                .await
                .unwrap_err()
                .downcast()
                .unwrap();

            assert!(matches!(result, WeatherApiError::Request(_)));
        }

        #[rstest]
        #[tokio::test]
        async fn test_get_weather_data_json_parse_error() {
            let address = "SomeCity";
            let api_key = "123";

            let mut mock_server = mockito::Server::new();
            let mock_endpoint = mock_server
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
            let api = OpenWeatherApiService::new(
                client,
                url.to_string() + "/data/2.5/weather",
                api_key.to_string(),
            )
            .unwrap();

            let result: WeatherDataError = api
                .get_weather_data(address, &None)
                .await
                .unwrap_err()
                .downcast()
                .unwrap();

            mock_endpoint.assert();
            assert!(matches!(result, WeatherDataError::JsonParse(_)));
        }

        #[rstest]
        #[tokio::test]
        async fn test_get_weather_data_server_response_error() {
            let address = "Invalid City";
            let api_key = "123";
            let mock_response = json!(
            {
                "cod": "404",
                "message": "city not found"
            });

            let mut mock_server = mockito::Server::new();
            let mock_endpoint = mock_server
                .mock("GET", "/data/2.5/weather")
                .match_query(mockito::Matcher::UrlEncoded("q".into(), address.into()))
                .match_query(mockito::Matcher::UrlEncoded(
                    "units".into(),
                    "metric".into(),
                ))
                .match_query(mockito::Matcher::UrlEncoded("appid".into(), api_key.into()))
                .with_status(404)
                .with_body(mock_response.to_string())
                .create();

            let url = mock_server.url();
            let client = Client::new();
            let api = OpenWeatherApiService::new(
                client,
                url.to_string() + "/data/2.5/weather",
                api_key.to_string(),
            )
            .unwrap();

            let result: WeatherApiError = api
                .get_weather_data(address, &None)
                .await
                .unwrap_err()
                .downcast()
                .unwrap();

            mock_endpoint.assert();
            assert!(matches!(result, WeatherApiError::Server(_)));
        }
    }
}
