use std::fmt;

#[derive(Debug)]
pub enum Error {
    Internal(String),
    Transport(ureq::Error),
    Api(String),
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl std::convert::From<ureq::Error> for Error {
    fn from(value: ureq::Error) -> Self {
        match value {
            err @ ureq::Error::Transport(_) => Error::Transport(err),
            err @ ureq::Error::Status(_, _) => Error::Api(err.to_string()),
        }
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl std::convert::From<uuid::Error> for Error {
    fn from(value: uuid::Error) -> Self {
        Error::Internal(value.to_string())
    }
}
