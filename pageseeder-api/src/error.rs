use std::{error::Error, fmt::Display};

use crate::model;

#[derive(Debug)]
pub enum PSError {
    CommunicationError { msg: String },
    ParseError { msg: String, xml: String },
    ServerError { msg: String },
    TokenError { msg: String },
    ApiError(model::Error),
}

pub type PSResult<T> = Result<T, PSError>;

impl Error for PSError {}
impl Display for PSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommunicationError { msg } => {
                write!(f, "Error communicating with server: {}", msg)
            }
            Self::ParseError { msg, xml } => {
                write!(
                    f,
                    "Error parsing server response: {}; Response was: {}",
                    msg, xml
                )
            }
            Self::ServerError { msg } => {
                write!(f, "Operation failed on the server: {}", msg)
            }
            Self::TokenError { msg } => {
                write!(f, "Error using token: {}", msg)
            }
            Self::ApiError(err) => {
                write!(
                    f,
                    "Server reported error during following request: {}; Error was: {}",
                    err.request, err.message
                )
            }
        }
    }
}
