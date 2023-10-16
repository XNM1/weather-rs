use serde::Deserialize;

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
