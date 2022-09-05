use std::collections::HashMap;

use serde::Deserialize;

pub enum PSCredentials {
    ClientCredentials {
        id: String,
        secret: String
    }
}

impl PSCredentials {
    /// Returns a map of parameters to use to request a grant.
    pub fn to_params(&self) -> HashMap<&'static str, String> {
        match self {
            Self::ClientCredentials { id, secret } => {
                let mut map = HashMap::new();
                map.insert("grant_type", "client_credentials".to_string());
                map.insert("client_id", id.clone());
                map.insert("client_secret", secret.clone());
                return map;
            }
        }
    }
}


#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub expires_in: usize,
    pub token_type: String,
    pub scope: Option<String>
}