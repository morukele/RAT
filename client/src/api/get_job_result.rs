use crate::api::Client;
use crate::{config, error};
use server::common;

impl Client {
    pub fn get_job_results(&self, job_id: uuid::Uuid) -> Result<Option<String>, error::Error> {
        let get_job_result_route = format!("{}/api/jobs/{}/result", config::SERVER_URL, job_id);

        let res = self.http_client.get(get_job_result_route).send()?;
        let api_res: common::Response<common::Job> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(error::Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap().output)
    }
}
