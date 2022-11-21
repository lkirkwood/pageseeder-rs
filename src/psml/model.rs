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
    pub frag_id: String,
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

impl XRef {
    /// Returns a default xref to the given uriid.
    pub fn uriid(uriid: String) -> XRef {
        return XRef {
            uriid: None,
            docid: None,
            href: Some(uriid),
            content: String::new(),
            config: None,
            display: XRefDisplayKind::Document,
            frag_id: "default".to_string(),
            labels: Vec::new(),
            level: None,
            reverselink: true,
            reversetitle: None,
            title: None,
            xref_type: None,
        };
    }

    /// Returns a default xref to the given docid.
    pub fn docid(docid: String) -> XRef {
        return XRef {
            uriid: None,
            docid: Some(docid),
            href: None,
            content: String::new(),
            config: None,
            display: XRefDisplayKind::Document,
            frag_id: "default".to_string(),
            labels: Vec::new(),
            level: None,
            reverselink: true,
            reversetitle: None,
            title: None,
            xref_type: None,
        };
    }

    /// Returns a default xref to the given href.
    pub fn href(href: String) -> XRef {
        return XRef {
            uriid: None,
            docid: None,
            href: Some(href),
            content: String::new(),
            config: None,
            display: XRefDisplayKind::Document,
            frag_id: "default".to_string(),
            labels: Vec::new(),
            level: None,
            reverselink: true,
            reversetitle: None,
            title: None,
            xref_type: None,
        };
    }

    /// Adds the specified content to the xref and returns it.
    pub fn with_content(self, content: String) -> XRef {
        return XRef {
            uriid: self.uriid,
            docid: self.docid,
            href: self.href,
            content,
            config: self.config,
            display: self.display,
            frag_id: self.frag_id,
            labels: self.labels,
            level: self.level,
            reverselink: self.reverselink,
            reversetitle: self.reversetitle,
            title: self.title,
            xref_type: self.xref_type,
        };
    }
}

// Property

/// Property datatype attribute values.
/// Does not support custom datatypes - they will be converted to "string".
#[derive(Debug, PartialEq, Eq)]
pub enum PropertyDatatype {
    String,
    Date,
    XRef,
    Link,
    Markdown,
    Markup,
}

impl PropertyDatatype {
    pub fn from_str(string: &str) -> PropertyDatatype {
        match string {
            "date" => PropertyDatatype::Date,
            "xref" => PropertyDatatype::XRef,
            "link" => PropertyDatatype::Link,
            "markdown" => PropertyDatatype::Markdown,
            "markup" => PropertyDatatype::Markup,
            _ => PropertyDatatype::String,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Date => "date",
            Self::XRef => "xref",
            Self::Link => "link",
            Self::Markdown => "markdown",
            Self::Markup => "markup",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PropertyValue {
    String(String),
    XRef(XRef),
    Link(Vec<Event<'static>>),
    Markdown(Vec<Event<'static>>),
    Markup(Vec<Event<'static>>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Property {
    pub name: String,
    pub title: Option<String>,
    pub datatype: PropertyDatatype,
    pub multiple: bool,
    pub values: Vec<PropertyValue>,
}

// Fragments

#[derive(Debug, PartialEq, Eq)]
pub struct PropertiesFragment {
    /// ID of the fragment.
    pub id: String,
    /// Template type for the fragment.
    pub frag_type: Option<String>,
    /// Labels on this fragment.
    pub labels: Vec<String>,
    /// Properties in this fragment.
    pub properties: Vec<Property>,
}

impl PropertiesFragment {
    /// Creates a new empty fragment with the given id.
    pub fn new(id: String) -> PropertiesFragment {
        return PropertiesFragment {
            id,
            frag_type: None,
            labels: Vec::new(),
            properties: Vec::new(),
        };
    }

    /// Adds the properties to the fragment and returns it.
    pub fn with_properties(self, properties: Vec<Property>) -> PropertiesFragment {
        return PropertiesFragment {
            id: self.id,
            frag_type: self.frag_type,
            labels: self.labels,
            properties: vec![self.properties, properties]
                .into_iter()
                .flatten()
                .collect::<Vec<Property>>(),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct XRefFragment {
    /// ID of the fragment.
    pub id: String,
    /// Template type for the fragment.
    pub frag_type: Option<String>,
    /// Labels on this fragment.
    pub labels: Vec<String>,
    pub xrefs: Vec<XRef>,
}

impl XRefFragment {
    /// Creates a new empty fragment with the given id.
    pub fn new(id: String) -> XRefFragment {
        return XRefFragment {
            id,
            frag_type: None,
            labels: Vec::new(),
            xrefs: Vec::new(),
        };
    }

    /// Adds the xrefs to the fragment and returns it.
    pub fn with_xrefs(self, xrefs: Vec<XRef>) -> XRefFragment {
        return XRefFragment {
            id: self.id,
            frag_type: self.frag_type,
            labels: self.labels,
            xrefs: vec![self.xrefs, xrefs]
                .into_iter()
                .flatten()
                .collect::<Vec<XRef>>(),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Fragment {
    /// ID of the fragment.
    pub id: String,
    /// Template type for the fragment.
    pub frag_type: Option<String>,
    /// Labels on this fragment.
    pub labels: Vec<String>,
    /// Contents of the fragment.
    pub content: Vec<Event<'static>>,
}

impl Fragment {
    /// Creates a new empty fragment with the given id.
    pub fn new(id: String) -> Fragment {
        return Fragment {
            id,
            frag_type: None,
            labels: Vec::new(),
            content: Vec::new(),
        };
    }

    /// Adds the content to the fragment and returns it.
    pub fn with_content(self, content: Vec<Event<'static>>) -> Fragment {
        return Fragment {
            id: self.id,
            frag_type: self.frag_type,
            labels: self.labels,
            content: vec![self.content, content]
                .into_iter()
                .flatten()
                .collect::<Vec<Event<'static>>>(),
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Fragments {
    Normal(Fragment),
    Properties(PropertiesFragment),
    XRef(XRefFragment),
    Media(()),
}

impl Fragments {
    pub fn id(&self) -> &str {
        match self {
            Fragments::Normal(frag) => &frag.id,
            Fragments::Properties(frag) => &frag.id,
            Fragments::XRef(frag) => &frag.id,
            Fragments::Media(_frag) => todo!("Implement media frag"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Section {
    /// ID of the section.
    pub id: String,
    /// Title of the section in the UI.
    pub title: Option<String>,
    /// Title of the content.
    pub content_title: Option<String>,
    /// Whether fragments in this section can be edited in the UI.
    pub edit: bool,
    /// Whether the structure of this section can be modified.
    pub lock: bool,
    /// Whether the existing section/fragments are to be overwritten by these during upload.
    pub overwrite: bool,
    /// Fragment types this section is allowed to contain.
    pub fragment_types: Vec<String>,
    /// Fragments in this section.
    pub fragments: IndexMap<String, Fragments>,
}

impl Section {
    /// Creates a new empty fragment with the given id.
    pub fn new(id: String) -> Section {
        return Section {
            id,
            title: None,
            content_title: None,
            edit: true,
            lock: false,
            overwrite: true,
            fragment_types: Vec::new(),
            fragments: IndexMap::new(),
        };
    }

    /// Adds the given fragments to this section and returns it.
    pub fn with_fragments(self, fragments: Vec<Fragments>) -> Section {
        let mut all_frags = IndexMap::from(self.fragments);
        for frag in fragments {
            all_frags.insert(frag.id().to_string(), frag);
        }
        return Section {
            id: self.id,
            title: self.title,
            content_title: self.content_title,
            edit: self.edit,
            lock: self.lock,
            overwrite: self.overwrite,
            fragment_types: self.fragment_types,
            fragments: all_frags,
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Document {
    pub sections: IndexMap<String, Section>,
}
