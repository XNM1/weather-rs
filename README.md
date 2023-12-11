# weather-rs

🌦️ Fast and simple CLI tool for fetching weather data from different weather API providers 🌏

![Demo](showcase.gif)

## Description

Weather-rs is a lightweight command-line tool designed to quickly fetch weather data from various API providers. It offers a convenient way to check the weather conditions for a specific location, providing real-time and historical data.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Features](#features)
- [Supported Weather API Service Providers](#supported-weather-api-service-providers)
- [Contributing](#contributing)
- [Authors](#authors)
- [Alternatives](#alternatives)
- [License](#license)

## Installation

### From Release Section

You can download pre-compiled binaries from the [Release Section](https://github.com/XNM1/weather-rs/releases).

### Using Cargo Package Manager

1. Install Rust and Cargo from the official website: [Rust Installation](https://doc.rust-lang.org/cargo/getting-started/installation.html).
2. Run the following command to install and compile `weather-rs`:

   ```bash
   cargo install weather-rs --git 'https://github.com/XNM1/weather-rs'
   ```

3. Add the `.cargo/bin` directory to your PATH environment variable in your shell or use it directly from the `.cargo/bin` directory.

   - **Fish Shell:**
     ```fish
     set -gx PATH $HOME/.cargo/bin $PATH
     ```

   - **Bash or Zsh:**
     ```bash
     export PATH="$HOME/.cargo/bin:$PATH"
     ```
     
   You can use the following commands in the shell for one-time use. Alternatively, you can set them in the appropriate shell configuration file to ensure they persist even after a shell restart. You can now use Cargo binaries seamlessly from the command line without specifying the full path to the `.cargo/bin` directory.

### For Nix Package Manager and NixOS

1. Clone the project from GitHub using the following command:

   ```bash
   git clone https://github.com/XNM1/weather-rs
   ```

2. Navigate to the cloned project folder:

   ```bash
   cd weather-rs
   ```

3. Build the project using Nix:

   ```bash
   nix build
   ```

4. Install the project using Nix Package Manager:

   ```bash
   nix-env -i ./result
   ```

### From Source

1. Clone the project from GitHub using the following command:

   ```bash
   git clone 'https://github.com/XNM1/weather-rs'
   ```

2. Navigate to the cloned project folder and build the project with the following command:

   ```bash
   cargo build --release
   ```

3. You can find the executable binary in `weather-rs/target/release` called `weather-rs`.

## Usage

```plaintext
Fast and simple CLI tool for weather data fetching from different providers

Usage: weather-rs <COMMAND>

Commands:
  provider-list    Get a full list of supported providers
  configure        Configure a provider with the given credentials
  select-provider  Select an available provider
  get              Get weather information
  help             Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

1. Check available weather provider APIs using the command:

   ```bash
   weather-rs provider-list
   ```

2. Configure weather providers using the command:

   ```bash
   weather-rs configure <PROVIDER> <API_KEY> [-u <URL>]
   ```

   Example: 

   ```bash
   weather-rs configure 'open-weather' '<your api key>'
   ```

3. Select a provider using the command:

   ```bash
   weather-rs select-provider <PROVIDER>
   ```

   Example: 

   ```bash
   weather-rs select-provider 'weather-api'
   ```

4. Get information about weather data using the command:

   ```bash
   weather-rs get <ADDRESS> [-d <DATE>] [--json] [-p <PROVIDER>]
   ```

   Example: 

   ```bash
   weather-rs get 'London'
   ```

   Another example: 

   ```bash
   weather-rs get 'London' -d '2023-10-11' --json
   ```

## Configuration

The configuration file is located in the following directories:

- Linux: `~/.config/weather-rs/config.toml`
- MacOS: `~/Library/Preferences/weather-rs/config.toml`
- Windows: `%USERPROFILE%\AppData\Roaming\weather-rs\config\config.toml`

The configuration file is in TOML format and includes settings for services (URL and API key). For example:

```toml
[open_weather]
url = 'https://api.openweathermap.org/data/2.5/weather'
api_key = 'your_api_key_here'
```

You can also set the selected main weather data provider in the `selected_provider` parameter. Example:

```toml
selected_provider = 'OpenWeather'
```

Please note that the selected provider should be in PascalCase in configuration file and in kebab-case when set from the command line (e.g., `weather-rs select-provider 'open-weather'`).

## Features

🌟 Simple and minimal

🚀 Very fast

🛡️ Safe (written in safe Rust 🦀)

📋 Supports output in JSON format for your scripts and tabular format for a more visually pleasing display

🌍 Supports different weather API service providers

## Supported Weather API Service Providers

At the moment, the project supports two providers:

1. Open Weather API version 2: https://api.openweathermap.org/data/2.5/weather (provides current weather data).

2. Weather API version 1: https://api.weatherapi.com/v1 (provides current and historical weather data).

More providers may be added in the future.

## Contributing

Contributions are welcome! Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file for more information on contribution guidelines.

## Authors

- XNM: [\[GitHub Profile\]](https://github.com/XNM1) [\[LinkedIn Profile\]](https://www.linkedin.com/in/art-shv/)

## Alternatives

If you're interested in other weather-related CLI tools, consider checking out these alternatives:

- [wthrr](https://github.com/ttytm/wthrr-the-weathercrab) - Weather companion for the terminal.
- [wttr](https://github.com/chubin/wttr.in) - The right way to check the weather.
- [weather](https://github.com/genuinetools/weather) - Weather via the command line.
- [weather-api](https://github.com/robertoduessmann/weather-api) - A RESTful API to check the weather.

## License

This project is licensed under the MIT license. For more details, please refer to the [LICENSE](LICENSE) file. 📜
