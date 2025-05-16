use crate::api::AppState;
use actix_web::{Responder, web};

pub async fn create_job(
    state: web::Data<AppState>,
    input: crate::api::CreateJob,
) -> impl Responder {
    let job = state.service.create_job(input).await?;
}
