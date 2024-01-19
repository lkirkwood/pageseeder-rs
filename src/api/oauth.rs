use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};
use reqwest::header::HeaderValue;
use serde::Deserialize;

use crate::error::{PSError, PSResult};

#[derive(Debug)]
pub enum PSCredentials {
    ClientCredentials { id: String, secret: String },
}

impl PSCredentials {
    /// Returns a map of parameters to use to request a grant.
    pub fn to_map(&self) -> HashMap<&'static str, String> {
        match self {
            Self::ClientCredentials { id, secret } => {
                let mut map = HashMap::new();
                map.insert("grant_type", "client_credentials".to_string());
                map.insert("client_id", id.clone());
                map.insert("client_secret", secret.clone());
                map
            }
        }
    }
}

#[derive(Debug, Clone)]
/// Temporary access token for making calls to psapi.
pub struct PSToken {
    pub token: String,
    pub expiry: DateTime<Utc>,
    pub header: HeaderValue,
}

impl PSToken {
    /// Creates a PSToken that will expire in the given number of seconds.
    pub fn expires_in(token: String, seconds: i64) -> PSResult<PSToken> {
        let header = match HeaderValue::from_str(&format!("Bearer {}", token)) {
            Err(err) => {
                return Err(PSError::TokenError {
                    msg: format!("Invalid token {}", err),
                })
            }
            Ok(header) => header,
        };

        Ok(PSToken {
            token,
            expiry: Utc::now() + Duration::seconds(seconds),
            header,
        })
    }
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: Option<String>,
}
