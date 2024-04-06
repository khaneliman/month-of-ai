use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub front_end_url: String,
    pub azure_search: AzureSearchConfig,
    pub open_ai: OpenAiConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AzureSearchConfig {
    pub url: String,
    pub api_version: String,
    pub key: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OpenAiConfig {
    pub url: String,
    pub api_version: String,
    pub key: String,
    pub model: String,
}
