use std::collections::HashMap;

use quick_xml::de;
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::{
    api::model::SearchResponse,
    error::{PSError, PSResult},
};

use super::{
    model::{
        DocumentFragment, EventType, Group, SearchResultPage, Service, Thread, Uri, UriHistory,
    },
    PSServer,
};

/// Returns an error if $resp has an error-class http return code.
/// $op is used in the error message.
macro_rules! handle_http {
    ($op:expr, $resp:ident) => {
        if !(200..300).contains(&$resp.status().as_u16()) {
            let op = $op;
            return Err(PSError::ServerError {
                msg: format!(
                    "{op} failed: {}",
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
            Err(err) => Err(PSError::ParseError {
                msg: format!("Deserialisation of xml failed [[ {} ]]: {:?}", text, err),
            }),
            Ok(obj) => Ok(obj),
        }
    }

    /// Gets a group from the server.
    pub async fn get_group(&self, name: &str) -> PSResult<Group> {
        let resp = self
            .checked_get(Service::GetGroup { group: name }, None, None)
            .await?;

        handle_http!("get group", resp);
        self.xml_from_response(resp).await
    }

    /// Gets info about a single URI.
    pub async fn get_uri(&self, member: &str, uri: &str) -> PSResult<Uri> {
        let resp = self
            .checked_get(Service::GetUri { member, uri }, None, None)
            .await?;

        handle_http!("get uri", resp);
        self.xml_from_response(resp).await
    }

    /// Gets the history of a single URI.
    pub async fn get_uri_history(&self, group: &str, uri: &str) -> PSResult<UriHistory> {
        let resp = self
            .checked_get(Service::GetUriHistory { group, uri }, None, None)
            .await?;

        handle_http!("get uri history", resp);
        self.xml_from_response(resp).await
    }

    /// Gets the history of all URIs in a group.
    /// TODO add auto pagination
    pub async fn get_uris_history(
        &self,
        group: &str,
        events: Vec<EventType>,
        mut params: HashMap<&str, &str>,
    ) -> PSResult<UriHistory> {
        let events = events
            .into_iter()
            .map(|e| e.into())
            .collect::<Vec<String>>()
            .join(",");
        params.insert("events", &events);

        let resp = self
            .checked_get(
                Service::GetUrisHistory { group },
                Some(params.into_iter().collect()),
                None,
            )
            .await?;

        handle_http!("get uris history", resp);
        self.xml_from_response(resp).await
    }

    pub async fn get_uri_fragment(
        &self,
        member: &str,
        group: &str,
        uri: &str,
        fragment: &str,
        params: HashMap<&str, &str>,
    ) -> PSResult<DocumentFragment> {
        let resp = self
            .checked_get(
                Service::GetUriFragment {
                    member,
                    group,
                    uri,
                    fragment,
                },
                Some(params.into_iter().collect()),
                None,
            )
            .await?;

        handle_http!("get uri fragment", resp);
        self.xml_from_response(resp).await
    }

    /// Returns the pageseeder thread that is exporting the URI(s).
    pub async fn uri_export(
        &self,
        member: &str,
        uri: &str,
        params: Vec<(&str, &str)>,
        // TODO find better solution for parameters (struct impl Default?)
    ) -> PSResult<Thread> {
        let resp = self
            .checked_get(Service::UriExport { member, uri }, Some(params), None)
            .await?;

        handle_http!("uri export", resp);
        self.xml_from_response(resp).await
    }

    /// Searches a group.
    /// Fetches all pages for a search if no page number is specified in params.
    /// This may result in multiple requests.
    pub async fn group_search(
        &self,
        group: &str,
        params: HashMap<&str, &str>,
    ) -> PSResult<Vec<SearchResultPage>> {
        let param_vec: Vec<(&str, &str)> = params.iter().map(|t| (*t.0, *t.1)).collect();

        let service = Service::GroupSearch { group };
        let resp = self.checked_get(service.clone(), Some(param_vec), None);

        let resp = resp.await?;
        handle_http!("group search", resp);
        let results = self
            .xml_from_response::<SearchResponse>(resp)
            .await?
            .results;

        let mut pages = vec![];
        // Fetches all pages if pagenum not specified.
        if !params.contains_key("page") {
            for page in 1..=results.total_pages {
                let page = page.to_string();
                let mut params = params.clone();

                params.insert("page", &page);
                let resp = self
                    .checked_get(
                        service.clone(),
                        Some(params.iter().map(|t| (*t.0, *t.1)).collect()),
                        None,
                    )
                    .await?;
                pages.push(
                    self.xml_from_response::<SearchResponse>(resp)
                        .await?
                        .results,
                );
            }
        }

        pages.insert(0, results);
        Ok(pages)
    }

    /// Gets the progress of a pageseeder thread.
    pub async fn thread_progress<'a>(&self, thread_id: &'a str) -> PSResult<Thread> {
        let resp = self
            .checked_get(Service::ThreadProgress { id: thread_id }, None, None)
            .await?;

        handle_http!("get thread progress", resp);
        self.xml_from_response(resp).await
    }
}
