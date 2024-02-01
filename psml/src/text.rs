use serde::{Deserialize, Serialize};

use super::model::XRef;

macro_rules! impl_char_style {
    ($name:ty) => {
        impl $name {
            pub fn new(content: Vec<CharacterStyle>) -> Self {
                Self { content }
            }

            pub fn text(text: String) -> Self {
                Self {
                    content: vec![CharacterStyle::Text(text)],
                }
            }
        }
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename = "bold")]
pub struct Bold {
    #[serde(rename = "$value", default)]
    pub content: Vec<CharacterStyle>,
}

impl_char_style!(Bold);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Italic {
    #[serde(rename = "$value", default)]
    pub content: Vec<CharacterStyle>,
}

impl_char_style!(Italic);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Underline {
    #[serde(rename = "$value", default)]
    pub content: Vec<CharacterStyle>,
}

impl_char_style!(Underline);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Subscript {
    #[serde(rename = "$value", default)]
    pub content: Vec<CharacterStyle>,
}

impl_char_style!(Subscript);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Superscript {
    #[serde(rename = "$value", default)]
    pub content: Vec<CharacterStyle>,
}

impl_char_style!(Superscript);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Monospace {
    #[serde(rename = "$value", default)]
    pub content: Vec<CharacterStyle>,
}

impl_char_style!(Monospace);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CharacterStyle {
    #[serde(rename = "$text")]
    Text(String),
    Bold(Bold),
    Italic(Italic),
    Underline(Underline),
    Subscript(Subscript),
    Superscript(Superscript),
    Monospace(Monospace),
    XRef(Box<XRef>),
    // TODO inline, anchor, placeholder, br, link
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename = "image")]
pub struct Image {
    // TODO enforce at least one of src, docid, uriid
    #[serde(rename = "@src")]
    src: Option<String>,
    #[serde(rename = "@docid")]
    docid: Option<String>,
    #[serde(rename = "@uriid")]
    uriid: Option<String>,
    #[serde(rename = "@labels")]
    labels: Option<String>,
    #[serde(rename = "@height")]
    height: Option<u64>,
    #[serde(rename = "@width")]
    width: Option<u64>,
    #[serde(rename = "@alt")]
    alt: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ParaContent {
    #[serde(rename = "$text")]
    Text(String),
    Bold(Bold),
    Italic(Italic),
    Underline(Underline),
    Subscript(Subscript),
    Superscript(Superscript),
    Monospace(Monospace),
    XRef(XRef),
    Image(Image),
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
#[serde(rename = "para")]
pub struct Para {
    #[serde(rename = "@indent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indent: Option<u8>,
    #[serde(rename = "@numbered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numbered: Option<bool>,
    #[serde(rename = "@prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "$value", default)]
    pub content: Vec<ParaContent>,
}

impl Para {
    pub fn new(content: Vec<ParaContent>) -> Self {
        Para {
            indent: None,
            numbered: None,
            prefix: None,
            content,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Heading {
    #[serde(rename = "@level")]
    pub level: Option<u8>,
    #[serde(rename = "$value", default)]
    pub content: Vec<CharacterStyle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Alignment {
    Left,
    Center,
    Right,
    Justify,
}
