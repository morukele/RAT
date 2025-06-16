use crate::{api, error};
use server::common;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

pub fn run(api_client: &api::Client, agent_id: &str, command: &str) -> Result<(), error::Error> {
    let agent_id = Uuid::parse_str(agent_id)?;
    let sleep_for = Duration::from_millis(500);

    let input = common::CreateJob {
        agent_id,
        command: command.trim().to_string(),
    };
    let job_id = api_client.create_job(input)?;

    log::debug!("Job created: {}", job_id);

    loop {
        let job_output = api_client.get_job_results(job_id)?;
        log::debug!("job results: {:?}", job_output);
        if let Some(job_output) = job_output {
            println!("{}", job_output);
            break;
        }
        sleep(sleep_for);
    }

    Ok(())
}
