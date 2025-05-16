use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error")]
    Internal(String),
    #[error("Not found error")]
    NotFound(String),
    #[error("Database error")]
    DataBaseError(String),
    #[error("Migration error")]
    MigrationError(String),
    #[error("Invalid argument")]
    InvalidArgument(String),
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Error::Internal(err.to_string())
    }
}

impl actix_web::error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match self {
            Error::Internal(_) => HttpResponse::InternalServerError().finish(),
            Error::NotFound(_) => HttpResponse::NotFound().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
