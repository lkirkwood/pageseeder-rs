use std::fs;

use super::model::Document;

#[test]
fn test_fragment() {
    let doc: Document =
        quick_xml::de::from_str(&fs::read_to_string("test/fragment.psml").unwrap()).unwrap();

    fs::write(
        "test/out/fragment.psml",
        quick_xml::se::to_string(&doc).unwrap(),
    )
    .unwrap();
}

#[test]
fn test_properties_fragment() {
    let doc: Document =
        quick_xml::de::from_str(&fs::read_to_string("test/properties_fragment.psml").unwrap())
            .unwrap();

    fs::write(
        "test/out/properties_fragment.psml",
        quick_xml::se::to_string(&doc).unwrap(),
    )
    .unwrap();
}

#[test]
fn test_full_doc() {
    let doc: Document =
        quick_xml::de::from_str(&fs::read_to_string("test/document.psml").unwrap()).unwrap();

    fs::write(
        "test/out/document.psml",
        quick_xml::se::to_string(&doc).unwrap(),
    )
    .unwrap();
}

#[test]
fn test_table() {
    let doc: Document =
        quick_xml::de::from_str(&fs::read_to_string("test/table.psml").unwrap()).unwrap();

    fs::write(
        "test/out/table.psml",
        quick_xml::se::to_string(&doc).unwrap(),
    )
    .unwrap();

    println!("{doc:#?}");
}
