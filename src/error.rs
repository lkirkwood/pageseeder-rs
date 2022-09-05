use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct PageSeederError {
    hostname: String,
    message: String
}
impl Error for PageSeederError {}
impl Display for PageSeederError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operation failed with PS server \"{}\"; {}", self.hostname, self.message)
    }
}

macro_rules! pserror {
    ($ps:expr, $msg:expr) => {
        PageSeederError {hostname: ps.hostname.clone(), message: msg.to_string()}
    }
}

pub(crate) use pserror;