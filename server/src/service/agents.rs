use crate::common::AgentRegistered;
use crate::service::Service;
use crate::{entities, error};
use chrono::Utc;
use uuid::Uuid;

impl Service {
    pub async fn list_agents(&self) -> Result<Vec<entities::Agent>, error::Error> {
        self.repo.find_all_agents(&self.db).await
    }

    pub async fn register_agent(&self) -> Result<AgentRegistered, error::Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();

        let agent = entities::Agent {
            id,
            created_at,
            last_seen_at: created_at,
        };

        self.repo.create_agent(&self.db, &agent).await?;

        Ok(AgentRegistered { id })
    }
}
