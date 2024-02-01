use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use psml::model::{Fragments, Locator};

#[derive(Debug, Clone)]
pub enum Service<'a> {
    GetGroup {
        /// Group to get.
        group: &'a str,
    },
    GetUri {
        /// Member to get details as.
        member: &'a str,
        /// URI to get.
        uri: &'a str,
    },
    GetUriHistory {
        /// Group URI is in.
        group: &'a str,
        /// URI to get history for.
        uri: &'a str,
    },
    GetUrisHistory {
        /// Group URIs are in.
        group: &'a str,
    },
    GetUriFragment {
        /// Member to get fragment as.
        member: &'a str,
        /// Group URI is in.
        group: &'a str,
        /// URI of document to get fragment from.
        uri: &'a str,
        /// ID of the fragment to return.
        fragment: &'a str,
    },
    UriExport {
        /// Member to export as.
        member: &'a str,
        /// URI to export.
        uri: &'a str,
    },
    GroupSearch {
        /// Group to search.
        group: &'a str,
    },
    ThreadProgress {
        /// Thread ID to get progress for.
        id: &'a str,
    },
    PutUriFragment {
        /// Member to put fragment as.
        member: &'a str,
        /// Group URI is in.
        group: &'a str,
        /// URI of document.
        uri: &'a str,
        /// ID of fragment to put.
        fragment: &'a str,
    },
    AddUriFragment {
        /// Member to add fragment as.
        member: &'a str,
        /// Group URI is in.
        group: &'a str,
        /// URI of document.
        uri: &'a str,
    },
    Upload,
    UnzipLoadingZone {
        /// Member owning the loading zone.
        member: &'a str,
        /// Group the loading zone is in.
        group: &'a str,
    },
    StartLoading {
        /// Member owning the loading zone.
        member: &'a str,
        /// Group the loading zone is in.
        group: &'a str,
    },
}

impl Service<'_> {
    /// Returns the url path for this service.
    /// e.g. GetGroup => /ps/service/groups/{group}
    pub fn url_path(&self) -> String {
        let path = match self {
            Self::GetGroup { group } => format!("groups/{group}"),
            Self::GetUri { member, uri } => format!("members/{member}/uris/{uri}"),
            Self::GetUriHistory { group, uri } => format!("groups/{group}/uris/{uri}/history"),
            Self::GetUrisHistory { group } => format!("groups/{group}/uris/history"),
            Self::GetUriFragment {
                member,
                group,
                uri,
                fragment,
            } => format!("members/{member}/groups/{group}/uris/{uri}/fragments/{fragment}"),
            Self::UriExport { member, uri } => format!("members/{member}/uris/{uri}/export"),
            Self::GroupSearch { group } => format!("groups/{group}/search"),
            Self::ThreadProgress { id } => format!("threads/{id}/progress"),
            Self::PutUriFragment {
                member,
                group,
                uri,
                fragment,
            } => format!("members/{member}/groups/{group}/uris/{uri}/fragments/{fragment}"),
            Self::AddUriFragment { member, group, uri } => {
                format!("members/{member}/groups/{group}/uris/{uri}/fragments")
            }
            Self::Upload => return format!("/ps/servlet/upload"),
            Self::UnzipLoadingZone { member, group } => {
                format!("members/{member}/groups/{group}/loadingzone/unzip")
            }
            Self::StartLoading { member, group } => {
                format!("members/{member}/groups/{group}/loadingzone/start")
            }
        };
        format!("/ps/service/{path}")
    }
}

impl From<Service<'_>> for String {
    fn from(val: Service<'_>) -> Self {
        val.url_path()
    }
}

// Error

#[derive(Debug, Deserialize)]
pub struct Error {
    #[serde(rename = "@id")]
    pub id: String,
    pub request: String,
    pub message: String,
}

// Group

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PSGroupAccess {
    Public,
    Member,
}

#[derive(Debug, Deserialize)]
pub struct Group {
    #[serde(rename = "@id")]
    pub id: u32,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@owner")]
    pub owner: String,
    #[serde(rename = "@description")]
    pub description: String,
    #[serde(rename = "@access")]
    pub access: PSGroupAccess,
}

impl Group {
    pub fn short_name(&self) -> &str {
        return self
            .name
            .rsplit_once('-')
            .unwrap_or_else(|| panic!("Group name has no '-': {}", self.name))
            .1;
    }
}

#[derive(Debug, Deserialize)]
pub struct Uri {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@scheme")]
    pub scheme: String,
    #[serde(rename = "@host")]
    pub host: String,
    #[serde(rename = "@port")]
    pub port: String,
    #[serde(rename = "@path")]
    pub path: String,
    #[serde(rename = "@decodedpath")]
    pub decodedpath: String,
    #[serde(rename = "@external")]
    pub external: bool,
    #[serde(rename = "@archived")]
    pub archived: Option<bool>,
    #[serde(rename = "@folder")]
    pub folder: Option<bool>,
    #[serde(rename = "@docid")]
    pub docid: Option<String>,
    #[serde(rename = "@mediatype")]
    pub mediatype: Option<String>,
    #[serde(rename = "@documenttype")]
    pub documenttype: Option<String>,
    #[serde(rename = "@title")]
    pub title: Option<String>,
    #[serde(rename = "@created")]
    pub created: Option<DateTime<Utc>>,
    #[serde(rename = "@modified")]
    pub modified: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    Upload,
    Creation,
    Move,
    Modification,
    Structure,
    Workflow,
    Version,
    Edit,
    Draft,
    Note,
    Xref,
    Image,
    Comment,
    Task,
}

impl From<EventType> for String {
    fn from(val: EventType) -> Self {
        match val {
            EventType::Upload => "upload ".to_string(),
            EventType::Creation => "creation ".to_string(),
            EventType::Move => "move ".to_string(),
            EventType::Modification => "modification ".to_string(),
            EventType::Structure => "structure ".to_string(),
            EventType::Workflow => "workflow ".to_string(),
            EventType::Version => "version ".to_string(),
            EventType::Edit => "edit ".to_string(),
            EventType::Draft => "draft ".to_string(),
            EventType::Note => "note ".to_string(),
            EventType::Xref => "xref ".to_string(),
            EventType::Image => "image ".to_string(),
            EventType::Comment => "comment ".to_string(),
            EventType::Task => "task ".to_string(),
        }
    }
}

// TODO implement if not PS member + other event children
// see: https://dev.pageseeder.com/api/element_reference/element_author.html
#[derive(Debug, Deserialize)]
pub struct Author {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@firstname")]
    pub firstname: String,
    #[serde(rename = "@surname")]
    pub surname: String,
    #[serde(rename = "@username")]
    pub username: String,
    #[serde(rename = "@status")]
    pub status: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventContent {
    Author(Author),
    Labels(()),
    Change(()),
    Uri(Uri),
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@datetime")]
    pub datetime: Option<DateTime<Utc>>,
    #[serde(rename = "@type")]
    pub event_type: EventType,
    #[serde(rename = "@fragment")]
    pub fragment: Option<String>,
    #[serde(rename = "@title")]
    pub title: Option<String>,
    #[serde(rename = "@uriid")]
    pub uriid: Option<String>,
    #[serde(rename = "@targetfragment")]
    pub targetfragment: Option<String>,
    #[serde(rename = "@version")]
    pub version: Option<String>,
    #[serde(rename = "$value", default)]
    pub content: Vec<EventContent>,
}

#[derive(Debug, Deserialize)]
pub struct UriHistory {
    #[serde(rename = "@events")]
    pub event_types: String,
    #[serde(rename = "$value")]
    pub events: Vec<Event>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "document-fragment")]
pub struct DocumentFragment {
    pub locator: Option<Locator>,
    #[serde(rename = "$value")]
    pub fragment: Option<Fragments>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "fragment-creation")]
pub struct FragmentCreation {
    #[serde(rename = "@unresolved-xrefs")]
    pub unresolved_xrefs: Option<bool>,
    #[serde(rename = "document-fragment")]
    pub document_fragment: DocumentFragment,
}

// Export

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThreadStatus {
    Initialised,
    InProgress,
    Error,
    Warning,
    Cancelled,
    Failed,
    Completed,
}

impl ThreadStatus {
    /// Returns true if thread is still running.
    pub fn running(&self) -> bool {
        matches!(self, Self::Initialised | Self::InProgress)
    }
}

impl Display for ThreadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initialised => write!(f, "initialised"),
            Self::InProgress => write!(f, "inprogress"),
            Self::Error => write!(f, "error"),
            Self::Warning => write!(f, "warning"),
            Self::Cancelled => write!(f, "cancelled"),
            Self::Failed => write!(f, "failed"),
            Self::Completed => write!(f, "completed"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "zip")]
pub struct ThreadZip {
    #[serde(rename = "$text")]
    pub filename: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "message")]
pub struct ThreadMessage {
    #[serde(rename = "$text")]
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "processing")]
pub struct ThreadProcessing {
    #[serde(rename = "@current")]
    pub current: u64,
    #[serde(rename = "@total")]
    pub total: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "packaging")]
pub struct ThreadPackaging {
    #[serde(rename = "@current")]
    pub current: u64,
    #[serde(rename = "@total")]
    pub total: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "thread")]
pub struct Thread {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@username")]
    pub username: String,
    #[serde(rename = "@groupid")]
    pub groupid: String,
    #[serde(rename = "@status")]
    pub status: ThreadStatus,
    pub processing: Option<ThreadProcessing>,
    pub packaging: Option<ThreadPackaging>,
    pub zip: Option<ThreadZip>,
    pub message: Option<ThreadMessage>,
}

// Search

#[derive(Debug, Deserialize)]
pub struct SearchResultField {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text", default)]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "field", default)]
    pub fields: Vec<SearchResultField>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResultPage {
    #[serde(rename = "@page")]
    pub page: u64,
    #[serde(rename = "@page-size")]
    pub page_size: u64,
    #[serde(rename = "@total-pages")]
    pub total_pages: u64,
    #[serde(rename = "@total-results")]
    pub total_results: u64,
    #[serde(rename = "@first-result")]
    pub first_result: Option<u64>,
    #[serde(rename = "@last-result")]
    pub last_result: Option<u64>,
    #[serde(rename = "result", default)]
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub results: SearchResultPage,
}

// Uploading

#[derive(Debug, Deserialize)]
pub struct File {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@path")]
    pub path: String,
    #[serde(rename = "@type")]
    pub ftype: String,
}

#[derive(Debug, Deserialize)]
pub struct Upload {
    #[serde(rename = "@member")]
    pub member: String,
    #[serde(rename = "@uploadid")]
    pub uploadid: Option<String>,
    #[serde(rename = "@status")]
    pub status: Option<String>,
    #[serde(rename = "@max-workflow-notifications")]
    pub max_workflow_notifications: u8,
    pub message: Option<String>,
    pub uri: Option<Uri>,
    pub file: Option<File>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "load-unzip")]
pub struct LoadUnzip {
    pub thread: Thread,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "load-start")]
pub struct LoadStart {
    pub thread: Thread,
}
