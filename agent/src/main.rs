use std::time::Duration;

pub mod config;
mod error;
mod init;
mod run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_client = ureq::AgentBuilder::new()
        .timeout(Duration::from_secs(10))
        .user_agent("0xSpada_agent/0.1")
        .build();

    let config = init::init(&api_client)?;
    run::run(&api_client, config);
}
