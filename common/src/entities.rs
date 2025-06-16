use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::crypto;

#[derive(Serialize, Debug, Deserialize)]
pub struct Response<T: Serialize> {
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        Response {
            data: Some(data),
            error: None,
        }
    }

    pub fn err(err: Error) -> Response<()> {
        Response::<()> {
            data: None,
            error: Some(err),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Error {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AgentRegistered {
    pub id: Uuid,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct CreateJob {
    pub id: Uuid,
    pub agent_id: Uuid,
    pub encrypted_job: Vec<u8>,
    pub ephemeral_public_key: [u8; crypto::X25519_PRIVATE_KEY_SIZE],
    pub nonce: [u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE],
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow)]
pub struct Job {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
    pub command: String,
    pub args: Vec<String>,
    pub output: Option<String>,
    pub agent_id: Uuid,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentJob {
    pub id: Uuid,
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobList {
    pub jobs: Vec<Job>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentList {
    pub agents: Vec<Agent>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateJobResult {
    pub job_id: Uuid,
    pub output: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Agent {
    pub id: Uuid,
    pub ip_addr: String,
    pub name: String,
    pub username: String,
    pub identity_public_key: [u8; crypto::ED25519_PUBLIC_KEY_SIZE],
    pub public_prekey: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
    pub public_prekey_signature: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentCreationDetail {
    pub ip_addr: String,
    pub name: String,
    pub username: String,
    pub identity_public_key: [u8; crypto::ED25519_PUBLIC_KEY_SIZE],
    pub public_prekey: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
    pub public_prekey_signature: Vec<u8>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobPayload {
    pub command: String,
    pub args: Vec<String>,
    pub result_ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
}
