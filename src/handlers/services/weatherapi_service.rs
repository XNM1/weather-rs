use dateparser::parse as parse_datetime_from_str;
use narrate::Result;
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

        let response_body = &response.text().await.map_err(WeatherApiError::Request)?;

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
