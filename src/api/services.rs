use std::collections::HashMap;

use quick_xml::de;
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::error::{PSError, PSResult};

use super::{
    model::{Group, SearchResultPage, Service, Thread},
    PSServer,
};

/// Returns an error if $resp has an error-class http return code.
/// $op is used in the error message.
macro_rules! handle_http {
    ($op:expr, $resp:ident) => {
        if !(200..300).contains(&$resp.status().as_u16()) {
            return Err(PSError::ServerError {
                msg: format!(
                    "$op failed: {}",
                    $resp
                        .text()
                        .await
                        .unwrap_or("failed to get error from response".to_string())
                ),
            });
        }
    };
}

impl PSServer {
    /// Returns a type from the xml content of a response.
    async fn xml_from_response<T: DeserializeOwned>(&self, resp: Response) -> PSResult<T> {
        let text = match resp.text().await {
            Err(err) => {
                return Err(PSError::ParseError {
                    msg: format!("Failed to decode server response: {:?}", err),
                })
            }
            Ok(_text) => _text,
        };
        match de::from_str(&text) {
            Err(err) => {
                return Err(PSError::ParseError {
                    msg: format!("Deserialisation of xml failed [[ {} ]]: {:?}", text, err),
                })
            }
            Ok(obj) => return Ok(obj),
        }
    }

    /// Gets a group from the server.
    pub async fn get_group(&mut self, name: &str) -> PSResult<Group> {
        let resp = self
            .checked_get(&Service::GetGroup { group: name }.url_path(), None, None)
            .await?;

        handle_http!("get group", resp);
        return self.xml_from_response(resp).await;
    }

    /// Returns the pageseeder thread that is exporting the URI(s).
    pub async fn uri_export(
        &mut self,
        member: &str,
        uri: &str,
        params: &Vec<(&str, &str)>,
        // TODO find better solution for parameters (struct impl Default?)
    ) -> PSResult<Thread> {
        let resp = self
            .checked_get(
                &Service::UriExport { member, uri }.url_path(),
                Some(params),
                None,
            )
            .await?;

        handle_http!("uri export", resp);
        return self.xml_from_response(resp).await;
    }
        return self.xml_from_response(resp).await;
    }
}
