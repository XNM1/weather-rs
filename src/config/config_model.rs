use serde::{Deserialize, Serialize};

use crate::providers::Provider;

#[derive(Serialize, Deserialize, Default, Debug)]
struct MainConfig {
    main_provider: Provider,
    open_weather: Option<ProviderConfig>,
    weather_api: Option<ProviderConfig>,
    accu_weather: Option<ProviderConfig>,
    aeris_weather: Option<ProviderConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ProviderConfig {
    url: String,
    api_key: String,
}
