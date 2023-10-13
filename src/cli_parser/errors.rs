use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ProviderError {
    #[error("Weather provider not found; use the command 'weather-rs provider-list' to get a list of all available providers")]
    ProviderNotFound,
}
