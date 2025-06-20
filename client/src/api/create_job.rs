use crate::api::Client;
use crate::{config, error};
use common::api;
use uuid::Uuid;

impl Client {
    pub fn create_job(&self, input: api::CreateJob) -> Result<Uuid, error::Error> {
        let post_job_route = format!("{}/api/jobs", config::SERVER_URL);

        let res = self.http_client.post(post_job_route).json(&input).send()?;

        // Log the raw response body before decoding
        let raw_body = res.text()?;
        println!("Raw response body: {}", raw_body);

        // Now decode as JSON
        let api_res: api::Response<api::Job> = serde_json::from_str(&raw_body)
            .map_err(|e| error::Error::Internal(format!("Failed to decode JSON: {}", e)))?;

        if let Some(err) = api_res.error {
            return Err(error::Error::Internal(err.message));
        }

        Ok(api_res.data.unwrap().id)
    }
}
