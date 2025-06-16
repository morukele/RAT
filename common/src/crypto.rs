use uuid::Uuid;

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
