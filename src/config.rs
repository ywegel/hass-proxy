use serde::Deserialize;

// TODO: maybe migrate to "clap"

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub address: String,
    pub port: u16,
    pub database_url: String,
}

pub fn load_config() -> Config {
    let env_path = dotenvy::dotenv().expect(".env variable should exist in working directory");
    tracing::info!("Loaded .env successfully from {}", env_path.display());

    let config = envy::from_env::<Config>().expect("Failed to deserialize env variables");
    tracing::debug!("Config: {:#?}", config);

    config
}