use indexmap::IndexMap;
use quick_xml::events::Event;

use crate::error::{PSError, PSResult};

// XRef
#[derive(Debug, PartialEq, Eq)]
pub enum XRefDisplayKind {
    Document,
    DocumentManual,
    DocumentFragment,
    Manual,
    Template,
}

impl XRefDisplayKind {
    /// Returns xref display kind from string.
    pub fn from_str(string: &str) -> PSResult<XRefDisplayKind> {
        match string {
            "document" => Ok(XRefDisplayKind::Document),
            "document+manual" => Ok(XRefDisplayKind::DocumentManual),
            "document+fragment" => Ok(XRefDisplayKind::DocumentFragment),
            "manual" => Ok(XRefDisplayKind::Manual),
            "template" => Ok(XRefDisplayKind::Template),
            other => Err(PSError::ParseError {
                msg: format!("Unknown exref display kind: {}", other),
            }),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::DocumentManual => "document+manual",
            Self::DocumentFragment => "document+fragment",
            Self::Manual => "manual",
            Self::Template => "template",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum XRefKind {
    None,
    Alternate,
    Math,
}

impl XRefKind {
    pub fn from_str(string: &str) -> PSResult<XRefKind> {
        match string {
            "none" => Ok(XRefKind::None),
            "alternate" => Ok(XRefKind::Alternate),
            "math" => Ok(XRefKind::Math),
            other => Err(PSError::ParseError {
                msg: format!("Unknown xref type {}", other),
            }),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Alternate => "alternate",
            Self::Math => "math",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct XRef {
    /// Destination uriid.
    pub uriid: Option<String>,
    /// Destination docid.
    pub docid: Option<String>,
    /// Destination href.#
    pub href: Option<String>,
    /// Text content to display instead of xref.
    pub content: String,
    /// XRef config name.
    pub config: Option<String>,
    /// How target link text should be displayed.
    pub display: XRefDisplayKind,
    /// ID of fragment to link to.
    pub frag_id: Option<String>,
    /// Comma separated xref labels.
    pub labels: Vec<String>,
    /// Level for heading numbering of target document (1-5).
    pub level: Option<String>,
    /// Whether xref is bidirectional.
    pub reverselink: bool,
    /// Manually entered title for reverse xref.
    pub reversetitle: Option<String>,
    /// Manually entered title for xref.
    pub title: Option<String>,
    /// XRef type
    pub xref_type: Option<XRefKind>,
}

// Property

#[derive(Debug, PartialEq, Eq)]
pub enum PropertyValue {
    XRef(XRef),
    String(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Property {
    pub name: String,
    pub title: Option<String>,
    pub value: Vec<PropertyValue>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PropertiesFragment {
    pub id: String,
    pub title: Option<String>,
    pub properties: Vec<Property>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct XRefFragment {
    pub id: String,
    pub title: Option<String>,
    pub xrefs: Vec<XRef>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fragment {
    pub id: String,
    pub title: Option<String>,
    pub content: Vec<Event<'static>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Fragments {
    Normal(Fragment),
    Properties(PropertiesFragment),
    XRef(XRefFragment),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    pub fragments: IndexMap<String, Fragments>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    pub sections: IndexMap<String, Section>,
}
