mod agents;
mod index;
mod jobs;

use super::AppState;
use actix_web::web;
use std::sync::Arc;

pub fn routes(app_state: Arc<AppState>, cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(get_jobs)
            .service(create_job)
            .service(get_job_result)
            .service(post_job_result)
            .service(post_agents)
            .service(get_agent_job),
    )
    .app_data(app_state);
}
