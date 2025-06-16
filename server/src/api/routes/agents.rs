use crate::api::AppState;
use crate::error;
use actix_web::web::Json;
use actix_web::{HttpResponse, get, post, web};
use common::entities;
use uuid::Uuid;

#[get("/agents")]
pub async fn get_agents(state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    let agent = state.service.list_agents().await?;
    let agents = agent.into_iter().collect();
    let res = entities::AgentList { agents };

    let res = entities::Response::ok(res);
    Ok(HttpResponse::Ok().json(res))
}

#[post("/agents")]
pub async fn post_agents(
    state: web::Data<AppState>,
    agent_details: Json<entities::AgentCreationDetail>,
) -> Result<HttpResponse, error::Error> {
    // get details of agent
    let agent_info = state
        .service
        .register_agent(agent_details.into_inner())
        .await?;

    let res = entities::Response::ok(agent_info);
    Ok(HttpResponse::Ok().json(res))
}

#[get("/agents/{agent_id}")]
pub async fn get_agent(
    state: web::Data<AppState>,
    agent_id: web::Path<Uuid>,
) -> Result<HttpResponse, error::Error> {
    // get agent info
    let agent = state.service.find_agent(agent_id.into_inner()).await?;
    let res = entities::Response::ok(agent);

    Ok(HttpResponse::Ok().json(res))
}
