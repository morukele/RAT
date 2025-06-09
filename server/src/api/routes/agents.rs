use crate::api::AppState;
use crate::entities::AgentDetail;
use crate::{common, error};
use actix_web::web::Json;
use actix_web::{HttpResponse, get, post, web};

#[get("/agents")]
pub async fn get_agents(state: web::Data<AppState>) -> Result<HttpResponse, error::Error> {
    let agent = state.service.list_agents().await?;
    let agents = agent.into_iter().collect();
    let res = common::AgentList { agents };

    let res = common::Response::ok(res);
    Ok(HttpResponse::Ok().json(res))
}

#[post("/agents")]
pub async fn post_agents(
    state: web::Data<AppState>,
    agent_details: Json<AgentDetail>,
) -> Result<HttpResponse, error::Error> {
    // get details of agent
    let agent_info = state
        .service
        .register_agent(agent_details.into_inner())
        .await?;

    let res = common::Response::ok(agent_info);
    Ok(HttpResponse::Ok().json(res))
}
