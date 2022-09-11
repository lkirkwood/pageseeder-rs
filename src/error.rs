use std::{error::Error, fmt::Display, future::Future};

use reqwest::{Error as ReqwError, Response};

#[derive(Debug)]
pub enum PSError {
    CommunicationError { msg: String },
    ServerError { msg: String },
    TokenError { msg: String },
}

pub type PSResult<T> = Result<T, PSError>;
pub type AsyncResult = Result<Response, ReqwError>;

impl Error for PSError {}
impl Display for PSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommunicationError { msg } => {
                write!(f, "Error communicating with server: {}", msg)
            }
            Self::ServerError { msg } => {
                write!(f, "Operation failed on the server: {}", msg)
            }
            Self::TokenError { msg } => {
                write!(f, "Error using token: {}", msg)
            }
        }
    }
}
