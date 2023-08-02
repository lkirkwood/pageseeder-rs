pub mod types {
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct DocumentId {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct PublicationId {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct DocumentType {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct PublicationType {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct SectionId {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct FragmentId {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct FragmentType {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct FragmentTypeList {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct Role {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct Label {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct LabelList {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct XrefConfig {
        #[yaserde(text)]
        pub content: std::string::String,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct CellStyle {
        #[yaserde(attribute)]
        pub role: Option<Role>,
        #[yaserde(attribute)]
        pub colspan: Option<u64>,
        #[yaserde(attribute)]
        pub rowspan: Option<u64>,
        #[yaserde(attribute)]
        pub align: Option<String>,
        #[yaserde(attribute)]
        pub valign: Option<String>,
        #[yaserde(attribute)]
        pub width: Option<String>,
    }
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct CharacterStyle {}
    #[doc = "A Cross Reference between two locations in documents.\r\n        The attribute frag is required plus at least one of the following: href, docid, uriid.\r\n        The content is INFORMATIONAL - ignored by upload as the link display text is generated."]
    #[derive(
        Clone,
        Debug,
        Default,
        PartialEq,
        yaserde_derive ::
        YaDeserialize,
        yaserde_derive :: YaSerialize,
    )]
    pub struct XrefType {
        #[yaserde(attribute)]
        pub archived: Option<bool>,
        #[yaserde(attribute)]
        pub id: Option<i64>,
        #[yaserde(attribute)]
        pub docid: Option<DocumentId>,
        #[yaserde(attribute)]
        pub href: Option<String>,
        #[yaserde(attribute)]
        pub config: Option<XrefConfig>,
        #[yaserde(attribute)]
        pub uriid: Option<i64>,
        #[yaserde(attribute)]
        pub frag: FragmentId,
        #[yaserde(attribute)]
        pub title: Option<String>,
        #[yaserde(attribute)]
        pub level: Option<i32>,
        #[yaserde(attribute)]
        pub display: Option<String>,
        #[yaserde(attribute)]
        pub labels: Option<LabelList>,
        #[yaserde(attribute)]
        pub urititle: Option<String>,
        #[yaserde(attribute)]
        pub urilabels: Option<LabelList>,
        #[yaserde(attribute)]
        pub mediatype: Option<String>,
        #[yaserde(attribute)]
        pub documenttype: Option<String>,
        #[yaserde(attribute)]
        pub reverselink: Option<bool>,
        #[yaserde(attribute)]
        pub reversetitle: Option<String>,
        #[yaserde(attribute)]
        pub reversetype: Option<String>,
        #[yaserde(attribute)]
        pub reversefrag: Option<FragmentId>,
        #[yaserde(attribute)]
        pub unresolved: Option<bool>,
        #[yaserde(attribute)]
        pub external: Option<bool>,
    }
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Document {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Fragments {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Section {
    #[yaserde(rename = "title")]
    pub title: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Toc {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct TocTree {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Fragment {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct MediaFragment {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct XrefFragment {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct PropertiesFragment {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Property {}
#[doc = "Markdown or PSML formatted content for a property."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Markdown {}
#[doc = "Metadata at the document level"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Documentinfo {}
#[doc = "The uri of the document"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Uri {}
#[doc = "The publication of the document"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Publication {}
#[doc = "The version this document is being compared to. INFORMATIONAL - ignored by upload."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Compareto {}
#[doc = "The structure of the document being compared with. INFORMATIONAL - ignored by upload."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Structure {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct SectionRef {
    #[yaserde(rename = "title")]
    pub title: Option<String>,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct FragmentRef {}
#[doc = "The list of existing versions. INFORMATIONAL - ignored by upload."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Versions {}
#[doc = "A version of the document. INFORMATIONAL - ignored by upload."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Version {}
#[doc = "The list of reverse cross-references. INFORMATIONAL - ignored by upload."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Reversexrefs {}
#[doc = "A cross-reference from another document. INFORMATIONAL - ignored by upload."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Reversexref {}
#[doc = "The author of a version. INFORMATIONAL - ignored by upload."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Author {
    #[yaserde(rename = "fullname")]
    pub fullname: String,
}
#[doc = "The labels of the document or fragment"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Labels {
    #[yaserde(flatten)]
    pub content: types::LabelList,
}
#[doc = "Metadata for document"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Metadata {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Properties {}
#[doc = "Metadata at the fragment level"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Fragmentinfo {}
#[doc = "Metadata of a locator"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Locator {}
#[doc = "The notes on the last notification of the fragment"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Notes {}
#[doc = "The notes on the last notification of the fragment"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Note {
    #[yaserde(rename = "content")]
    pub contents: Vec<String>,
}
#[doc = "Details of previous fragment content if different from current (used when doing a compare). INFORMATIONAL - ignored by upload."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Compare {}
#[doc = "Diffx compare of previous document content with current (used when doing a compare)"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Diff {}
#[doc = "Previous document content if different from current (used when doing a compare)"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Content {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Para {}
#[doc = "Labeled block (previously paraLabel)"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Block {}
#[doc = "A block level cross reference."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Blockxref {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Heading {}
#[doc = "A reference to an image.\r\n        Requires at least one of the following: src, docid, uriid."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Image {}
#[doc = "Used for computer source code typically with a monospaced font (previously code)."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Preformat {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Table {
    #[yaserde(rename = "caption")]
    pub caption: Option<types::CharacterStyle>,
}
#[doc = "For defining column properties (starting from left - not all columns need be defined)"]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Col {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Row {}
#[doc = "For body of a table."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Cell {
    #[yaserde(flatten)]
    pub content: types::CellStyle,
}
#[doc = "For header of a table. DEPRECATED - use @part."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Hcell {
    #[yaserde(flatten)]
    pub content: types::CellStyle,
}
#[doc = "Unnumbered list."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct List {}
#[doc = "Auto-numbered list."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Nlist {}
#[doc = "Item in a list or an nlist."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Item {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Bold {
    #[yaserde(flatten)]
    pub content: types::CharacterStyle,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Italic {
    #[yaserde(flatten)]
    pub content: types::CharacterStyle,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Underline {
    #[yaserde(flatten)]
    pub content: types::CharacterStyle,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Sup {
    #[yaserde(flatten)]
    pub content: types::CharacterStyle,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Sub {
    #[yaserde(flatten)]
    pub content: types::CharacterStyle,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Monospace {
    #[yaserde(flatten)]
    pub content: types::CharacterStyle,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Br {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Inline {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Placeholder {
    pub content: String,
}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Link {}
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Anchor {}
#[doc = "An in-line cross reference."]
#[derive(
    Clone,
    Debug,
    Default,
    PartialEq,
    yaserde_derive ::
    YaDeserialize,
    yaserde_derive :: YaSerialize,
)]
pub struct Xref {}
