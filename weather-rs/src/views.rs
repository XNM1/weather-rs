use convert_case::{Case, Casing};
use narrate::anyhow::Result;
use narrate::colored::Colorize;
use prettytable::{row, Table};

use weather_api_services::models::WeatherData;

/// Renders weather data in a tabular format for display in the terminal.
///
/// This function takes weather data as input and displays it in a tabular format.
/// It creates a table with columns "Name" and "Value" to present the weather data attributes.
///
/// # Arguments
///
/// * `weather_data` - The `WeatherData` structure containing weather-related information to be displayed.
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

/// Renders weather data in JSON format for display in the terminal.
///
/// This function takes weather data as input, serializes it into JSON format, and prints it to the terminal.
///
/// # Arguments
///
/// * `weather_data` - The `WeatherData` structure containing weather-related information to be displayed in JSON format.
///
/// # Returns
///
/// A `Result` indicating success or an error when serializing the weather data into JSON format.
pub fn json_terminal_view(weather_data: WeatherData) -> Result<()> {
    println!("{}", serde_json::to_string(&weather_data)?);

    Ok(())
}
