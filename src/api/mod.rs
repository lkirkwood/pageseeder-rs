pub mod model;
pub mod oauth;
pub mod services;
#[cfg(test)]
mod tests;

use chrono::Utc;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Body, Client, Response};
use serde::Serialize;

use crate::error::{PSError, PSResult};

use self::oauth::PSToken;

/// A struct for making asynchronous calls to a PageSeeder server.
pub struct PSServer {
    pub url: String,
    credentials: oauth::PSCredentials,
    client: Client,
    token: Option<PSToken>,
    token_header: Option<HeaderValue>,
}

impl PSServer {
    /// Instantiates a new PSServer.
    /// Defaults to HTTPS and port 443.
    pub fn new(url: String, credentials: oauth::PSCredentials) -> Self {
        PSServer {
            url,
            credentials,
            client: Client::new(),
            token: None,
            token_header: None,
        }
    }

    /// Returns the uri slug appended to the PS url.
    fn format_url(&self, uri: &str) -> String {
        format!("{}/{}", self.url, uri.trim_start_matches('/'))
    }

    // Unchecked

    /// Makes a get request to the server at the specified uri slug.
    async fn get(
        &self,
        uri: &str,
        params: Option<Vec<(&str, &str)>>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> PSResult<Response> {
        let mut req = self.client.get(self.format_url(uri));

        if let Some(params) = params {
            req = req.query(&params);
        }
        if let Some(headers) = headers {
            req = req.headers(headers)
        }

        match req.send().await {
            Ok(resp) => Ok(resp),
            Err(err) => Err(PSError::CommunicationError {
                msg: format!("Failed to get {}: {:?}", uri, err),
            }),
        }
    }

    /// Makes a post request to the server at the specified uri slug.
    /// Body data is included if provided.
    async fn post<T: Into<Body>>(
        &self,
        uri_slug: &str,
        params: Option<Vec<(&str, &str)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        body: Option<T>,
    ) -> PSResult<Response> {
        let mut req = self.client.get(self.format_url(uri_slug));

        if let Some(params) = params {
            req = req.query(&params);
        }
        if let Some(headers) = headers {
            req = req.headers(headers);
        }
        if let Some(body) = body {
            req = req.body(body);
        }

        match req.send().await {
            Ok(resp) => Ok(resp),
            Err(err) => Err(PSError::CommunicationError {
                msg: format!("Failed to post {}: {:?}", uri_slug, err),
            }),
        }
    }

    /// Makes a post request to the server at the specified uri slug.
    /// Form data is included if provided.
    async fn post_form<F: Serialize + ?Sized>(
        &self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        form: Option<&F>,
    ) -> PSResult<Response> {
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
        match req.send().await {
            Ok(resp) => Ok(resp),
            Err(err) => Err(PSError::CommunicationError {
                msg: format!("Failed to post {}: {:?}", uri_slug, err),
            }),
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
    async fn get_token(&self) -> PSResult<PSToken> {
        let resp_res = self
            .client
            .post(self.format_url("/ps/oauth/token"))
            .form(&self.credentials.to_map())
            .send()
            .await;

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
        Ok(PSToken::expires_in(
            token_resp.access_token,
            token_resp.expires_in,
        ))
    }

    /// Gets a new access token and stores it only if the current one is invalid.
    async fn update_token(&mut self) -> PSResult<&HeaderValue> {
        if !self.valid_token() {
            self.token = Some(self.get_token().await?);
            let header =
                HeaderValue::from_str(&format!("Bearer {}", self.token.as_ref().unwrap().token));
            match header {
                Err(err) => {
                    return Err(PSError::TokenError {
                        msg: format!("Invalid token {}", err),
                    })
                }
                Ok(header) => {
                    self.token_header = Some(header);
                }
            }
        }
        return Ok(self.token_header.as_ref().unwrap());
    }

    // Checked

    pub async fn checked_get(
        &mut self,
        uri_slug: &str,
        params: Option<Vec<(&str, &str)>>,
        headers: Option<HeaderMap<HeaderValue>>,
    ) -> PSResult<Response> {
        let token = self.update_token().await?;
        let mut new_headers = headers.unwrap_or(HeaderMap::new());
        new_headers.insert("authorization", token.clone());
        self.get(uri_slug, params, Some(new_headers)).await
    }

    async fn checked_post<T: Into<Body>>(
        &mut self,
        uri_slug: &str,
        params: Option<Vec<(&str, &str)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        body: Option<T>,
    ) -> PSResult<Response> {
        let token = self.update_token().await?;
        let mut new_headers = headers.unwrap_or(HeaderMap::new());
        new_headers.insert("authorization", token.clone());
        self.post(uri_slug, params, Some(new_headers), body).await
    }

    async fn checked_post_form<F: Serialize + ?Sized>(
        &mut self,
        uri_slug: &str,
        params: Option<&Vec<(String, String)>>,
        headers: Option<HeaderMap<HeaderValue>>,
        form: Option<&F>,
    ) -> PSResult<Response> {
        let token = self.update_token().await?;
        let mut new_headers = headers.unwrap_or(HeaderMap::new());
        new_headers.insert("authorization", token.clone());
        self.post_form(uri_slug, params, Some(new_headers), form)
            .await
    }
}
