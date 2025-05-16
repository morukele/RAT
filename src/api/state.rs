use actix_web::dev::Service;

#[derive(Debug)]
pub struct AppState {
    pub service: Service,
}

impl AppState {
    pub fn new() -> Self {
        AppState {}
    }
}
