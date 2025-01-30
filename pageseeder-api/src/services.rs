use std::collections::HashMap;

use quick_xml::de;
use reqwest::Response;
use serde::de::DeserializeOwned;

use crate::model::{LoadClear, PSError, PSResult, SearchResponse, VersionCreation};

use super::{
    model::{
        DocumentFragment, Error, EventType, FragmentCreation, Group, LoadStart, LoadUnzip,
        SearchResultPage, Service, Thread, Upload, Uri, UriHistory,
    },
    PSServer,
};

impl PSServer {
    async fn handle_http<'de, T: DeserializeOwned>(&self, op: &str, resp: Response) -> PSResult<T> {
        if !(200..300).contains(&resp.status().as_u16()) {
            let err: Error = de::from_str(&resp.text().await?)?;
            Err(PSError::ApiError {
                id: err.id,
                req: op.to_string(),
                msg: err.message,
            })
        } else {
            Ok(de::from_str(&resp.text().await?)?)
        }
    }

    /// Gets a group from the server.
    pub async fn get_group(&self, name: &str) -> PSResult<Group> {
        let resp = self
            .checked_get(Service::GetGroup { group: name }, None, None)
            .await?;

        self.handle_http("get group", resp).await
    }

    /// Gets info about a single URI.
    pub async fn get_uri(&self, member: &str, uri: &str) -> PSResult<Uri> {
        let resp = self
            .checked_get(Service::GetUri { member, uri }, None, None)
            .await?;

        self.handle_http("get uri", resp).await
    }

    /// Gets the history of a single URI.
    pub async fn get_uri_history(&self, group: &str, uri: &str) -> PSResult<UriHistory> {
        let resp = self
            .checked_get(Service::GetUriHistory { group, uri }, None, None)
            .await?;

        self.handle_http("get uri history", resp).await
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

        self.handle_http("get uris history", resp).await
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

        self.handle_http("get uri fragment", resp).await
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

        self.handle_http("uri export", resp).await
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
        let resp = self
            .checked_get(service.clone(), Some(param_vec), None)
            .await?;

        let results = self
            .handle_http::<SearchResponse>("group search", resp)
            .await?
            .results;

        let mut pages = vec![];
        // Fetches all pages if pagenum not specified.
        if !params.contains_key("page") {
            for page in 2..=results.total_pages {
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
                pages.push(de::from_str(&resp.text().await?)?);
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

        self.handle_http("get thread progress", resp).await
    }

    pub async fn put_uri_fragment(
        &self,
        member: &str,
        group: &str,
        uri: &str,
        fragment: &str,
        content: String,
        params: Option<Vec<(&str, &str)>>,
    ) -> PSResult<FragmentCreation> {
        let resp = self
            .checked_put(
                Service::PutUriFragment {
                    member,
                    group,
                    uri,
                    fragment,
                },
                params,
                None,
                Some(content),
            )
            .await?;

        self.handle_http("put uri fragment", resp).await
    }

    pub async fn add_uri_fragment(
        &self,
        member: &str,
        group: &str,
        uri: &str,
        content: &str,
        mut params: HashMap<&str, &str>,
    ) -> PSResult<FragmentCreation> {
        params.insert("content", content);

        let resp = self
            .checked_post(
                Service::AddUriFragment { member, group, uri },
                Some(params.into_iter().collect()),
                None,
                Option::<&[u8]>::None,
            )
            .await?;

        self.handle_http("add uri fragment", resp).await
    }

    pub async fn upload(
        &self,
        group: &str,
        filename: &str,
        file: Vec<u8>,
        mut params: HashMap<&str, &str>,
    ) -> PSResult<Upload> {
        params.insert("group", group);
        params.insert("filename", filename);

        let resp = self
            .checked_put(
                Service::Upload,
                Some(params.into_iter().collect()),
                None,
                Some(file),
            )
            .await?;

        self.handle_http("upload", resp).await
    }

    pub async fn clear_loading_zone(&self, member: &str, group: &str) -> PSResult<LoadClear> {
        let resp = self
            .checked_post(
                Service::ClearLoadingZone { member, group },
                None,
                None,
                Option::<&[u8]>::None,
            )
            .await?;

        self.handle_http("clear loading zone", resp).await
    }

    pub async fn unzip_loading_zone(
        &self,
        member: &str,
        group: &str,
        path: &str,
        mut params: HashMap<&str, &str>,
    ) -> PSResult<LoadUnzip> {
        params.insert("path", path);

        let resp = self
            .checked_post(
                Service::UnzipLoadingZone { member, group },
                Some(params.into_iter().collect()),
                None,
                Option::<&[u8]>::None,
            )
            .await?;

        self.handle_http("unzip loading zone content", resp).await
    }

    pub async fn start_loading(
        &self,
        member: &str,
        group: &str,
        params: HashMap<&str, &str>,
    ) -> PSResult<LoadStart> {
        let resp = self
            .checked_post(
                Service::StartLoading { member, group },
                Some(params.into_iter().collect()),
                None,
                Option::<&[u8]>::None,
            )
            .await?;

        self.handle_http("start loading the loading zone content", resp)
            .await
    }

    /// Downloads a member resource like the result of an export thread.
    pub async fn download_member_resource(&self, group: &str, filename: &str) -> PSResult<Vec<u8>> {
        let resp = self
            .checked_get(Service::DownloadMemberResource { group, filename }, None, None)
            .await?;

        Ok(resp.bytes().await?.to_vec())
    }

    /// Creates a version for a URI.
    pub async fn create_uri_version(
        &self,
        member: &str,
        group: &str,
        uri: &str,
        name: &str,
        mut params: HashMap<&str, &str>,
    ) -> PSResult<VersionCreation> {
        params.insert("name", name);

        let resp = self
            .checked_post(
                Service::CreateUriVersion { member, group, uri },
                Some(params.into_iter().collect()),
                None,
                Option::<&[u8]>::None,
            )
            .await?;

        self.handle_http("create version", resp).await
    }
}
