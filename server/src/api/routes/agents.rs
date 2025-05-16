use crate::api::AppState;
use crate::{common, error};
use actix_web::{get, post, web, HttpResponse};

#[get("/agents")]
pub async fn get_agents(state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    let agent = state.service.list_agents().await?;
    let agents = agent.into_iter().map(Into::into).collect();
    let res = common::AgentList { agents };

    let res = common::Response::ok(res);
    Ok(HttpResponse::Ok().json(res))
}

#[post("/agents")]
pub async fn post_agents(state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    let agent_info = state.service.register_agent().await?;

    let res = common::Response::ok(agent_info);
    Ok(HttpResponse::Ok().json(res))
}
