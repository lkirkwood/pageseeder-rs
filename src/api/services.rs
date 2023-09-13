use quick_xml::de;
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::error::{PSError, PSResult};

use super::{
    model::{Group, Service, Thread},
    PSServer,
};

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

        if !(200..300).contains(&resp.status().as_u16()) {
            return Err(PSError::ServerError {
                msg: format!(
                    "Get group {} failed: {}",
                    name,
                    resp.text()
                        .await
                        .unwrap_or("failed to get error from response".to_string())
                ),
            });
        }
        return self.xml_from_response(resp).await;
    }

    /// Returns the pageseeder thread that is exporting the URI(s).
    pub async fn uri_export(
        &mut self,
        member: &str,
        uri: &str,
        binary_metadata_only: Option<bool>,
        compare: Option<String>,
        context: Option<String>,
        excludes: Option<Vec<String>>,
        fail_on_error: Option<bool>,
        forward_depth: Option<u64>,
        includes: Option<Vec<String>>,
        load_alternates: Option<bool>,
        load_images: Option<bool>,
        metadata_only: Option<bool>,
        publication_id: Option<String>,
        version: Option<String>,
        reverse_depth: Option<u64>,
        since: Option<String>,
        with: Option<Vec<String>>,
        xref_types: Option<Vec<String>>,
    ) -> PSResult<Thread> {
        let mut params = vec![];
        // TODO get away from this (awful!)
        if binary_metadata_only.is_some() {
            params.push((
                "binary-metadata-only",
                binary_metadata_only.unwrap().to_string(),
            ));
        }
        if compare.is_some() {
            params.push(("compare", compare.unwrap().to_string()));
        }
        if context.is_some() {
            params.push(("context", context.unwrap().to_string()));
        }
        if excludes.is_some() {
            params.push(("excludes", excludes.unwrap().join(",").to_string()));
        }
        if fail_on_error.is_some() {
            params.push(("fail-on-error", fail_on_error.unwrap().to_string()));
        }
        if forward_depth.is_some() {
            params.push(("forward-depth", forward_depth.unwrap().to_string()));
        }
        if includes.is_some() {
            params.push(("includes", includes.unwrap().join(",").to_string()));
        }
        if load_alternates.is_some() {
            params.push(("load-alternates", load_alternates.unwrap().to_string()));
        }
        if load_images.is_some() {
            params.push(("load-images", load_images.unwrap().to_string()));
        }
        if metadata_only.is_some() {
            params.push(("metadata-only", metadata_only.unwrap().to_string()));
        }
        if publication_id.is_some() {
            params.push(("publicationid", publication_id.unwrap().to_string()));
        }
        if version.is_some() {
            params.push(("version", version.unwrap().to_string()));
        }
        if reverse_depth.is_some() {
            params.push(("reverse-depth", reverse_depth.unwrap().to_string()));
        }
        if since.is_some() {
            params.push(("since", since.unwrap().to_string()));
        }
        if with.is_some() {
            params.push(("with", with.unwrap().join(",").to_string()));
        }
        if xref_types.is_some() {
            params.push(("xref-types", xref_types.unwrap().join(",").to_string()));
        }

        let resp = self
            .checked_get(&Service::UriExport { member, uri }.url_path(), None, None)
            .await?;

        if !(200..300).contains(&resp.status().as_u16()) {
            return Err(PSError::ServerError {
                msg: format!(
                    "Uri Export failed: {}",
                    resp.text()
                        .await
                        .unwrap_or("failed to get error from response".to_string())
                ),
            });
        }
        return self.xml_from_response(resp).await;
    }
}
