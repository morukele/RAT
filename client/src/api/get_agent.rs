use crate::{api::Client, config, error};
use common::entities;
use uuid::Uuid;

impl Client {
    pub fn get_agent(&self, agent_id: Uuid) -> Result<entities::Agent, error::Error> {
        let get_agent_route = format!("{}/api/agents/{}", config::SERVER_URL, agent_id);

        let res = self.http_client.get(get_agent_route).send()?;
        let api_res: entities::Response<entities::Agent> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(error::Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap())
    }
}
