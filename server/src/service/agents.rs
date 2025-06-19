use crate::entities::{self, Agent};
use crate::error;
use crate::service::Service;
use chrono::Utc;
use common::{api, crypto};
use uuid::Uuid;

impl Service {
    pub async fn list_agents(&self) -> Result<api::AgentList, error::Error> {
        let agents: Vec<api::Agent> = self
            .repo
            .find_all_agents(&self.db)
            .await?
            .into_iter()
            .map(|a| {
                let a: api::Agent = a.into();
                a
            })
            .collect();

        Ok(api::AgentList { agents })
    }

    pub async fn find_agent(&self, agent_id: Uuid) -> Result<entities::Agent, error::Error> {
        self.repo.find_agent_by_id(&self.db, agent_id).await
    }

    pub async fn register_agent(
        &self,
        input: api::RegisterAgent,
    ) -> Result<api::AgentRegistered, error::Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();

        // Verify input
        if input.public_prekey_signature.len() != crypto::ED25519_SIGNATURE_SIZE {
            return Err(error::Error::InvalidArgument(
                "Agent's public prekey Signature size is not valid".to_string(),
            ));
        }

        let agent_identity_public_key =
            ed25519_dalek::PublicKey::from_bytes(&input.identity_public_key)
                .map_err(|e| error::Error::Internal(e.to_string()))?;
        let signature = ed25519_dalek::Signature::try_from(&input.public_prekey_signature[0..64])
            .map_err(|e| error::Error::Internal(e.to_string()))?;

        log::debug!("register_agent: input is valid");

        if agent_identity_public_key
            .verify_strict(&input.public_prekey, &signature)
            .is_err()
        {
            return Err(error::Error::InvalidArgument(
                "Signature is not valid".to_string(),
            ));
        }

        let agent = Agent {
            id,
            created_at,
            last_seen_at: created_at,
            ip_addr: input.ip_addr,
            name: input.name,
            username: input.username,
            identity_public_key: input.identity_public_key.to_vec(),
            public_prekey: input.public_prekey.to_vec(),
            public_prekey_signature: input.public_prekey_signature.to_vec(),
        };

        self.repo.create_agent(&self.db, &agent).await?;

        Ok(api::AgentRegistered { id })
    }
}
