use uuid::Uuid;

// This should be secret in your config, do not expose this in your public git.
// Consider using include! macro to inject the string at compile time.
pub const CLIENT_IDENTITY_PUBLIC_KEY: &str = "xQ6gstFLtTbDC06LDb5dAQap+fXVG45BnRZj0L5th+M=";
pub const SERVER_URL: &str = "http://127.0.0.1:8000";
pub const AGENT_ID_FILE: &str = "0xSpada";

pub const X25519_PRIVATE_KEY_SIZE: usize = 32;
pub const X25519_PUBLIC_KEY_SIZE: usize = 32;

pub const XCHACHA20_POLY1305_NONCE_SIZE: usize = 24;
pub const XCHACHA20_POLY1305_KEY_SIZE: usize = 32;

pub const ED25519_PUBLIC_KEY_SIZE: usize = 32;
pub const ED25519_PRIVATE_KEY_SIZE: usize = 32;
pub const ED25519_SIGNATURE_SIZE: usize = 64;

#[derive(Debug)]
pub struct Config {
    pub agent_id: Uuid,
    pub identity_public_key: ed25519_dalek::PublicKey,
    pub identity_private_key: ed25519_dalek::SecretKey,
    pub public_prekey: [u8; 32],
    pub private_prekey: [u8; 32],
    pub client_identity_public_key: ed25519_dalek::PublicKey,
}
