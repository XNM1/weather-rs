use color_eyre::{owo_colors::OwoColorize, Result};
use convert_case::{Case, Casing};
use prettytable::{row, Table};

use super::services::models::WeatherData;

pub fn table_terminal_view(weather_data: WeatherData) {
    let mut table = Table::new();
    table.add_row(row!["Name", "Value"]);
    table.add_row(row![
        "Description",
        (weather_data.description.to_case(Case::Title)).green()
    ]);
    table.add_row(row![
        "Temperature",
        (weather_data.temp.to_string() + " °C").yellow()
    ]);
    table.add_row(row![
        "Humidity",
        (weather_data.humidity.to_string() + " %").blue()
    ]);
    table.add_row(row![
        "Pressure",
        (weather_data.pressure.to_string() + " hPa").green()
    ]);
    table.add_row(row![
        "Wind speed",
        (weather_data.wind_speed.to_string() + " m/sec").cyan()
    ]);
    table.add_row(row![
        "Visibility",
        (weather_data.visibility.to_string() + " m").magenta()
    ]);

    table.printstd();
}

pub fn json_terminal_view(weather_data: WeatherData) -> Result<()> {
    println!("{}", serde_json::to_string(&weather_data)?);

    Ok(())
}
