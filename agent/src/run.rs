use crate::SERVER_URL;
use server::common;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;

pub fn run(api_client: &ureq::Agent, agent_id: Uuid) -> ! {
    let sleep_for = Duration::from_secs(1);
    let get_job_route = format!("{}/api/agents/{}/job", SERVER_URL, agent_id);
    let post_job_result_route = format!("{}/api/jobs/result", SERVER_URL);

    loop {
        let server_res = match api_client.get(get_job_route.as_str()).call() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error getting job from server: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        let api_res: common::Response<common::AgentJob> = match server_res.into_json() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error parsing JSON: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        log::debug!("api response successfully received: {:?}", api_res);

        let job = match api_res.data {
            Some(job) => job,
            None => {
                log::debug!("No job found. Trying again in {:?} seconds", sleep_for);
                sleep(sleep_for);
                continue;
            }
        };

        let output = execute_command(job.command, job.args);
        let job_result = common::UpdateJobResult {
            job_id: job.id,
            output,
        };
        match api_client
            .post(post_job_result_route.as_str())
            .send_json(ureq::json!(job_result))
        {
            Ok(_) => {}
            Err(err) => {
                log::debug!("Error sending job result back go the server: {}", err);
            }
        }
    }
}

fn execute_command(command: String, args: Vec<String>) -> String {
    let mut ret = String::new();
    let output = match Command::new(&command).args(args).output() {
        Ok(output) => output,
        Err(err) => {
            log::debug!("Error executing command: {}", err);
            return ret;
        }
    };

    ret = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(err) => {
            log::debug!("Error converting command's output to string: {}", err);
            return ret;
        }
    };

    ret
}
