use serde::{de::Visitor, Deserialize};

pub enum Service {
    GetGroup {
        /// Group to get.
        group: String,
    },
    UriExport {
        /// Member to export as.
        member: String,
        /// URI to export.
        uri: String,
    },
}

impl Service {
    /// Returns the url path for this service.
    /// e.g. GetGroup => /ps/service/groups/{group}
    pub fn url_path(&self) -> String {
        let path = match self {
            Self::GetGroup { group } => format!("groups/{group}"),
            Self::UriExport { member, uri } => format!("members/{member}/uris/{uri}/export"),
        };
        format!("/ps/service/{path}")
    }
}

impl Into<String> for Service {
    fn into(self) -> String {
        self.url_path()
    }
}

pub trait URLSerializable {
    fn for_url(&self) -> &str;
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

#[derive(Debug)]
pub enum PSGroupAccess {
    Public,
    Member,
}
impl<'de> Deserialize<'de> for PSGroupAccess {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        return deserializer.deserialize_str(PSGroupAccessStrVisitor);
    }
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
impl URLSerializable for Group {
    fn for_url(&self) -> &str {
        return &self.name;
    }
}

#[derive(Debug, Deserialize)]
pub struct ThreadProcessing {
    current: u64,
    total: u64,
}

#[derive(Debug, Deserialize)]
pub struct ThreadPackaging {
    current: u64,
    total: u64,
}

#[derive(Debug, Deserialize)]
pub struct Thread {
    status: String,
    processing: Option<ThreadProcessing>,
    packaging: Option<ThreadPackaging>,
}
