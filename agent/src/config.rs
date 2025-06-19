use serde::{Deserialize, Serialize};
use uuid::Uuid;
use x25519_dalek::{X25519_BASEPOINT_BYTES, x25519};

use crate::error::{self, Error};

// This should be secret in your config, do not expose this in your public git.
// Consider using include! macro to inject the string at compile time.
pub const CLIENT_IDENTITY_PUBLIC_KEY: &str = "xQ6gstFLtTbDC06LDb5dAQap+fXVG45BnRZj0L5th+M=";
pub const SERVER_URL: &str = "http://127.0.0.1:8000";
pub const AGENT_ID_FILE: &str = "0xSpada";

#[derive(Debug)]
pub struct Config {
    pub agent_id: Uuid,
    pub identity_public_key: ed25519_dalek::PublicKey,
    pub identity_private_key: ed25519_dalek::SecretKey,
    pub public_prekey: [u8; 32],
    pub private_prekey: [u8; 32],
    pub client_identity_public_key: ed25519_dalek::PublicKey,
}

impl TryFrom<SerializedConfig> for Config {
    type Error = Error;

    fn try_from(conf: SerializedConfig) -> Result<Config, Self::Error> {
        let agent_id = conf.agent_id;

        let identity_private_key = ed25519_dalek::SecretKey::from_bytes(&conf.identity_private_key)
            .map_err(|e| error::Error::Internal(e.to_string()))?;
        let identity_public_key: ed25519_dalek::PublicKey = (&identity_private_key).into();

        let private_prekey = conf.private_prekey;
        let public_prekey = x25519(private_prekey.clone(), X25519_BASEPOINT_BYTES);

        let client_public_key_bytes = base64::decode(CLIENT_IDENTITY_PUBLIC_KEY)
            .map_err(|e| error::Error::Internal(e.to_string()))?;
        let client_identity_public_key =
            ed25519_dalek::PublicKey::from_bytes(&client_public_key_bytes)
                .map_err(|e| error::Error::Internal(e.to_string()))?;

        Ok(Config {
            agent_id,
            identity_public_key,
            identity_private_key,
            public_prekey,
            private_prekey,
            client_identity_public_key,
        })
    }
}

impl Into<SerializedConfig> for &Config {
    fn into(self) -> SerializedConfig {
        SerializedConfig {
            agent_id: self.agent_id,
            identity_private_key: self.identity_private_key.to_bytes(),
            private_prekey: self.private_prekey,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SerializedConfig {
    pub agent_id: Uuid,
    pub identity_private_key: [u8; ed25519_dalek::SECRET_KEY_LENGTH],
    pub private_prekey: [u8; 32],
}
