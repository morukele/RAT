use crate::{config, error};
use common::{api, crypto};
use ed25519_dalek::ed25519::signature::SignerMut;
use local_ip_address::local_ip;
use rand::RngCore;
use std::fs;
use std::net::{IpAddr, Ipv4Addr};
use std::path::PathBuf;
use x25519_dalek::{X25519_BASEPOINT_BYTES, x25519};

pub fn init(api_client: &ureq::Agent) -> Result<config::Config, error::Error> {
    let saved_agent_id = get_saved_agent_config()?;

    let config = match saved_agent_id {
        Some(agent_id) => agent_id,
        None => {
            let config = register(api_client)?;
            save_agent_config(&config)?;
            config
        }
    };

    Ok(config)
}

pub fn register(api_client: &ureq::Agent) -> Result<config::Config, error::Error> {
    let register_agent_route = format!("{}/api/agents", config::SERVER_URL);

    // key generation
    let mut rand_generator = rand::rngs::OsRng;
    let mut identity_keypair = ed25519_dalek::Keypair::generate(&mut rand_generator);
    let mut private_prekey = [0u8; crypto::X25519_PRIVATE_KEY_SIZE];
    rand_generator.fill_bytes(&mut private_prekey);
    let public_prekey = x25519(private_prekey, X25519_BASEPOINT_BYTES);
    let public_prekey_signature = identity_keypair.sign(&public_prekey);

    let agent_detals = api::RegisterAgent {
        ip_addr: local_ip()
            .unwrap_or(IpAddr::from(Ipv4Addr::new(0, 0, 0, 0)))
            .to_string(),
        name: whoami::realname(),
        username: whoami::username(),
        identity_public_key: identity_keypair.public.to_bytes(),
        public_prekey: public_prekey.clone(),
        public_prekey_signature: public_prekey_signature.to_bytes().to_vec(),
    };

    let api_res: api::Response<api::AgentRegistered> = api_client
        .post(register_agent_route.as_str())
        .send_json(agent_detals)?
        .into_json()?;

    if let Some(err) = api_res.error {
        return Err(error::Error::Api(err.message));
    }

    // return agent information
    let client_public_key_bytes = base64::decode(config::CLIENT_IDENTITY_PUBLIC_KEY)
        .map_err(|e| error::Error::Internal(e.to_string()))?;
    let client_identity_public_key = ed25519_dalek::PublicKey::from_bytes(&client_public_key_bytes)
        .map_err(|e| error::Error::Internal(e.to_string()))?;

    let config = config::Config {
        agent_id: api_res.data.unwrap().id,
        identity_public_key: identity_keypair.public,
        identity_private_key: identity_keypair.secret,
        public_prekey,
        private_prekey,
        client_identity_public_key,
    };

    Ok(config)
}

pub fn save_agent_config(config: &config::Config) -> Result<(), error::Error> {
    let agent_config_file = get_agent_config_file_path()?;

    let serialized_config: config::SerializedConfig = config.into();
    let config_json = serde_json::to_string(&serialized_config)
        .map_err(|e| error::Error::Internal(e.to_string()))?;

    fs::write(agent_config_file, config_json.as_bytes())?;

    Ok(())
}

pub fn get_saved_agent_config() -> Result<Option<config::Config>, error::Error> {
    let agent_id_file = get_agent_config_file_path()?;
    if agent_id_file.exists() {
        let agent_file_content = fs::read(agent_id_file)?;

        let serialized_config: config::SerializedConfig =
            serde_json::from_slice(&agent_file_content)
                .map_err(|e| error::Error::Internal(e.to_string()))?;

        let config = serialized_config.try_into()?;
        Ok(Some(config))
    } else {
        Ok(None)
    }
}

pub fn get_agent_config_file_path() -> Result<PathBuf, error::Error> {
    let mut home_dir = match dirs::home_dir() {
        Some(home_dir) => home_dir,
        None => {
            return Err(error::Error::Internal(
                "Error getting home directory".to_string(),
            ));
        }
    };

    home_dir.push(config::AGENT_ID_FILE);

    Ok(home_dir)
}
