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

impl From<Service<'_>> for String {
    fn from(val: Service<'_>) -> Self {
        val.url_path()
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
            .unwrap_or_else(|| panic!("Group name has no '-': {}", self.name))
            .1;
    }
}

// Export

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
    #[serde(rename = "@status")]
    pub status: String,
    pub processing: Option<ThreadProcessing>,
    pub packaging: Option<ThreadPackaging>,
}

// Search

#[derive(Debug, Deserialize)]
pub struct SearchResultField {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$text")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResult {
    #[serde(rename = "field")]
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
    pub first_result: u64,
    #[serde(rename = "@last-result")]
    pub last_result: u64,
    #[serde(rename = "result")]
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub results: SearchResultPage,
}
