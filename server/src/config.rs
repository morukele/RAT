use crate::error::{self, Error};

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub host: String,
    pub database_url: String,
    pub client_identity_public_key: ed25519_dalek::PublicKey,
}

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_PORT: &str = "ENV_PORT";
const ENV_HOST: &str = "ENV_HOST";
const DEFAULT_PORT: u16 = 8000;
const ENV_CLIENT_IDENTITY_PUBLIC_KEY: &str = "CLIENT_IDENTITY_PUBLIC_KEY";

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

        let client_identity_key_str = std::env::var(ENV_CLIENT_IDENTITY_PUBLIC_KEY)
            .ok()
            .unwrap_or(String::new());
        let client_identity_public_key_bytes = base64::decode(&client_identity_key_str)
            .map_err(|e| error::Error::Internal(e.to_string()))?;

        let client_identity_public_key =
            ed25519_dalek::PublicKey::from_bytes(&client_identity_public_key_bytes)
                .map_err(|e| error::Error::Internal(e.to_string()))?;

        Ok(Config {
            port,
            host,
            database_url,
            client_identity_public_key,
        })
    }
}

fn env_not_found(env: &str) -> Error {
    Error::NotFound(format!("config: {} env var not found", env))
}
