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

impl Into<common::Job> for Job {
    fn into(self) -> common::Job {
        common::Job {
            id: self.id,
            created_at: self.created_at,
            executed_at: self.executed_at,
            command: self.command,
            args: self.args.0,
            output: self.output,
            agent_id: self.agent_id,
        }
    }
}
