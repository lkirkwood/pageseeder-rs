use crate::psml::gen::Document;
use std::fs;

#[test]
fn test_parse_document() {
    let doc_str = fs::read_to_string("test/document.psml").unwrap();
    let _doc: Document = yaserde::de::from_str(&doc_str).unwrap();
}
