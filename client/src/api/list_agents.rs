use crate::api::Client;
use crate::error::Error;
use crate::{config, error};
use common::api;

impl Client {
    pub fn list_agents(&self) -> Result<Vec<api::Agent>, error::Error> {
        let get_agent_route = format!("{}/api/agents", config::SERVER_URL);

        let res = self.http_client.get(get_agent_route).send()?;
        let api_res: api::Response<api::AgentList> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap().agents)
    }
}
