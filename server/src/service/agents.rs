use crate::common::AgentRegistered;
use crate::entities::{Agent, AgentCreationDetail};
use crate::service::Service;
use crate::{entities, error};
use chrono::Utc;
use uuid::Uuid;

impl Service {
    pub async fn list_agents(&self) -> Result<Vec<entities::Agent>, error::Error> {
        self.repo.find_all_agents(&self.db).await
    }

    pub async fn find_agent(&self, agent_id: Uuid) -> Result<Agent, error::Error> {
        self.repo.find_agent_by_id(&self.db, agent_id).await
    }

    pub async fn register_agent(
        &self,
        agent_details: AgentCreationDetail,
    ) -> Result<AgentRegistered, error::Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        let agent = entities::Agent {
            id,
            created_at,
            last_seen_at: created_at,
            ip_addr: agent_details.ip_addr,
            name: agent_details.name,
            username: agent_details.username,
            identity_public_key: agent_details.identity_public_key,
            public_prekey: agent_details.public_prekey,
            public_prekey_signature: agent_details.public_prekey_signature,
        };

        self.repo.create_agent(&self.db, &agent).await?;

        Ok(AgentRegistered { id })
    }
}
