pub mod routes;
pub mod state;

use actix_web::http::StatusCode;
use actix_web::web;
use serde::de::DeserializeOwned;
// public re-export
pub use state::AppState;

pub fn json_body<T: DeserializeOwned + Send + 'static>() -> web::JsonConfig {
    web::JsonConfig::default()
        .limit(1024 * 16)
        .error_handler(|err, _req| {
            let error_msg = format!("Invalid JSON {:?}", err);
            actix_web::error::InternalError::new(error_msg, StatusCode::BAD_REQUEST).into()
        })
}
