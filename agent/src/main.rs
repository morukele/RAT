use std::time::Duration;

mod error;
mod init;
mod run;

pub const SERVER_URL: &str = "http://127.0.0.1:8000";
pub const AGENT_ID_FILE: &str = "0xSpada";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("0xSpada_agent/0.1")
        .build();

    let agent_id = init::init(&api_client)?;
    run::run(&api_client, agent_id);
}
