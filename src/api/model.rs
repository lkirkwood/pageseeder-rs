use serde::{de::Visitor, Deserialize};

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
}

impl Service<'_> {
    /// Returns the url path for this service.
    /// e.g. GetGroup => /ps/service/groups/{group}
    pub fn url_path(&self) -> String {
        let path = match self {
            Self::GetGroup { group } => format!("groups/{group}"),
            Self::UriExport { member, uri } => format!("members/{member}/uris/{uri}/export"),
            Self::GroupSearch { group } => format!("groups/{group}/search"),
        };
        format!("/ps/service/{path}")
    }
}

impl Into<String> for Service<'_> {
    fn into(self) -> String {
        self.url_path()
    }
}

// Group

struct PSGroupAccessStrVisitor;
impl<'de> Visitor<'de> for PSGroupAccessStrVisitor {
    type Value = PSGroupAccess;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("one of: \"public\", \"member\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "public" => Ok(PSGroupAccess::Public),
            "member" => Ok(PSGroupAccess::Member),
            _ => Err(E::custom(format!(
                "Server sent unknown PSGroupAccess type: {}",
                v
            ))),
        }
    }
}

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
            .expect(&format!("Group name has no '-': {}", self.name))
            .1;
    }
}

// Export

#[derive(Debug, Deserialize)]
#[serde(rename = "processing")]
pub struct ThreadProcessing {
    current: u64,
    total: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "packaging")]
pub struct ThreadPackaging {
    current: u64,
    total: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "thread")]
pub struct Thread {
    status: String,
    processing: Option<ThreadProcessing>,
    packaging: Option<ThreadPackaging>,
}

// Search

#[derive(Debug, Deserialize)]
#[serde(rename = "field")]
pub struct SearchResultField {
    name: String,
    #[serde(rename = "$text")]
    value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "result")]
pub struct SearchResult {
    #[serde(flatten)]
    fields: Vec<SearchResultField>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "results")]
pub struct SearchResultPage {
    page: u64,
    #[serde(rename = "page-size")]
    page_size: u64,
    #[serde(rename = "total-pages")]
    total_pages: u64,
    #[serde(rename = "total-results")]
    total_results: u64,
    #[serde(rename = "first-result")]
    first_result: u64,
    #[serde(rename = "last-result")]
    last_result: u64,
    #[serde(flatten)]
    results: Vec<SearchResult>,
}
