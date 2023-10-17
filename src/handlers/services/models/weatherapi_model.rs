use serde::Deserialize;

// Weather Data Section
#[derive(Deserialize)]
pub struct WeatherApiData {
    pub current: WeatherCurrent,
}

#[derive(Deserialize)]
pub struct WeatherCurrent {
    pub temp_c: f32,
    pub condition: WeatherCondition,
    pub wind_kph: f32,
    pub pressure_mb: f32,
    pub humidity: u8,
    pub vis_km: f32,
}

#[derive(Deserialize)]
pub struct WeatherCondition {
    pub text: String,
}
// End of Weather Data Section

// Weather History Data Section
#[derive(Deserialize)]
pub struct WeatherApiHistoryData {
    pub forecast: HistoryForecast,
}

#[derive(Deserialize)]
pub struct HistoryForecast {
    pub forecastday: Vec<HistoryForecastDay>,
}

#[derive(Deserialize)]
pub struct HistoryForecastDay {
    pub hour: Vec<WeatherCurrent>,
}
// End of Weather History Data Secction

// Weather Server Error Section
#[derive(Deserialize)]
pub struct WeatherApiErrorData {
    pub error: DataError,
}

#[derive(Deserialize)]
pub struct DataError {
    pub code: u16,
    pub message: String,
}
// End of Weather Server Error Section
