use crate::{AGENT_ID_FILE, SERVER_URL, error};
use local_ip_address::local_ip;
use server::{common, entities};
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use uuid::Uuid;

pub fn init(api_client: &ureq::Agent) -> Result<Uuid, error::Error> {
    let saved_agent_id = get_saved_agent_id()?;

    let agent_id = match saved_agent_id {
        Some(agent_id) => agent_id,
        None => {
            let agent_id = register(api_client)?;
            save_agent_id(agent_id)?;
            agent_id
        }
    };

    Ok(agent_id)
}

pub fn register(api_client: &ureq::Agent) -> Result<Uuid, error::Error> {
    let register_agent_route = format!("{}/api/agents", SERVER_URL);
    let mut rand_generator = rand::rngs::OsRng;
    let identity_key = ed25519_dalek::Keypair::generate(&mut rand_generator);
    let agent_detals = entities::AgentDetail {
        ip_addr: local_ip()
            .unwrap_or(IpAddr::from(Ipv4Addr::new(0, 0, 0, 0)))
            .to_string(),
        name: whoami::realname(),
        username: whoami::username(),
    };

    let api_res: common::Response<common::AgentRegistered> = api_client
        .post(register_agent_route.as_str())
        .send_json(agent_detals)?
        .into_json()?;

    let agent_id = match (api_res.data, api_res.error) {
        (Some(data), None) => Ok(data.id),
        (None, Some(error)) => Err(error::Error::Api(error.message)),
        (None, None) => Err(error::Error::Api(
            "Received invalid api response: data and error are both null".to_string(),
        )),
        (Some(_), Some(_)) => Err(error::Error::Api(
            "Received invalid api response: data and error are both non null".to_string(),
        )),
    }?;

    Ok(agent_id)
}

pub fn save_agent_id(agent_id: Uuid) -> Result<(), error::Error> {
    let agent_id_file = get_agent_id_file_path()?;
    fs::write(agent_id_file, agent_id.as_bytes())?;

    Ok(())
}

pub fn get_saved_agent_id() -> Result<Option<Uuid>, error::Error> {
    let agent_id_file = get_agent_id_file_path()?;
    if agent_id_file.exists() {
        let agent_file_content = fs::read(agent_id_file)?;

        let agent_id = Uuid::from_slice(&agent_file_content)?;
        Ok(Some(agent_id))
    } else {
        Ok(None)
    }
}

pub fn get_agent_id_file_path() -> Result<PathBuf, error::Error> {
    let mut home_dir = match dirs::home_dir() {
        Some(home_dir) => home_dir,
        None => return Err(error::Error::Internal("Error getting home dir".to_string())),
    };

    home_dir.push(AGENT_ID_FILE);

    Ok(home_dir)
}
