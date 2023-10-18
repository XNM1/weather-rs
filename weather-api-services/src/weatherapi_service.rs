use anyhow::Result;
use dateparser::parse as parse_datetime_from_str;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;

use super::{
    models::weatherapi_model::{WeatherApiData, WeatherApiErrorData, WeatherApiHistoryData},
    *,
};

#[derive(Debug)]
pub struct WeatherApiService {
    url: String,
    api_key: String,
    client: Client,
}

impl WeatherApiService {
    pub fn new(client: Client, mut url: String, api_key: String) -> Result<Self> {
        if url.is_empty() || api_key.is_empty() {
            return Err(WeatherApiError::Creation.into());
        }

        if url.ends_with('/') {
            url.pop();
        }

        Ok(WeatherApiService {
            client,
            url,
            api_key,
        })
    }

    #[allow(dead_code)]
    pub fn get_url(&self) -> &str {
        &self.url
    }
}

#[async_trait]
impl WeatherApi for WeatherApiService {
    async fn get_weather_data(&self, address: &str, date: &Option<String>) -> Result<WeatherData> {
        let mut params = HashMap::new();

        params.insert("q", address.to_owned());
        params.insert("key", self.api_key.to_owned());
        if let Some(date) = date {
            let timestamp = parse_datetime_from_str(date)
                .map_err(|_| DateTimeError::Parse)?
                .timestamp();
            params.insert("unixdt", timestamp.to_string());
        }

        let client = &self.client;
        let url = match date {
            Some(_) => format!("{}/history.json", &self.url),
            None => format!("{}/current.json", &self.url),
        };

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
            let weather_data = match date {
                Some(_) => serde_json::from_str::<WeatherApiHistoryData>(response_body)
                    .map_err(WeatherDataError::JsonParse)?
                    .into(),
                None => serde_json::from_str::<WeatherApiData>(response_body)
                    .map_err(WeatherDataError::JsonParse)?
                    .into(),
            };

            Ok(weather_data)
        } else {
            let weather_error_data: WeatherApiErrorData =
                serde_json::from_str(response_body).map_err(WeatherDataError::JsonParse)?;

            Err(WeatherApiError::Server(weather_error_data.error.message).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    mod tests_weatherapi_struct {
        use super::*;

        #[rstest]
        #[case(
            "https://api.weatherapi.com/v1/",
            "my_weather_api_key",
            "https://api.weatherapi.com/v1"
        )]
        #[case("https://example.com", "my_example_api_key", "https://example.com")]
        fn test_weather_api_creation(
            #[case] url: &str,
            #[case] api_key: &str,
            #[case] expected_url: &str,
        ) {
            let client = Client::new();
            let api = WeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

            assert_eq!(api.url, expected_url);
            assert_eq!(api.api_key, api_key);
        }

        #[rstest]
        #[case(
            "https://api.weatherapi.com/v1",
            "my_weather_api_key",
            "https://api.weatherapi.com/v1"
        )]
        fn test_get_url_method(
            #[case] url: &str,
            #[case] api_key: &str,
            #[case] expected_url: &str,
        ) {
            let client = Client::new();
            let api = WeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

            assert_eq!(api.get_url(), expected_url);
        }

        #[rstest]
        #[case("", "")]
        #[case("", "some key")]
        #[case("some url", "")]
        fn test_weather_api_with_empty_url_and_api_key(#[case] url: &str, #[case] api_key: &str) {
            let client = Client::new();
            let api = WeatherApiService::new(client, url.to_string(), api_key.to_string())
                .unwrap_err()
                .downcast()
                .unwrap();

            assert!(matches!(api, WeatherApiError::Creation));
        }
    }

    mod tests_get_weather_data {
        use super::*;
        use float_cmp::approx_eq;
        use serde_json::json;

        #[allow(clippy::too_many_arguments)]
        fn mock_weather_api_server(
            address: &str,
            temp: f32,
            humidity: u8,
            pressure: u16,
            wind_speed: f32,
            visibility: u16,
            description: &str,
            api_key: &str,
        ) -> (mockito::ServerGuard, mockito::Mock) {
            let mock_response = serde_json::json!({
                "current": {
                    "temp_c": temp,
                    "condition": {
                        "text": description
                    },
                    "wind_kph": wind_speed * 3.6,
                    "pressure_mb": pressure as f32,
                    "humidity": humidity,
                    "vis_km": visibility as f32 / 1000.0
                }
            });
            let mut mock_server = mockito::Server::new();

            let mock_endpoint = mock_server
                .mock("GET", "/current.json")
                .match_query(mockito::Matcher::UrlEncoded("q".into(), address.into()))
                .match_query(mockito::Matcher::UrlEncoded("key".into(), api_key.into()))
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
            let (mock_server, mock_endpoint) = mock_weather_api_server(
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
            let api = WeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

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
        fn mock_weather_api_history_server(
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
            let mock_response = serde_json::json!({
                "forecast": {
                    "forecastday": [
                        {
                            "hour": [
                                {
                                    "temp_c": temp,
                                    "condition": {
                                        "text": description
                                    },
                                    "wind_kph": wind_speed * 3.6,
                                    "pressure_mb": pressure as f32,
                                    "humidity": humidity,
                                    "vis_km": visibility as f32 / 1000.0
                                },
                            ]
                        },
                    ]
                }
            });
            let mut mock_server = mockito::Server::new();

            let mock_endpoint = mock_server
                .mock("GET", "/history.json")
                .match_query(mockito::Matcher::UrlEncoded("q".into(), address.into()))
                .match_query(mockito::Matcher::UrlEncoded("key".into(), api_key.into()))
                .match_query(mockito::Matcher::UrlEncoded("unixdt".into(), date.into()))
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
            let (mock_server, mock_endpoint) = mock_weather_api_history_server(
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
            let api = WeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

            let result = api
                .get_weather_data(address, &Some(date.to_owned()))
                .await
                .unwrap();

            mock_endpoint.assert();
            assert_eq!(result.temp, temp);
            assert_eq!(result.humidity, humidity);
            assert_eq!(result.pressure, pressure);
            assert!(approx_eq!(f32, result.wind_speed, wind_speed, ulps = 2));
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
            let (mock_server, _) = mock_weather_api_history_server(
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
            let api = WeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

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
            let api = WeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

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
                .mock("GET", "/current.json")
                .match_query(mockito::Matcher::UrlEncoded("q".into(), address.into()))
                .match_query(mockito::Matcher::UrlEncoded("key".into(), api_key.into()))
                .with_status(200)
                .with_body("invalid json")
                .create();

            let url = mock_server.url();
            let client = Client::new();
            let api = WeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

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
        async fn test_get_weather_data_with_date_json_parse_error() {
            let address = "SomeCity";
            let api_key = "123";
            let date = "2023-10-17 00:00";

            let mut mock_server = mockito::Server::new();
            let mock_endpoint = mock_server
                .mock("GET", "/history.json")
                .match_query(mockito::Matcher::UrlEncoded("q".into(), address.into()))
                .match_query(mockito::Matcher::UrlEncoded("key".into(), api_key.into()))
                .match_query(mockito::Matcher::UrlEncoded(
                    "unixdt".into(),
                    parse_datetime_from_str(date)
                        .unwrap()
                        .timestamp()
                        .to_string(),
                ))
                .with_status(200)
                .with_body("invalid json")
                .create();

            let url = mock_server.url();
            let client = Client::new();
            let api = WeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

            let result: WeatherDataError = api
                .get_weather_data(address, &Some(date.to_owned()))
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
                "error": {
                    "code": 1006,
                    "message": "No matching location found."
                }
            });

            let mut mock_server = mockito::Server::new();
            let mock_endpoint = mock_server
                .mock("GET", "/current.json")
                .match_query(mockito::Matcher::UrlEncoded("q".into(), address.into()))
                .match_query(mockito::Matcher::UrlEncoded("key".into(), api_key.into()))
                .with_status(404)
                .with_body(mock_response.to_string())
                .create();

            let url = mock_server.url();
            let client = Client::new();
            let api = WeatherApiService::new(client, url.to_string(), api_key.to_string()).unwrap();

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
