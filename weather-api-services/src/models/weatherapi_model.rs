use serde::Deserialize;

// Weather Data Section

/// Represents weather data from the Weather API.
#[derive(Deserialize)]
pub struct WeatherApiData {
    pub current: WeatherCurrent,
}

/// Represents current weather data from the Weather API.
#[derive(Deserialize)]
pub struct WeatherCurrent {
    pub temp_c: f32,
    pub condition: WeatherCondition,
    pub wind_kph: f32,
    pub pressure_mb: f32,
    pub humidity: u8,
    pub vis_km: f32,
}

/// Represents weather condition from the Weather API.
#[derive(Deserialize)]
pub struct WeatherCondition {
    pub text: String,
}

// End of Weather Data Section

//--------------------------------

// Weather History Data Section

/// Represents weather data for a specific date in history from the Weather API.
#[derive(Deserialize)]
pub struct WeatherApiHistoryData {
    pub forecast: HistoryForecast,
}

/// Represents weather forecast for a specific date in history data from the Weather API.
#[derive(Deserialize)]
pub struct HistoryForecast {
    pub forecastday: Vec<HistoryForecastDay>,
}

/// Represents a day's weather data in a historical forecast.
#[derive(Deserialize)]
pub struct HistoryForecastDay {
    pub hour: Vec<WeatherCurrent>,
}

// End of Weather History Data Secction

//---------------------------------------

// Weather Server Error Section

/// Represents error data from the Weather API.
#[derive(Deserialize)]
pub struct WeatherApiErrorData {
    pub error: DataError,
}

/// Represents an error message from the Weather API.
#[derive(Deserialize)]
pub struct DataError {
    pub code: u16,
    pub message: String,
}

// End of Weather Server Error Section
