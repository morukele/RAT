use crate::entities::Agent;
use actix_web::FromRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
            error: Some(err.into()),
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
    pub agent_id: Uuid,
    pub command: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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
