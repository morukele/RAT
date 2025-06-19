use crate::api::AppState;
use crate::error;
use actix_web::{HttpResponse, get, post, web};
use common::api;
use std::time::Duration;
use uuid::Uuid;

#[post("/jobs")]
pub async fn create_job(
    state: web::Data<AppState>,
    input: web::Json<api::CreateJob>,
) -> Result<HttpResponse, error::Error> {
    let job = state.service.create_job(input.into_inner()).await?;
    let job: api::Job = job.into();
    let res = api::Response::ok(job);

    Ok(HttpResponse::Ok().json(res))
}
#[get("/jobs/{job_id}/result")]
pub async fn get_job_result(
    state: web::Data<AppState>,
    job_id: web::Path<Uuid>,
) -> Result<HttpResponse, error::Error> {
    // check if job_id is a valid Uuid
    let job_id = job_id.into_inner();
    log::debug!("Job Route -> job_id: {}", job_id);

    let sleep_for = Duration::from_secs(1);

    // long polling: 5 secs
    for _ in 0..5u64 {
        match state.service.get_job_result(job_id).await? {
            Some(job) => {
                let job: api::Job = job.into();
                let res = api::Response::ok(job);
                return Ok(HttpResponse::Ok().json(res));
            }
            None => tokio::time::sleep(sleep_for).await,
        }
    }

    Ok(HttpResponse::Ok().finish())
}

#[get("/agents/{agent_id}/job")]
pub async fn get_agent_job(
    state: web::Data<AppState>,
    agent_id: web::Path<Uuid>,
) -> Result<HttpResponse, error::Error> {
    let agent_id = agent_id.into_inner();
    let sleep_for = Duration::from_secs(1);

    // long polling: 5 secs
    for _ in 0..5u64 {
        match state.service.get_agent_job(agent_id).await? {
            Some(job) => {
                let agent_job = api::AgentJob {
                    id: job.id,
                    encrypted_job: job.encrypted_job,
                    ephemeral_public_key: job
                        .ephemeral_public_key
                        .try_into()
                        .expect("get_agent_job: invalid ephemeral_public_key"),
                    nonce: job.nonce.try_into().expect("get_agen_job: invalid nonce"),
                    signature: job.signature,
                };

                let res = api::Response::ok(agent_job);
                return Ok(HttpResponse::Ok().json(res));
            }
            None => tokio::time::sleep(sleep_for).await,
        }
    }

    // if no job is found, we return empty response
    let res = api::Response::<Option<()>>::ok(None);
    Ok(HttpResponse::Ok().json(res))
}

#[get("/jobs")]
pub async fn get_jobs(state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    let jobs = state.service.list_jobs().await?;
    let jobs = jobs.into_iter().map(Into::into).collect();
    let res = api::JobList { jobs };

    let res = api::Response::ok(res);
    Ok(HttpResponse::Ok().json(res))
}

#[post("/jobs/result")]
pub async fn post_job_result(
    state: web::Data<AppState>,
    input: web::Json<api::UpdateJobResult>,
) -> Result<HttpResponse, error::Error> {
    state.service.update_job_result(input.into_inner()).await?;

    let res = api::Response::ok(true);
    Ok(HttpResponse::Ok().json(res))
}
