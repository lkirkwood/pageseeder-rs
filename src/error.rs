use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum PSError {
    ServerError {
        msg: String
    },
    TokenError {
        cause: String
    }
}

pub type PSResult<T> = Result<T, PSError>;

impl Error for PSError {}
impl Display for PSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ServerError { msg } => {
                write!(f, "Operation failed on the server: {}", msg)
            },
            Self::TokenError { cause } => {
                write!(f, "Error using token: {}", cause)
            }
        }
    }
}