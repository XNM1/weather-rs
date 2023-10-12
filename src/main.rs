mod cli_parser;

use clap::Parser;
use cli_parser::WeatherCLI;

fn main() {
    let weather_cli = WeatherCLI::parse();
    println!("Hello, world!");
}
