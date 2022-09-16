pub mod model;
pub mod oauth;
pub mod services;

use std::future::Future;

use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Body, Client};
use serde::Serialize;

use crate::error::{AsyncResult, PSError, PSResult};

use self::model::HttpScheme;
use self::oauth::PSToken;

/// A struct for making asynchronous calls to a PageSeeder server.
pub struct PSServer {
    pub hostname: String,
    pub port: usize,
    pub scheme: HttpScheme,
    credentials: oauth::PSCredentials,
    client: Client,
    token: Option<PSToken>,
    token_header: Option<HeaderValue>,
}

impl PSServer {
    /// Instantiates a new PSServer.
    /// Defaults to HTTPS and port 443.
    pub fn new(
        hostname: String,
        credentials: oauth::PSCredentials,
        scheme: Option<HttpScheme>,
        port: Option<usize>,
    ) -> Self {
        return PSServer {
            hostname: hostname,
            port: port.unwrap_or(443),
            scheme: scheme.unwrap_or(HttpScheme::Https),
            credentials,
            client: Client::new(),
            token: None,
            token_header: None,
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
    fn get(
        &self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> impl Future<Output = AsyncResult> {
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
    fn post<T: Into<Body>>(
        &self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        body: Option<T>,
    ) -> impl Future<Output = AsyncResult> {
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
    fn post_form<F: Serialize + ?Sized>(
        &self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        form: Option<&F>,
    ) -> impl Future<Output = AsyncResult> {
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
    async fn update_token(&mut self) -> PSResult<&HeaderValue> {
        if !self.valid_token() {
            self.get_token().await?;
            let token = self.token.unwrap();
            let header = HeaderValue::from_str(format!("Bearer {}", token));
            match header {
                Err(err) => return Err(PSError::TokenError {
                    msg: format!("Invalid token {}", err)}),
                Ok(header) => { self.token_header = Some(header); }
            }
        }
        return Ok(&self.token_header);
    }

    // Checked

    async fn checked_get(
        &mut self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> impl Future<Output = AsyncResult> {
        self.update_token().await;
        return self.get(uri_slug, params, headers);
    }

    async fn checked_post<T: Into<Body>>(
        &mut self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        body: Option<T>,
    ) -> impl Future<Output = AsyncResult> {
        self.update_token().await;
        return self.post(uri_slug, params, headers, body);
    }

    async fn checked_post_form<F: Serialize + ?Sized>(
        &mut self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        form: Option<&F>,
    ) -> impl Future<Output = AsyncResult> {
        self.update_token().await;
        return self.post_form(uri_slug, params, headers, form);
    }
}
