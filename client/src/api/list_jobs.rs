use crate::api::Client;
use crate::error::Error;
use crate::{config, error};
use server::common;

impl Client {
    pub fn list_jobs(&self) -> Result<Vec<common::Job>, error::Error> {
        let get_jobs_route = format!("{}/api/jobs", config::SERVER_URL);

        let res = self.http_client.get(get_jobs_route).send()?;
        let api_res: common::Response<common::JobList> = res.json()?;

        if let Some(err) = api_res.error {
            return Err(Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap().jobs)
    }
}
