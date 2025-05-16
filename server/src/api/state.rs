use crate::service::Service;

#[derive(Debug, Clone)]
pub struct AppState {
    pub service: Service,
}

impl AppState {
    pub fn new(service: Service) -> Self {
        AppState { service }
    }
}
