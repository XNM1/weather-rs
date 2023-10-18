use serde::Deserialize;

// Weather Data Section

/// Represents weather data from the OpenWeather API.
#[derive(Deserialize)]
pub struct OpenWeatherData {
    pub main: WeatherMain,
    pub weather: Vec<Weather>,
    pub visibility: u16,
    pub wind: Wind,
}

/// Represents main weather parameters from OpenWeather data.
#[derive(Deserialize)]
pub struct WeatherMain {
    pub temp: f32,
    pub humidity: u8,
    pub pressure: u16,
}

/// Represents weather conditions from OpenWeather data.
#[derive(Deserialize)]
pub struct Weather {
    pub description: String,
}

/// Represents wind data from OpenWeather data.
#[derive(Deserialize)]
pub struct Wind {
    pub speed: f32,
}

// End of Weather Data Section

//--------------------------------

// Weather Server Error Section

/// Represents error data from the OpenWeather API server.
#[derive(Deserialize)]
pub struct OpenWeatherErrorData {
    pub cod: String,
    pub message: String,
}

// End of Weather Server Error Section
