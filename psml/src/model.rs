use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::HashMap};

use super::text::{Alignment, Heading, Image, Para};

// XRef
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum XRefKind {
    None,
    Alternate,
    Math,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BlockXRefKind {
    None,
    Alternate,
    Math,
    Embed,
    Transclude,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
/// A PSML xref.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-xref.html
pub struct XRef {
    #[serde(rename = "@uriid", skip_serializing_if = "Option::is_none")]
    /// Destination uriid.
    pub uriid: Option<String>,
    #[serde(rename = "@docid", skip_serializing_if = "Option::is_none")]
    /// Destination docid.
    pub docid: Option<String>,
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    /// Destination href.#
    pub href: Option<String>,
    #[serde(rename = "$text")]
    /// Text content to display instead of xref.
    pub content: String,
    #[serde(rename = "@config", skip_serializing_if = "Option::is_none")]
    /// XRef config name.
    pub config: Option<String>,
    #[serde(rename = "@display")]
    /// How target link text should be displayed.
    pub display: XRefDisplayKind,
    #[serde(rename = "@frag")]
    /// ID of fragment to link to.
    pub frag_id: String,
    #[serde(rename = "@labels", skip_serializing_if = "Option::is_none")]
    /// Comma separated xref labels.
    pub labels: Option<String>,
    #[serde(rename = "@level", skip_serializing_if = "Option::is_none")]
    /// Level for heading numbering of target document (1-5).
    pub level: Option<String>,
    #[serde(rename = "@reverselink")]
    /// Whether xref is bidirectional.
    pub reverselink: bool,
    #[serde(rename = "@reversetitle", skip_serializing_if = "Option::is_none")]
    /// Manually entered title for reverse xref.
    pub reversetitle: Option<String>,
    #[serde(rename = "@title", skip_serializing_if = "Option::is_none")]
    /// Manually entered title for xref.
    pub title: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    /// XRef type
    pub xref_type: Option<XRefKind>,
}

impl XRef {
    /// Returns a default xref to the given uriid.
    pub fn uriid(uriid: String) -> XRef {
        XRef {
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
        }
    }

    /// Returns a default xref to the given docid.
    pub fn docid(docid: String) -> XRef {
        XRef {
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
        }
    }

    /// Returns a default xref to the given href.
    pub fn href(href: String) -> XRef {
        XRef {
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
        }
    }

    /// Adds the specified content to the xref and returns it.
    pub fn with_content(self, content: String) -> XRef {
        XRef { content, ..self }
    }

    /// Sets the title on the xref and returns it.
    pub fn with_title(self, title: Option<String>) -> XRef {
        XRef { title, ..self }
    }

    /// Sets the display mode on the xref and returns it.
    pub fn with_display(self, display: XRefDisplayKind) -> XRef {
        XRef { display, ..self }
    }
}

// Property

/// Property datatype attribute values.
/// Does not support custom datatypes - they will be converted to "string".
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyDatatype {
    String,
    Datetime,
    XRef,
    Link,
    Markdown,
    Markup,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyValue {
    XRef(Box<XRef>),
    Link(String),
    Markdown(String),
    Markup(String),
    Value(String),
}

impl From<String> for PropertyValue {
    fn from(value: String) -> Self {
        Self::Value(value)
    }
}

impl PropertyValue {
    pub fn datatype(&self) -> PropertyDatatype {
        match self {
            Self::XRef(_) => PropertyDatatype::XRef,
            Self::Link(_) => PropertyDatatype::Link,
            Self::Markdown(_) => PropertyDatatype::Markdown,
            Self::Markup(_) => PropertyDatatype::Markup,
            Self::Value(_) => PropertyDatatype::String,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename = "property")]
/// A PSML property.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-property.html
pub struct Property {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "@datatype", skip_serializing_if = "Option::is_none")]
    pub datatype: Option<PropertyDatatype>,
    #[serde(rename = "@multiple", skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub attr_value: Option<String>,
    #[serde(rename = "$value", default)]
    pub values: Vec<PropertyValue>,
}

lazy_static! {
    /// Matches a leading '-' or a char not in [a-zA-Z0-9_-].
    static ref PROPERTY_BAD_NAME: Regex = Regex::new(r"(^-|[^a-zA-Z0-9_-]+)").unwrap();
}

impl Property {
    pub fn with_value(name: String, title: String, value: PropertyValue) -> Self {
        let datatype = value.datatype();
        Property {
            name,
            title: Some(title),
            values: vec![value],
            datatype: Some(datatype),
            multiple: None,
            attr_value: None,
        }
    }

    /// Replaces characters in `name` that are illegal for a PSML Property name.
    pub fn sanitize_name<'a, 'b>(name: &'a str, repl: &'b str) -> Cow<'a, str> {
        PROPERTY_BAD_NAME.replace_all(name, repl)
    }
}

// Fragments

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
        PropertiesFragment {
            id,
            frag_type: None,
            labels: None,
            properties: vec![],
            attrs: HashMap::new(),
        }
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub struct BlockXRef {
    #[serde(rename = "@docid", skip_serializing_if = "Option::is_none")]
    pub docid: Option<String>,
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "@uriid", skip_serializing_if = "Option::is_none")]
    pub uriid: Option<String>,

    #[serde(rename = "@archived")]
    pub archived: Option<bool>,
    #[serde(rename = "@config", skip_serializing_if = "Option::is_none")]
    pub config: Option<String>,
    #[serde(rename = "@display", skip_serializing_if = "Option::is_none")]
    pub display: Option<XRefDisplayKind>,
    #[serde(rename = "@documenttype", skip_serializing_if = "Option::is_none")]
    pub documenttype: Option<String>,
    #[serde(rename = "@external", skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(rename = "@frag")]
    pub frag: String,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<String>,
    #[serde(rename = "@level", skip_serializing_if = "Option::is_none")]
    pub level: Option<u8>,
    #[serde(rename = "@mediatype", skip_serializing_if = "Option::is_none")]
    pub mediatype: Option<String>,
    #[serde(rename = "@reversetitle", skip_serializing_if = "Option::is_none")]
    pub reversetitle: Option<String>,
    #[serde(rename = "@reverselink", skip_serializing_if = "Option::is_none")]
    pub reverselink: Option<bool>,
    #[serde(rename = "@reversefrag", skip_serializing_if = "Option::is_none")]
    pub reversefrag: Option<String>,
    #[serde(rename = "@title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub xref_type: Option<BlockXRefKind>,
    #[serde(rename = "@unresolved", skip_serializing_if = "Option::is_none")]
    pub unresolved: Option<bool>,
    #[serde(rename = "@urititle", skip_serializing_if = "Option::is_none")]
    pub urititle: Option<String>,
    #[serde(rename = "@urilabels", skip_serializing_if = "Option::is_none")]
    pub urilabels: Option<String>,
}

impl BlockXRef {
    pub fn docid(docid: String) -> Self {
        Self {
            docid: Some(docid),
            ..Default::default()
        }
    }

    pub fn uriid(uriid: String) -> Self {
        Self {
            uriid: Some(uriid),
            ..Default::default()
        }
    }

    pub fn href(href: String) -> Self {
        Self {
            href: Some(href),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
        XRefFragment {
            id,
            frag_type: None,
            labels: String::new(),
            xrefs: Vec::new(),
            attrs: HashMap::new(),
        }
    }

    /// Adds the xrefs to the fragment and returns it.
    pub fn with_xrefs(self, xrefs: Vec<BlockXRef>) -> XRefFragment {
        XRefFragment {
            id: self.id,
            frag_type: self.frag_type,
            labels: self.labels,
            xrefs: vec![self.xrefs, xrefs]
                .into_iter()
                .flatten()
                .collect::<Vec<BlockXRef>>(),
            attrs: self.attrs,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct TableCaption {
    #[serde(rename = "$text", default)]
    caption: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TablePart {
    Header,
    Body,
    Footer,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename = "col")]
pub struct TableColumn {
    #[serde(rename = "@align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Alignment>,
    #[serde(rename = "@part", skip_serializing_if = "Option::is_none")]
    pub part: Option<TablePart>,
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename = "row")]
pub struct TableRow {
    #[serde(rename = "@align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Alignment>,
    #[serde(rename = "@part", skip_serializing_if = "Option::is_none")]
    pub part: Option<TablePart>,
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "cell", default)]
    pub cells: Vec<TableCell>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename = "cell")]
pub struct TableCell {
    #[serde(rename = "@align", skip_serializing_if = "Option::is_none")]
    pub align: Option<Alignment>,
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "@colspan", skip_serializing_if = "Option::is_none")]
    pub colspan: Option<u64>,
    #[serde(rename = "@rowspan", skip_serializing_if = "Option::is_none")]
    pub rowspan: Option<u64>,
    #[serde(rename = "$text", default)]
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Table {
    pub caption: Option<TableCaption>,
    #[serde(rename = "@role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "@summary", skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "@height", skip_serializing_if = "Option::is_none")]
    pub height: Option<String>,
    #[serde(rename = "@width", skip_serializing_if = "Option::is_none")]
    pub width: Option<String>,
    #[serde(rename = "col", default)]
    pub cols: Vec<TableColumn>,
    #[serde(rename = "row", default)]
    pub rows: Vec<TableRow>,
}

impl Table {
    pub fn basic(cols: usize, cells: Vec<Vec<String>>, caption: String) -> Self {
        Table {
            caption: Some(TableCaption { caption }),
            role: None,
            summary: None,
            height: None,
            width: None,
            cols: (0..cols).map(|_| TableColumn::default()).collect(),
            rows: cells
                .into_iter()
                .map(|row| TableRow {
                    cells: row
                        .into_iter()
                        .map(|cell| TableCell {
                            content: cell,
                            ..Default::default()
                        })
                        .collect(),
                    ..Default::default()
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
    Table(Table),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
        Fragment {
            id,
            frag_type: None,
            labels: None,
            content: vec![],
            // attrs: HashMap::new(),
        }
    }

    /// Adds the content to the fragment and returns it.
    pub fn with_content(mut self, content: Vec<FragmentContent>) -> Fragment {
        self.content.extend(content);
        Fragment {
            id: self.id,
            frag_type: self.frag_type,
            labels: self.labels,
            content: self.content,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Fragments {
    #[serde(rename = "fragment")]
    Fragment(Fragment),
    #[serde(rename = "properties-fragment")]
    Properties(PropertiesFragment),
    #[serde(rename = "xref-fragment")]
    Xref(XRefFragment),
    #[serde(rename = "media-fragment")]
    Media(()),
}

// Section

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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
    #[serde(rename = "@lockstructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the structure of this section can be modified.
    pub lockstructure: Option<bool>,
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
        Section {
            id,
            title: None,
            content_title: None,
            edit: Some(true),
            lockstructure: Some(false),
            overwrite: Some(true),
            fragment_types: None,
            content: Vec::new(),
        }
    }

    pub fn add_fragment(&mut self, fragment: Fragments) {
        self.content.push(match fragment {
            Fragments::Fragment(frag) => SectionContent::Fragment(frag),
            Fragments::Media(frag) => SectionContent::Media(frag),
            Fragments::Properties(frag) => SectionContent::PropertiesFragment(frag),
            Fragments::Xref(frag) => SectionContent::XRefFragment(frag),
        });
    }

    pub fn with_fragments(mut self, fragments: Vec<Fragments>) -> Self {
        for frag in fragments {
            self.add_fragment(frag);
        }

        self
    }
}

// Document

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
/// The <description> element is used to provide a short text-only description.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-description.html
pub struct Description {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
/// Metadata about this URI.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-uri.html
pub struct URIDescriptor {
    // Attributes
    #[serde(rename = "@docid", skip_serializing_if = "Option::is_none")]
    /// Docid of this document.
    pub docid: Option<String>,
    #[serde(rename = "@documenttype", skip_serializing_if = "Option::is_none")]
    /// Type of the document.
    pub doc_type: Option<String>,
    #[serde(rename = "@title", skip_serializing_if = "Option::is_none")]
    /// Title for the document.
    pub title: Option<String>,
    #[serde(rename = "@folder", skip_serializing_if = "Option::is_none")]
    /// If true, this is a folder.
    pub folder: Option<bool>,

    // Elements
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Description of the document.
    pub description: Option<Description>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Labels on the document.
    pub labels: Option<Labels>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
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
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
/// A comma-separated list of label values for a document, note or fragment.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-labels.html
pub struct Labels {
    #[serde(rename = "$value")]
    pub value: String,
}

/// Previous document content if different from current (used when doing a compare).
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-content.html
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Content {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
/// The notes on the last notification of the fragment.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-notes.html
pub struct Notes {
    pub notes: Vec<Note>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
/// Metadata relating to a fragment.
/// For PSML definition see: https://dev.pageseeder.com/psml/element_reference/element-locator.html
pub struct Locator {
    #[serde(rename = "fragment")]
    /// ID of the fragment.
    pub fragment_id: Option<String>,
    /// Notes on this fragment.
    pub notes: Option<Notes>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct FragmentInfo {
    pub value: Vec<Locator>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DocumentLevel {
    Metadata,
    Portable,
    Processed,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
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

impl Document {
    pub fn docid(&self) -> Option<&str> {
        match &self.doc_info {
            Some(doc_info) => match &doc_info.uri {
                Some(uri) => match &uri.docid {
                    Some(docid) => Some(docid),
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }

    pub fn get_section(&self, id: &str) -> Option<&Section> {
        self.sections.iter().find(|&section| section.id == id)
    }

    pub fn get_mut_section(&mut self, id: &str) -> Option<&mut Section> {
        self.sections.iter_mut().find(|s| s.id == id)
    }
}

impl Default for Document {
    fn default() -> Self {
        Self {
            doc_info: None,
            frag_info: vec![],
            sections: vec![],
            doc_type: None,
            edit: None,
            level: DocumentLevel::Portable,
            lockstructure: None,
        }
    }
}
