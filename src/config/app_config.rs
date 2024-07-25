use anyhow::Result;
use config::{Config, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
    pub endpoint_url: String,
    pub path_style: bool,
}

impl AppConfig {
    pub fn load() -> Result<Self> {
        let config = Config::builder()
            .add_source(File::with_name("config").required(false))
            .add_source(Environment::with_prefix("RS3"))
            .build()?;

        Ok(config.try_deserialize()?)
    }
}
