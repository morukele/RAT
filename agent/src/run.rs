use crate::{config, error};
use blake2::digest::{Update, VariableOutput};
use chacha20poly1305::XChaCha20Poly1305;
use chacha20poly1305::aead::{AeadMut, NewAead};
use common::api::{self, AgentJob, JobPayload, UpdateJobResult};
use common::crypto;
use rand::RngCore;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;
use x25519_dalek::x25519;
use zeroize::Zeroize;

pub fn run(api_client: &ureq::Agent, config: config::Config) -> ! {
    let sleep_for = Duration::from_secs(1);
    let get_job_route = format!("{}/api/agents/{}/job", config::SERVER_URL, config.agent_id);
    let post_job_result_route = format!("{}/api/jobs/result", config::SERVER_URL);

    loop {
        let server_res = match api_client.get(get_job_route.as_str()).call() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error getting job from server: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        let api_res: api::Response<api::AgentJob> = match server_res.into_json() {
            Ok(res) => res,
            Err(err) => {
                log::debug!("Error parsing JSON: {}", err);
                sleep(sleep_for);
                continue;
            }
        };

        log::debug!("api response successfully received: {:?}", api_res);

        let encrypted_job = match api_res.data {
            Some(job) => job,
            None => {
                log::debug!("No job found. Trying again in {:?} seconds", sleep_for);
                sleep(sleep_for);
                continue;
            }
        };

        let (job_id, job) = match decrypt_and_verify_job(&config, encrypted_job) {
            Ok(res) => res,
            Err(e) => {
                log::debug!("Error decrypting job: {}", e);
                sleep(sleep_for);
                continue;
            }
        };

        let output = execute_command(job.command, job.args);

        let job_result = match encrypt_and_sign_job_result(
            &config,
            job_id,
            output,
            job.result_ephemeral_public_key,
        ) {
            Ok(res) => res,
            Err(e) => {
                log::debug!("Error encrypting job result: {}", e);
                sleep(sleep_for);
                continue;
            }
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

fn decrypt_and_verify_job(
    config: &config::Config,
    job: AgentJob,
) -> Result<(Uuid, JobPayload), error::Error> {
    // verify input
    if job.signature.len() != crypto::ED25519_SIGNATURE_SIZE {
        return Err(error::Error::Internal(
            "Job's signature size is not valid".to_string(),
        ));
    }

    // verify job_id, agent_id, encrypted_job, ephemeral_public_key, nonce
    let mut buffer_to_verify = job.id.as_bytes().to_vec();
    buffer_to_verify.append(&mut config.agent_id.as_bytes().to_vec());
    buffer_to_verify.append(&mut job.encrypted_job.clone());
    buffer_to_verify.append(&mut job.ephemeral_public_key.to_vec());
    buffer_to_verify.append(&mut job.nonce.to_vec());

    let signature = ed25519_dalek::Signature::try_from(&job.signature[0..64])
        .map_err(|e| error::Error::Internal(e.to_string()))?;
    if config
        .client_identity_public_key
        .verify_strict(&buffer_to_verify, &signature)
        .is_err()
    {
        return Err(error::Error::Internal(
            "Agent's prekey Signature not valid".to_string(),
        ));
    }

    // key exchange
    let mut shared_secret = x25519(config.private_prekey, job.ephemeral_public_key);

    // derive key
    let mut kdf =
        blake2::VarBlake2b::new_keyed(&shared_secret, crypto::XCHACHA20_POLY1305_KEY_SIZE);
    kdf.update(&job.nonce);
    let mut key = kdf.finalize_boxed();

    // decrypt job
    let mut cipher = XChaCha20Poly1305::new(key.as_ref().into());
    let decrypted_job_bytes = cipher
        .decrypt(&job.nonce.into(), job.encrypted_job.as_ref())
        .map_err(|e| error::Error::Internal(e.to_string()))?;

    shared_secret.zeroize();
    key.zeroize();

    // deserialize job
    let job_payload: api::JobPayload = serde_json::from_slice(&decrypted_job_bytes)
        .map_err(|e| error::Error::Internal(e.to_string()))?;

    Ok((job.id, job_payload))
}

fn encrypt_and_sign_job_result(
    config: &config::Config,
    job_id: Uuid,
    output: String,
    job_result_ephemeral_public_key: [u8; crypto::X25519_PUBLIC_KEY_SIZE],
) -> Result<UpdateJobResult, error::Error> {
    let mut rand_generator = rand::rngs::OsRng {};

    // generate ephemeral keypair for job result encryption
    let mut ephemeral_private_key = [0u8; crypto::X25519_PRIVATE_KEY_SIZE];
    rand_generator.fill_bytes(&mut ephemeral_private_key);
    let ephemeral_public_key = x25519(
        ephemeral_private_key.clone(),
        x25519_dalek::X25519_BASEPOINT_BYTES,
    );

    // key exchange for job result encryption
    let mut shared_secret = x25519(ephemeral_private_key, job_result_ephemeral_public_key);

    // generate nonce
    let mut nonce = [0u8; crypto::XCHACHA20_POLY1305_NONCE_SIZE];
    rand_generator.fill_bytes(&mut nonce);

    // derive key
    let mut kdf =
        blake2::VarBlake2b::new_keyed(&shared_secret, crypto::XCHACHA20_POLY1305_KEY_SIZE);
    kdf.update(&nonce);
    let mut key = kdf.finalize_boxed();

    // serialize job result
    let job_result_payload = api::JobResult { output };
    let job_result_payload_json = serde_json::to_vec(&job_result_payload)
        .map_err(|e| error::Error::Internal(e.to_string()))?;

    // encrypt job
    let mut cipher = XChaCha20Poly1305::new(key.as_ref().into());
    let encrypted_job_result = cipher
        .encrypt(&nonce.into(), job_result_payload_json.as_ref())
        .map_err(|e| error::Error::Internal(e.to_string()))?;

    shared_secret.zeroize();
    key.zeroize();

    // Sign the payload
    let mut buffer_to_sign = job_id.as_bytes().to_vec();
    buffer_to_sign.append(&mut config.agent_id.as_bytes().to_vec());
    buffer_to_sign.append(&mut encrypted_job_result.clone());
    buffer_to_sign.append(&mut ephemeral_public_key.to_vec());
    buffer_to_sign.append(&mut nonce.to_vec());

    let identity = ed25519_dalek::ExpandedSecretKey::from(&config.identity_private_key);
    let signature = identity.sign(&buffer_to_sign, &config.identity_public_key);

    Ok(UpdateJobResult {
        job_id,
        encrypted_job_result,
        ephemeral_public_key,
        nonce,
        signature: signature.to_bytes().to_vec(),
    })
}
