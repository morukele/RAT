use std::time::Duration;

mod config;
mod error;
mod init;
mod run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("0xSpada_agent/0.1")
        .build();

    let agent_id = init::init(&api_client)?;
    run::run(&api_client, agent_id);
}
