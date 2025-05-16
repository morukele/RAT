use crate::common::AgentRegistered;
use crate::service::Service;
use crate::{entities, error};
use chrono::Utc;
use local_ip_address::local_ip;
use std::net::{IpAddr, Ipv4Addr};
use uuid::Uuid;

impl Service {
    pub async fn list_agents(&self) -> Result<Vec<entities::Agent>, error::Error> {
        self.repo.find_all_agents(&self.db).await
    }

    pub async fn register_agent(&self) -> Result<AgentRegistered, error::Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        let ip_addr = local_ip()
            .unwrap_or(IpAddr::from(Ipv4Addr::new(0, 0, 0, 0)))
            .to_string();
        let name = whoami::realname();
        let username = whoami::username();
        let agent = entities::Agent {
            id,
            ip_addr,
            name,
            username,
            created_at,
            last_seen_at: created_at,
        };

        self.repo.create_agent(&self.db, &agent).await?;

        Ok(AgentRegistered { id })
    }
}
