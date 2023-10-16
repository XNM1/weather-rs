use dateparser::parse as parse_datetime_from_str;
use narrate::Result;
use reqwest::Client;
use std::collections::HashMap;

use super::{
    models::weatherapi_model::{WeatherApiData, WeatherApiHistoryData},
    *,
};

#[derive(Debug)]
pub struct WeatherApiService {
    url: String,
    api_key: String,
    client: Client,
}

impl WeatherApiService {
    pub fn new(client: Client, url: String, api_key: String) -> Result<Self> {
        if url.is_empty() || api_key.is_empty() {
            return Err(WeatherApiError::CreationError.into());
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
                .map_err(|_| DateTimeError::ParseError)?
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
            .map_err(WeatherApiError::RequestError)?;

        let response_body = &response
            .text()
            .await
            .map_err(WeatherApiError::RequestError)?;

        // panic!("{}", response_body);

        let weather_data = match date {
            Some(_) => serde_json::from_str::<WeatherApiHistoryData>(response_body)
                .map_err(WeatherDataError::JsonParseError)?
                .into(),
            None => serde_json::from_str::<WeatherApiData>(response_body)
                .map_err(WeatherDataError::JsonParseError)?
                .into(),
        };

        Ok(weather_data)
    }
}
