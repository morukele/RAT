use crate::common;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Agent {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub last_seen_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
    pub command: String,
    pub args: Json<Vec<String>>,
    pub output: Option<String>,
    pub agent_id: Uuid,
}

impl From<Job> for common::Job {
    fn from(val: Job) -> Self {
        common::Job {
            id: val.id,
            created_at: val.created_at,
            executed_at: val.executed_at,
            command: val.command,
            args: val.args.0,
            output: val.output,
            agent_id: val.agent_id,
        }
    }
}
