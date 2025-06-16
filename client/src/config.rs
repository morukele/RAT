use crate::error;

pub const SERVER_URL: &str = "http://localhost:8000";
pub const IDENTITY_PRIVATE_KEY: &str = "wToLgDfjCxFijRA+YKi6T9j7bTc/4grwoTRJZJs5DU8=";

#[derive(Debug)]
pub struct Config {
    pub identity_public_key: ed25519_dalek::PublicKey,
    pub identity_private_key: ed25519_dalek::SecretKey,
}

impl Config {
    pub fn load() -> Result<Config, error::Error> {
        let private_key_bytes = base64::decode(IDENTITY_PRIVATE_KEY)
            .map_err(|e| error::Error::Internal(e.to_string()))?;

        let identity_private_key = ed25519_dalek::SecretKey::from_bytes(&private_key_bytes)
            .map_err(|e| error::Error::Internal(e.to_string()))?;
        let identity_public_key: ed25519_dalek::PublicKey = (&identity_private_key).into();

        Ok(Config {
            identity_public_key,
            identity_private_key,
        })
    }
}
