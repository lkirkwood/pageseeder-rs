use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::text::{Heading, Image, Para};

// XRef
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum XRefDisplayKind {
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "document+manual")]
    DocumentManual,
    #[serde(rename = "document+fragment")]
    DocumentFragment,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "template")]
    Template,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum XRefKind {
    None,
    Alternate,
    Math,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// A PSML xref.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-xref.html
pub struct XRef {
    #[serde(rename = "@uriid")]
    /// Destination uriid.
    pub uriid: Option<String>,
    #[serde(rename = "@docid")]
    /// Destination docid.
    pub docid: Option<String>,
    #[serde(rename = "@href")]
    /// Destination href.#
    pub href: Option<String>,
    #[serde(rename = "$text")]
    /// Text content to display instead of xref.
    pub content: String,
    #[serde(rename = "@config")]
    /// XRef config name.
    pub config: Option<String>,
    #[serde(rename = "@display")]
    /// How target link text should be displayed.
    pub display: XRefDisplayKind,
    #[serde(rename = "@frag")]
    /// ID of fragment to link to.
    pub frag_id: String,
    #[serde(rename = "@labels")]
    /// Comma separated xref labels.
    pub labels: Option<String>,
    #[serde(rename = "@level")]
    /// Level for heading numbering of target document (1-5).
    pub level: Option<String>,
    #[serde(rename = "@reverselink")]
    /// Whether xref is bidirectional.
    pub reverselink: bool,
    #[serde(rename = "@reversetitle")]
    /// Manually entered title for reverse xref.
    pub reversetitle: Option<String>,
    #[serde(rename = "@title")]
    /// Manually entered title for xref.
    pub title: Option<String>,
    #[serde(rename = "@type")]
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
            labels: None,
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
            labels: None,
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
            labels: None,
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
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyDatatype {
    String,
    Datetime,
    XRef,
    Link,
    Markdown,
    Markup,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyValue {
    XRef(XRef),
    Link(String),
    Markdown(String),
    Markup(String),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename = "property")]
/// A PSML property.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-property.html
pub struct Property {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@title")]
    pub title: Option<String>,
    #[serde(rename = "@datatype")]
    pub datatype: Option<PropertyDatatype>,
    #[serde(rename = "@multiple")]
    pub multiple: Option<bool>,
    #[serde(rename = "@value")]
    pub attr_value: Option<String>,
    #[serde(rename = "$value", default)]
    pub values: Vec<PropertyValue>,
}

// Fragments

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// A PSML properties fragment.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-properties-fragment.html
pub struct PropertiesFragment {
    #[serde(rename = "@id")]
    /// ID of the fragment.
    pub id: String,
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Template type for the fragment.
    pub frag_type: Option<String>,
    #[serde(rename = "@labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels on this fragment.
    pub labels: Option<String>,
    #[serde(rename = "property", default)]
    /// Properties in this fragment.
    pub properties: Vec<Property>,
    #[serde(flatten)]
    /// Other attributes on this fragment.
    pub attrs: HashMap<String, String>,
}

impl PropertiesFragment {
    /// Creates a new empty fragment with the given id.
    pub fn new(id: String) -> PropertiesFragment {
        return PropertiesFragment {
            id,
            frag_type: None,
            labels: None,
            properties: vec![],
            attrs: HashMap::new(),
        };
    }

    /// Adds the properties to the fragment and returns it.
    pub fn with_properties(self, properties: Vec<Property>) -> PropertiesFragment {
        PropertiesFragment {
            id: self.id,
            frag_type: self.frag_type,
            labels: self.labels,
            properties: vec![self.properties, properties]
                .into_iter()
                .flatten()
                .collect::<Vec<Property>>(),
            attrs: self.attrs,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BlockXRef {
    #[serde(rename = "@archived")]
    pub archived: bool,

    #[serde(rename = "@display")]
    pub display: Option<XRefDisplayKind>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// A PSML xref fragment.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-xref-fragment.html
pub struct XRefFragment {
    #[serde(rename = "@id")]
    /// ID of the fragment.
    pub id: String,
    #[serde(rename = "@type")]
    /// Template type for the fragment.
    pub frag_type: Option<String>,
    #[serde(rename = "@labels")]
    /// Labels on this fragment.
    pub labels: String,
    #[serde(rename = "blockxref", default)]
    pub xrefs: Vec<BlockXRef>,
    #[serde(flatten)]
    /// Other attributes on this fragment.
    pub attrs: HashMap<String, String>,
}

impl XRefFragment {
    /// Creates a new empty fragment with the given id.
    pub fn new(id: String) -> XRefFragment {
        return XRefFragment {
            id,
            frag_type: None,
            labels: String::new(),
            xrefs: Vec::new(),
            attrs: HashMap::new(),
        };
    }

    /// Adds the xrefs to the fragment and returns it.
    pub fn with_xrefs(self, xrefs: Vec<BlockXRef>) -> XRefFragment {
        return XRefFragment {
            id: self.id,
            frag_type: self.frag_type,
            labels: self.labels,
            xrefs: vec![self.xrefs, xrefs]
                .into_iter()
                .flatten()
                .collect::<Vec<BlockXRef>>(),
            attrs: self.attrs,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FragmentContent {
    #[serde(rename = "$text")]
    Text(String),
    Heading(Heading),
    Block {
        #[serde(rename = "$value", default)]
        child: Vec<FragmentContent>,
    },
    BlockXRef(BlockXRef),
    Para(Para),
    Preformat {
        #[serde(rename = "$value", default)]
        child: Vec<FragmentContent>,
    },
    Image(Image),
    Table(()), // TODO impl fragment table
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// A PSML fragment.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-fragment.html
pub struct Fragment {
    #[serde(rename = "@id")]
    /// ID of the fragment.
    pub id: String,
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Template type for the fragment.
    pub frag_type: Option<String>,
    #[serde(rename = "@labels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels on this fragment.
    pub labels: Option<String>,
    #[serde(rename = "$value", default)]
    /// Contents of the fragment.
    pub content: Vec<FragmentContent>,
    // #[serde(flatten)]
    // /// Other attributes.
    // pub attrs: HashMap<String, String>,

    // TODO ^ wait till $value and flatten can be used together ^
    // https://github.com/tafia/quick-xml/issues/326
}

impl Fragment {
    /// Creates a new empty fragment with the given id.
    pub fn new(id: String) -> Fragment {
        return Fragment {
            id,
            frag_type: None,
            labels: None,
            content: vec![],
            // attrs: HashMap::new(),
        };
    }

    /// Adds the content to the fragment and returns it.
    pub fn with_content(mut self, content: Vec<FragmentContent>) -> Fragment {
        // pub fn with_content(mut self, content: Vec<String>) -> Fragment {
        self.content.extend(content);
        Fragment {
            id: self.id,
            frag_type: self.frag_type,
            labels: self.labels,
            content: self.content,
            // attrs: self.attrs,
        }
    }
}

// Section

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SectionContent {
    #[serde(rename = "fragment")]
    Fragment(Fragment),
    #[serde(rename = "properties-fragment")]
    PropertiesFragment(PropertiesFragment),
    #[serde(rename = "xref-fragment")]
    XRefFragment(XRefFragment),
    #[serde(rename = "media-fragment")]
    Media(()),
    #[serde(rename = "title")]
    Title {
        #[serde(rename = "$text", default)]
        text: String,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// A PSML Section.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-section.html
pub struct Section {
    #[serde(rename = "@id")]
    /// ID of the section.
    pub id: String,
    #[serde(rename = "@title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Title of the section in the UI.
    pub title: Option<String>,
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Title of the content.
    pub content_title: Option<String>,
    #[serde(rename = "@edit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether fragments in this section can be edited in the UI.
    pub edit: Option<bool>,
    #[serde(rename = "@lock")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the structure of this section can be modified.
    pub lock: Option<bool>,
    #[serde(rename = "@overwrite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the existing section/fragments are to be overwritten by these during upload.
    pub overwrite: Option<bool>,
    #[serde(rename = "@fragmenttype")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Fragment types this section is allowed to contain.
    pub fragment_types: Option<String>,
    #[serde(rename = "$value", default)]
    /// Fragments in this section.
    pub content: Vec<SectionContent>,
}

impl Section {
    /// Creates a new empty fragment with the given id.
    pub fn new(id: String) -> Section {
        return Section {
            id,
            title: None,
            content_title: None,
            edit: Some(true),
            lock: Some(false),
            overwrite: Some(true),
            fragment_types: None,
            content: Vec::new(),
        };
    }
}

// Document

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// Describes the publication.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-publication.html
pub struct Publication {
    // #[serde(rename = "@id")]
    /// Publication ID.
    pub id: String,
    #[serde(rename = "@type")]
    /// Publication type.
    pub pub_type: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// The <description> element is used to provide a short text-only description.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-description.html
pub struct Description {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// Metadata about this URI.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-uri.html
pub struct URIDescriptor {
    // Attributes
    #[serde(rename = "@docid")]
    /// Docid of this document.
    pub docid: Option<String>,
    #[serde(rename = "@documenttype")]
    /// Type of the document.
    pub doc_type: Option<String>,
    #[serde(rename = "@title")]
    /// Title for the document.
    pub title: Option<String>,
    #[serde(rename = "@folder")]
    /// If true, this is a folder.
    pub folder: Option<bool>,

    // Elements
    /// Description of the document.
    pub description: Option<Description>,
    /// Labels on the document.
    pub labels: Option<Labels>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// Wrapper for metadata about the document.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-documentinfo.html
pub struct DocumentInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// URI descriptor.
    pub uri: Option<URIDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Publication descriptor.
    pub publication: Option<Publication>,
}

// TODO change this to vec of strings with custom deserializer.
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// A comma-separated list of label values for a document, note or fragment.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-labels.html
pub struct Labels {
    #[serde(rename = "$value")]
    pub value: String,
}

/// Previous document content if different from current (used when doing a compare).
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-content.html
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Content {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// The note on the last notification of the fragment.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-note.html
pub struct Note {
    // #[serde(rename = "@id")]
    pub id: Option<String>,
    #[serde(rename = "@title")]
    /// Title of the note.
    pub title: Option<String>,
    #[serde(rename = "@modified")]
    /// Date and time this note was modified.
    pub modified: String,
    /// Labels on this note.
    pub labels: Labels,
    /// Content in this note.
    pub content: Content,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// The notes on the last notification of the fragment.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-notes.html
pub struct Notes {
    pub notes: Vec<Note>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
/// Metadata relating to a fragment.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-locator.html
pub struct Locator {
    #[serde(rename = "fragment")]
    /// ID of the fragment.
    pub fragment_id: Option<String>,
    /// Notes on this fragment.
    pub notes: Option<Notes>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FragmentInfo {
    pub value: Vec<Locator>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentLevel {
    Metadata,
    Portable,
    Processed,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename = "document")]
/// A PSML document.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-document.html
pub struct Document {
    #[serde(rename = "documentinfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Metadata about the document.
    pub doc_info: Option<DocumentInfo>,
    #[serde(rename = "fragmentinfo", default)]
    /// Fragment metadata
    pub frag_info: Vec<Locator>,
    #[serde(rename = "section")]
    /// Sections in the document.
    pub sections: Vec<Section>,
    #[serde(rename = "@type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    #[serde(rename = "@edit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<bool>,
    #[serde(rename = "@level")]
    pub level: DocumentLevel,
    #[serde(rename = "@lockstructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lockstructure: Option<bool>,
}

// TODO impl toc index
