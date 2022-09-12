use std::fmt::Display;

use serde::{de::Visitor, Deserialize};

use crate::error::PSError;

pub enum HttpScheme {
    Http,
    Https,
}

impl HttpScheme {
    fn to_str(&self) -> &'static str {
        match self {
            Self::Http => "http",
            Self::Https => "https",
        }
    }
}

impl Display for HttpScheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

pub enum ServicePath {
    GetGroup,
}

impl ServicePath {
    /// Returns a string that can be used with format! to return the uri slug
    /// for this service.
    /// e.g. GetGroup => "/ps/service/groups/{}"
    pub fn url_path(&self) -> String {
        let path = match self {
            Self::GetGroup => "groups/{}",
        };
        return format!("/ps/service/{}", path);
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

pub struct PSGroup {
    pub id: u32,
    pub name: String,
    pub owner: String,
    pub description: String,
    pub access: PSGroupAccess,
}
impl PSGroup {
    pub fn short_name(&self) -> &str {
        return self
            .name
            .rsplit_once('-')
            .expect(&format!("Group name has no '-': {}", self.name))
            .1;
    }
}
impl URLSerializable for PSGroup {
    fn for_url(&self) -> &str {
        return &self.name;
    }
}
