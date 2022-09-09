pub mod model;
pub mod oauth;

use std::error::Error;
use std::future::Future;

use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{blocking, Body, Client, Error as ReqwError, Response};
use serde::Serialize;

use crate::error::{PSError, PSResult};

use self::model::HttpScheme;
use self::oauth::PSToken;

pub struct BlockingPSServer {
    pub hostname: String,
    pub port: usize,
    pub scheme: HttpScheme,
    credentials: oauth::PSCredentials,
    client: blocking::Client,
    token: Option<PSToken>,
}

impl BlockingPSServer {
    pub fn new(
        hostname: String,
        credentials: oauth::PSCredentials,
        scheme: Option<HttpScheme>,
        port: Option<usize>,
    ) -> Self {
        return BlockingPSServer {
            hostname,
            port: port.unwrap_or(443),
            scheme: scheme.unwrap_or(HttpScheme::Https),
            credentials,
            client: blocking::Client::new(),
            token: None,
        };
    }

    /// Returns the uri slug appended to the PS url.
    fn format_url(&self, uri_slug: &str) -> String {
        format!(
            "{}://{}:{}/{}",
            self.scheme,
            self.hostname,
            self.port,
            uri_slug.trim_start_matches('/')
        )
    }

    // Unchecked

    /// Makes a get request to the server at the specified uri slug.
    pub fn get(
        &self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> PSResult<blocking::Response> {
        let mut req = self.client.get(self.format_url(uri_slug));
        if params.is_some() {
            req = req.query(&params.unwrap());
        }
        if headers.is_some() {
            req = req.headers(headers.unwrap());
        }
        match req.send() {
            Ok(resp) => return Ok(resp),
            Err(err) => {
                return Err(PSError::CommunicationError {
                    msg: format!("Failed to get {}; {:?}", uri_slug, err),
                })
            }
        }
    }

    /// Makes a post request to the server at the specified uri slug.
    /// Body data is included if provided.
    pub fn post<T: Into<blocking::Body>>(
        &self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        body: Option<T>,
    ) -> PSResult<blocking::Response> {
        let mut req = self.client.get(self.format_url(uri_slug));
        if params.is_some() {
            req = req.query(params.unwrap());
        }
        if headers.is_some() {
            req = req.headers(headers.unwrap());
        }
        if body.is_some() {
            req = req.body(body.unwrap());
        }
        match req.send() {
            Ok(resp) => return Ok(resp),
            Err(err) => {
                return Err(PSError::CommunicationError {
                    msg: format!("Failed to post {}; {:?}", uri_slug, err),
                })
            }
        }
    }

    /// Makes a post request to the server at the specified uri slug.
    /// Form data is included if provided.
    pub fn post_form<F: Serialize + ?Sized>(
        &self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        form: Option<&F>,
    ) -> PSResult<blocking::Response> {
        let mut req = self.client.post(self.format_url(uri_slug));
        if params.is_some() {
            req = req.query(params.unwrap());
        }
        if headers.is_some() {
            req = req.headers(headers.unwrap());
        }
        if form.is_some() {
            req = req.form(form.unwrap());
        }
        match req.send() {
            Ok(resp) => return Ok(resp),
            Err(err) => {
                return Err(PSError::CommunicationError {
                    msg: format!("Failed to post {}; {:?}", uri_slug, err),
                })
            }
        }
    }

    // Token

    /// Returns true if the currently stored token is valid.
    fn valid_token(&self) -> bool {
        match &self.token {
            None => false,
            Some(token) => token.expiry.gt(&Utc::now()),
        }
    }

    /// Gets a new access token for the server.
    fn get_token(&self) -> PSResult<PSToken> {
        let resp = self.post_form(
            "/ps/oauth/token",
            None,
            None,
            Some(&self.credentials.to_map()),
        )?;

        let resp_text = match resp.text() {
            Err(err) => {
                return Err(PSError::TokenError {
                    msg: format!("Failed to get text from token response"),
                })
            }
            Ok(txt) => txt,
        };
        let token_resp: oauth::TokenResponse = match serde_json::from_str(&resp_text) {
            Err(err) => {
                return Err(PSError::TokenError {
                    msg: format!(
                        "Failed to parse response as json: {:?}. Response was: {}",
                        err, &resp_text
                    ),
                })
            }
            Ok(tr) => tr,
        };
        return Ok(PSToken::expires_in(
            token_resp.access_token,
            token_resp.expires_in,
        ));
    }

    /// Gets a new access token and stores it only if the current one is invalid.
    fn update_token(&mut self) -> PSResult<()> {
        if !self.valid_token() {
            self.get_token()?;
        }
        return Ok(());
    }

    // Checked

    pub fn checked_get(
        &mut self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> PSResult<blocking::Response> {
        self.update_token()?;
        return self.get(uri_slug, params, headers);
    }

    /// Makes a post request to the server at the specified uri slug.
    /// Body data is included if provided.
    pub fn checked_post<T: Into<blocking::Body>>(
        &mut self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        body: Option<T>,
    ) -> PSResult<blocking::Response> {
        self.update_token()?;
        return self.post(uri_slug, params, headers, body);
    }

    /// Makes a post request to the server at the specified uri slug.
    /// Form data is included if provided.
    /// Token is updated if necessary.
    pub fn checked_post_form<F: Serialize + ?Sized>(
        &mut self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        form: Option<&F>,
    ) -> PSResult<blocking::Response> {
        self.update_token()?;
        return self.post_form(uri_slug, params, headers, form);
    }
}

/// A struct for making asynchronous calls to a PageSeeder server.
pub struct AsyncPSServer {
    pub hostname: String,
    pub port: usize,
    pub scheme: HttpScheme,
    credentials: oauth::PSCredentials,
    client: Client,
    token: Option<PSToken>,
}

impl AsyncPSServer {
    pub fn new(
        hostname: String,
        credentials: oauth::PSCredentials,
        scheme: Option<HttpScheme>,
        port: Option<usize>,
    ) -> Self {
        return AsyncPSServer {
            hostname: hostname,
            port: port.unwrap_or(443),
            scheme: scheme.unwrap_or(HttpScheme::Https),
            credentials,
            client: Client::new(),
            token: None,
        };
    }

    /// Returns the uri slug appended to the PS url.
    fn format_url(&self, uri_slug: &str) -> String {
        format!(
            "{}://{}:{}/{}",
            self.scheme,
            self.hostname,
            self.port,
            uri_slug.trim_start_matches('/')
        )
    }

    // Unchecked

    /// Makes a get request to the server at the specified uri slug.
    pub fn get(
        &self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> impl Future<Output = Result<Response, ReqwError>> {
        let mut req = self.client.get(self.format_url(uri_slug));
        if params.is_some() {
            req = req.query(params.unwrap());
        }
        if headers.is_some() {
            req = req.headers(headers.unwrap())
        }
        return req.send();
    }

    /// Makes a post request to the server at the specified uri slug.
    /// Body data is included if provided.
    pub fn post<T: Into<Body>>(
        &self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        body: Option<T>,
    ) -> impl Future<Output = Result<Response, ReqwError>> {
        let mut req = self.client.get(self.format_url(uri_slug));
        if params.is_some() {
            req = req.query(params.unwrap());
        }
        if headers.is_some() {
            req = req.headers(headers.unwrap());
        }
        if body.is_some() {
            req = req.body(body.unwrap());
        }
        return req.send();
    }

    /// Makes a post request to the server at the specified uri slug.
    /// Form data is included if provided.
    pub fn post_form<F: Serialize + ?Sized>(
        &self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        form: Option<&F>,
    ) -> impl Future<Output = Result<Response, ReqwError>> {
        let mut req = self.client.get(self.format_url(uri_slug));
        if params.is_some() {
            req = req.query(params.unwrap());
        }
        if headers.is_some() {
            req = req.headers(headers.unwrap());
        }
        if form.is_some() {
            req = req.form(form.unwrap());
        }
        return req.send();
    }

    // Token

    /// Returns true if the currently stored token is valid.
    fn valid_token(&self) -> bool {
        match &self.token {
            None => false,
            Some(token) => token.expiry.gt(&Utc::now()),
        }
    }

    /// Gets a new access token for the server.
    async fn get_token(&self) -> PSResult<PSToken> {
        let resp_res = self.client.post("/ps/oauth/token").send().await;

        let resp = match resp_res {
            Ok(resp) => resp,
            Err(err) => {
                return Err(PSError::CommunicationError {
                    msg: format!("Post to token endpoint failed: {:?}", err),
                })
            }
        };

        let resp_text = match resp.text().await {
            Err(err) => {
                return Err(PSError::TokenError {
                    msg: format!("Failed to get text from token response: {:?}", err),
                })
            }
            Ok(txt) => txt,
        };

        let token_resp: oauth::TokenResponse = match serde_json::from_str(&resp_text) {
            Err(err) => {
                return Err(PSError::TokenError {
                    msg: format!(
                        "Failed to parse response as json: {:?}. Response was: {}",
                        err, &resp_text
                    ),
                })
            }
            Ok(tr) => tr,
        };
        return Ok(PSToken::expires_in(
            token_resp.access_token,
            token_resp.expires_in,
        ));
    }

    /// Gets a new access token and stores it only if the current one is invalid.
    async fn update_token(&mut self) -> PSResult<()> {
        if !self.valid_token() {
            self.get_token().await?;
        }
        return Ok(());
    }

    // Checked

    pub async fn checked_get(
        &mut self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> impl Future<Output = Result<Response, ReqwError>> {
        self.update_token().await;
        return self.get(uri_slug, params, headers);
    }

    pub async fn checked_post<T: Into<Body>>(
        &mut self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        body: Option<T>,
    ) -> impl Future<Output = Result<Response, ReqwError>> {
        self.update_token().await;
        return self.post(uri_slug, params, headers, body);
    }

    pub async fn checked_post_form<F: Serialize + ?Sized>(
        &mut self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        form: Option<&F>,
    ) -> impl Future<Output = Result<Response, ReqwError>> {
        self.update_token().await;
        return self.post_form(uri_slug, params, headers, form);
    }
}
