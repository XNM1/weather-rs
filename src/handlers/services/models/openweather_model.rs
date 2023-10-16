use serde::Deserialize;

#[derive(Deserialize)]
pub struct OpenWeatherData {
    pub main: WeatherMain,
    pub weather: Vec<Weather>,
    pub visibility: u16,
    pub wind: Wind,
}

#[derive(Deserialize)]
pub struct WeatherMain {
    pub temp: f32,
    pub humidity: u8,
    pub pressure: u16,
}

#[derive(Deserialize)]
pub struct Weather {
    pub description: String,
}

#[derive(Deserialize)]
pub struct Wind {
    pub speed: f32,
}
