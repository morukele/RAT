use crate::api::Client;
use crate::{config, error};
use common::api;

impl Client {
    pub fn get_job_results(&self, job_id: uuid::Uuid) -> Result<Option<api::Job>, error::Error> {
        let get_job_result_route = format!("{}/api/jobs/{}/result", config::SERVER_URL, job_id);

        let res = self.http_client.get(get_job_result_route).send()?;
        let api_res: api::Response<api::Job> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(error::Error::Internal(err.message));
        }

        Ok(api_res.data)
    }
}
