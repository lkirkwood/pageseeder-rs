pub mod oauth;

use std::{error::Error, fmt::Display};
use chrono::{Utc, DateTime};


use reqwest::blocking;
use serde::Serialize;

use self::oauth::PSToken;

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
impl Display for HttpScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

pub struct PSServer {
    pub hostname: String,
    pub port: usize,
    pub scheme: HttpScheme,
    credentials: oauth::PSCredentials,
    client: blocking::Client,
    token: Option<PSToken>
}

impl PSServer {
    pub fn new(hostname: String, credentials: oauth::PSCredentials) -> Self {
        return PSServer { 
            hostname, 
            port: 443, 
            scheme: HttpScheme::Https, 
            credentials,
            client: blocking::Client::new(),
            token: None 
        }
    }

    // Unchecked

    /// Makes a post request to the server at the specified uri slug.
    /// Form data is included if provided.
    /// No token checking is done.
    fn generic_post<T: Serialize + ?Sized>(
        &self, uri_slug: &str, form: Option<&T>
    ) -> Result<blocking::Response, Box<dyn Error>> {
        let mut req = self.client.post(
            format!("{}://{}:{}/{}", self.scheme, self.hostname, self.port, uri_slug)
        );
        if form.is_some() {
            req = req.form(form.unwrap());
        }

        return Ok(req.send()?)
    }

    /// Gets a new access token for the server.
    fn get_token(&self) -> Result<PSToken, Box<dyn Error>> {
        let resp = self.generic_post("/ps/oauth/token", 
            Some(&self.credentials.to_params()))?;

        let token_resp: oauth::TokenResponse = serde_json::from_str(&resp.text()?)?;
        return Ok(PSToken::expires_in(token_resp.access_token, token_resp.expires_in))
    }

    /// Gets a new access token and stores it ONLY if it is invalid.
    fn update_token(&mut self) -> Result<(), Box<dyn Error>> {
        match &self.token {
            None => {
                self.token = Some(self.get_token()?);
            },
            Some(token) => {
                if token.expiry < Utc::now() {
                    self.token = Some(self.get_token()?);
                }
            }
        }
        return Ok(())
    }

    // Checked

    /// Makes a post request to the server at the specified uri slug.
    /// Form data is included if provided.
    /// Token is updated if expired.
    fn checked_post<T: Serialize + ?Sized>(
        &mut self, uri_slug: &str, form: Option<&T>
    ) -> Result<blocking::Response, Box<dyn Error>> {
        self.update_token()?;
        return self.generic_post(uri_slug, form)
    }
}