mod oauth;

use std::{error::Error};


use reqwest::blocking;

pub enum HttpScheme {
    Http,
    Https
}

impl HttpScheme {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Https => "https"
        }
    }
}

pub struct PSServer {
    pub hostname: String,
    pub port: usize,
    pub scheme: HttpScheme,
    credentials: oauth::PSCredentials,
    client: blocking::Client,
    token: Option<String>
}

impl PSServer {
    pub fn new(hostname: String, credentials: oauth::PSCredentials) -> Self {
        return PSServer { 
            hostname, 
            port: 44, 
            scheme: HttpScheme::Https, 
            credentials,
            client: blocking::Client::new(),
            token: None 
        }
    }

    /// Gets a new access token.
    /// Returns the expiry in seconds.
    pub fn get_token(&mut self) -> Result<usize, Box<dyn Error>> {
        let resp = self.client.post(
            format!("{}://{}:{}/ps/oauth", self.scheme.to_str(), self.hostname, self.port)
        ).form(&self.credentials.to_params()).send()?;

        let token_resp: oauth::TokenResponse = serde_json::from_str(&resp.text()?)?;
        self.token = Some(token_resp.access_token);
        return Ok(token_resp.expires_in);
    }
}