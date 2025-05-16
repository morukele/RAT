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
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            _ => Error::Internal(err.to_string()),
        }
    }
}
