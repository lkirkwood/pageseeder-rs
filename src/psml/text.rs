use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename = "bold")]
pub struct Bold {
    #[serde(rename = "$value", default)]
    content: Vec<CharacterStyle>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Italic {
    #[serde(rename = "$value", default)]
    content: Vec<CharacterStyle>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Underline {
    #[serde(rename = "$value", default)]
    content: Vec<CharacterStyle>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Subscript {
    #[serde(rename = "$value", default)]
    content: Vec<CharacterStyle>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Superscript {
    #[serde(rename = "$value", default)]
    content: Vec<CharacterStyle>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Monospace {
    #[serde(rename = "$value", default)]
    content: Vec<CharacterStyle>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum CharacterStyle {
    #[serde(rename = "$text")]
    Text(String),
    #[serde(rename = "bold")]
    Bold(Bold),
    #[serde(rename = "italic")]
    Italic(Italic),
    #[serde(rename = "underline")]
    Underline(Underline),
    #[serde(rename = "subscript")]
    Subscript(Subscript),
    #[serde(rename = "superscript")]
    Superscript(Superscript),
    #[serde(rename = "monospace")]
    Monospace(Monospace),
    // TODO inline, anchor, placeholder, br, link
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
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

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
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
    Image(Image),
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Para {
    #[serde(rename = "@indent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    indent: Option<u8>,
    #[serde(rename = "@numbered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    numbered: Option<bool>,
    #[serde(rename = "@prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    prefix: Option<String>,
    #[serde(rename = "$value", default)]
    // content: Vec<ParaContent>,
    content: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Heading {
    #[serde(rename = "@level")]
    level: Option<u8>,
    #[serde(rename = "$value")]
    child: Vec<String>,
}
