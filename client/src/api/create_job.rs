use crate::api::Client;
use crate::{config, error};
use server::common;
use uuid::Uuid;

impl Client {
    pub fn create_job(&self, input: common::CreateJob) -> Result<Uuid, error::Error> {
        let post_job_route = format!("{}/api/jobs", config::SERVER_URL);

        let res = self.http_client.post(post_job_route).json(&input).send()?;
        let api_res: common::Response<common::Job> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(error::Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap().id)
    }
}
