mod agents;
mod index;
mod jobs;

use crate::api::routes::agents::{get_agents, post_agents};
use crate::api::routes::jobs::{
    create_job, get_agent_job, get_job_result, get_jobs, post_job_result,
};
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_jobs)
            .service(create_job)
            .service(get_job_result)
            .service(post_job_result)
            .service(post_agents)
            .service(get_agents)
            .service(get_agent_job),
    );
}
