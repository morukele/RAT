use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error: {0}")]
    Internal(String),
    #[error("{0}")]
    NotFound(String),
}

impl std::convert::From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
        Error::Internal(value.to_string())
    }
}

impl std::convert::From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Internal(value.to_string())
    }
}
