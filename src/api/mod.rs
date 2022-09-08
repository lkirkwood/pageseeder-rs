pub mod oauth;

use std::{error::Error, fmt::Display, collections::HashMap};
use chrono::{Utc, DateTime};


use reqwest::blocking::{self, RequestBuilder};
use serde::Serialize;

use crate::error::{PSResult, PSError};

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

    /// Returns the uri slug appended to the PS url.
    fn format_url(&self, uri_slug: &str) -> String {
        format!("{}://{}:{}/{}", self.scheme, self.hostname, self.port, 
            uri_slug.trim_start_matches('/'))
    }

    // Unchecked

    /// Makes a get request to the server at the specified uri slug.
    pub fn get(
        &self, uri_slug: &str, params: Option<&Vec<(String, String)>>
    ) -> PSResult<blocking::Response> {
        let mut req = self.client.get(self.format_url(uri_slug));
        if params.is_some() {
            req = req.query(&params.unwrap());
        }
        match req.send() {
            Ok(resp) => return Ok(resp),
            Err(err) => {
                return Err(PSError::CommunicationError { 
                    msg: format!("Failed to get {}; {:?}", uri_slug, err)})
            }
        }
    }

    /// Makes a post request to the server at the specified uri slug.
    /// Form data is included if provided.
    pub fn post<F: Serialize + ?Sized>(
        &self, 
        uri_slug: &str, 
        params: Option<&Vec<(String, String)>>, 
        form: Option<&F>
    ) -> PSResult<blocking::Response> {
        let mut req = self.client.post(self.format_url(uri_slug));
        if params.is_some() {
            req = req.query(params.unwrap());
        }
        if form.is_some() {
            req = req.form(form.unwrap());
        }
        match req.send() {
            Ok(resp) => return Ok(resp),
            Err(err) => {
                return Err(PSError::CommunicationError { 
                    msg: format!("Failed to post {}; {:?}", uri_slug, err)})
            }
        }
    }

    // Token

    /// Returns true if the currently stored token is valid.
    fn valid_token(&self) -> bool {
        match &self.token {
            None => false,
            Some(token) => {
                token.expiry.gt(&Utc::now())
            }
        }
    }

    /// Gets a new access token for the server.
    fn get_token(&self) -> PSResult<PSToken> {
        let resp = self.post("/ps/oauth/token", 
            None, Some(&self.credentials.to_map()))?;

        let resp_text = match resp.text() {
            Err(err) => return Err(PSError::TokenError { 
                msg: format!("Failed to get text from token response")}),
            Ok(txt) => txt
        };
        let token_resp: oauth::TokenResponse = match serde_json::from_str(&resp_text) {
            Err(err) => return Err(PSError::TokenError {
                msg: format!("Failed to parse response as json: {:?}. Response was: {}", 
                    err, &resp_text)
            }),
            Ok(tr) => tr
        }        ;
        return Ok(PSToken::expires_in(token_resp.access_token, token_resp.expires_in))
    }

    /// Gets a new access token and stores it only if the current one is invalid.
    fn update_token(&mut self) -> PSResult<()> {
        if !self.valid_token() {
            self.get_token()?;
        }
        return Ok(())
    }

    // Checked

    pub fn checked_get(
        &mut self, uri_slug: &str, params: Option<&Vec<(String, String)>>
    ) -> PSResult<blocking::Response> {
        self.update_token()?;
        return self.get(uri_slug, params)
    }

    /// Makes a post request to the server at the specified uri slug.
    /// Form data is included if provided.
    /// Token is updated if necessary.
    pub fn checked_post<F: Serialize + ?Sized>(
        &mut self, 
        uri_slug: &str, 
        params: Option<&Vec<(String, String)>>, 
        form: Option<&F>
    ) -> PSResult<blocking::Response> {
        self.update_token()?;
        return self.post(uri_slug, params, form)
    }   
}