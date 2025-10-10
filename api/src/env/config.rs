use serde::Deserialize;
use config::{Config, ConfigError, File, Environment};

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        println!("Current dir: {:?}", std::env::current_dir().unwrap());
        let settings = Config::builder()
            .add_source(File::with_name("api/config/application.yml"))
            .add_source(File::with_name("api/config/dev.yml").required(false))
            .add_source(File::with_name("api/config/production.yml").required(false))
            .add_source(Environment::with_prefix("APP"))
            .build()?;

        settings.try_deserialize()
    }
}
