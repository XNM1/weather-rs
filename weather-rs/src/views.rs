use convert_case::{Case, Casing};
use narrate::anyhow::Result;
use narrate::colored::Colorize;
use prettytable::{row, Table};

use weather_api_services::models::WeatherData;

pub fn table_terminal_view(weather_data: WeatherData) {
    let mut table = Table::new();
    table.add_row(row!["Name", "Value"]);
    table.add_row(row![
        "Description",
        weather_data.description.to_case(Case::Title).green()
    ]);
    table.add_row(row![
        "Temperature",
        format!("{:.2} °C", weather_data.temp).yellow()
    ]);
    table.add_row(row![
        "Humidity",
        format!("{} %", weather_data.humidity).blue()
    ]);
    table.add_row(row![
        "Pressure",
        format!("{} hPa", weather_data.pressure).green()
    ]);
    table.add_row(row![
        "Wind speed",
        format!("{:.2} m/sec", weather_data.wind_speed).cyan()
    ]);
    table.add_row(row![
        "Visibility",
        format!("{} m", weather_data.visibility).magenta()
    ]);

    table.printstd();
}

pub fn json_terminal_view(weather_data: WeatherData) -> Result<()> {
    println!("{}", serde_json::to_string(&weather_data)?);

    Ok(())
}
