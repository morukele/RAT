use crate::error::Error;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub database_url: String,
}

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_PORT: &str = "ENV_PORT";
const ENV_HOST: &str = "ENV_HOST";
const DEFAULT_PORT: u16 = 8000;

impl Config {
    pub fn load() -> Result<Config, Error> {
        dotenv::dotenv().ok();
        let port = std::env::var(ENV_PORT)
            .ok()
            .map_or(Ok(DEFAULT_PORT), |env_val| env_val.parse::<u16>())
            .map_err(|_| Error::Internal("Unable to parse port".to_string()))?;
        let host = std::env::var(ENV_HOST).map_err(|_| env_not_found(ENV_HOST))?;
        let database_url =
            std::env::var(ENV_DATABASE_URL).map_err(|_| env_not_found(ENV_DATABASE_URL))?;

        Ok(Config {
            port,
            host,
            database_url,
        })
    }
}

fn env_not_found(env: &str) -> Error {
    Error::NotFound(format!("config: {} env var not found", env))
}
