use std::fmt::Display;

use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum Service<'a> {
    GetGroup {
        /// Group to get.
        group: &'a str,
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
}

impl Service<'_> {
    /// Returns the url path for this service.
    /// e.g. GetGroup => /ps/service/groups/{group}
    pub fn url_path(&self) -> String {
        let path = match self {
            Self::GetGroup { group } => format!("groups/{group}"),
            Self::UriExport { member, uri } => format!("members/{member}/uris/{uri}/export"),
            Self::GroupSearch { group } => format!("groups/{group}/search"),
            Self::ThreadProgress { id } => format!("threads/{id}/progress"),
        };
        format!("/ps/service/{path}")
    }
}

impl From<Service<'_>> for String {
    fn from(val: Service<'_>) -> Self {
        val.url_path()
    }
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
    pub id: u32,
    pub name: String,
    pub owner: String,
    pub description: String,
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
        match self {
            Self::Initialised | Self::InProgress => true,
            _ => false,
        }
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
