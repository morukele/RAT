use crate::entities::Agent;
use crate::error;
use crate::error::Error;
use crate::repository::Repository;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

impl Repository {
    pub async fn create_agent(
        &self,
        db: &Pool<Postgres>,
        agent: &Agent,
    ) -> Result<(), error::Error> {
        const QUERY: &str = "INSERT INTO agents\
            (id, ip_addr, name, username, created_at, last_seen_at)\
            VALUES ($1, $2, $3, $4, $5, $6)";

        match sqlx::query(QUERY)
            .bind(agent.id)
            .bind(&agent.ip_addr)
            .bind(&agent.name)
            .bind(&agent.username)
            .bind(agent.created_at)
            .bind(agent.last_seen_at)
            .execute(db)
            .await
        {
            Err(err) => {
                log::error!("create_agent: Inserting agent: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }

    pub async fn update_agent(
        &self,
        db: &Pool<Postgres>,
        agent: &Agent,
    ) -> Result<(), error::Error> {
        const QUERY: &str = "UPDATE agents
            SET last_seen_at = $1
            WHERE id = $2";

        match sqlx::query(QUERY)
            .bind(agent.last_seen_at)
            .bind(agent.id)
            .execute(db)
            .await
        {
            Err(err) => {
                log::error!("update_event: updating agent: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }

    pub async fn find_all_agents(&self, db: &Pool<Postgres>) -> Result<Vec<Agent>, error::Error> {
        const QUERY: &str = "SELECT * FROM agents ORDER BY created_at";

        match sqlx::query_as::<_, Agent>(QUERY).fetch_all(db).await {
            Err(err) => {
                log::error!("find_all_agents: finding agents: {}", &err);
                Err(err.into())
            }
            Ok(res) => Ok(res),
        }
    }

    pub async fn find_agent_by_id(
        &self,
        db: &Pool<Postgres>,
        agent_id: Uuid,
    ) -> Result<Agent, error::Error> {
        const QUERY: &str = "SELECT * FROM agents WHERE id = $1";

        match sqlx::query_as::<_, Agent>(QUERY)
            .bind(agent_id)
            .fetch_optional(db)
            .await
        {
            Err(err) => {
                log::error!("find_agent_by_id: finding agent: {}", &err);
                Err(err.into())
            }
            Ok(None) => Err(Error::NotFound("Agent not found".to_string())),
            Ok(Some(res)) => Ok(res),
        }
    }
}
