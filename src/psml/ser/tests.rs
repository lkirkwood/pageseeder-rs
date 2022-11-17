use std::{fs, io::Cursor};

use quick_xml::{
    events::{BytesText, Event},
    Reader, Writer,
};

use pretty_assertions::assert_eq;

use crate::{
    error::PSResult,
    psml::model::{Fragment, PropertiesFragment, Property, PropertyDatatype, PropertyValue, XRef},
};

use super::{read_event, write_text, PSMLObject};

/// Reads psmlobjs from a string.
fn read_psmlobjs<T: PSMLObject>(string: &str) -> PSResult<Vec<T>> {
    let mut reader = Reader::from_str(string);

    let mut objs = Vec::new();
    let elem_name = T::elem_name().as_bytes();
    loop {
        match read_event(&mut reader)? {
            Event::Start(elem) => match elem.name().as_ref() {
                _en if _en == elem_name => objs.push(T::from_psml(&mut reader, elem)?),
                _ => {}
            },
            Event::Eof => break,
            _ => {}
        }
    }
    return Ok(objs);
}

/// Writes a psmlobj to a string.
fn write_psmlobjs(psmlobjs: Vec<impl PSMLObject>) -> PSResult<String> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));
    for psmlobj in psmlobjs {
        psmlobj.to_psml(&mut writer)?;
        write_text(&mut writer, BytesText::new("\n"))?;
    }

    return Ok(String::from_utf8(writer.into_inner().into_inner()).unwrap());
}

/// Reads events from a string.
fn read_events(string: &str) -> PSResult<Vec<Event<'static>>> {
    let mut reader = Reader::from_str(string);
    let mut events = Vec::new();
    loop {
        match read_event(&mut reader)? {
            Event::Eof => break,
            other => events.push(other),
        }
    }
    return Ok(events);
}

//// Property

// Fixtures

const MARKDOWN_PROPERTY_CONTENT: &str =
    "<markdown>The **quick** brown *fox* jumps over the `lazy` dog.\
</markdown>";
const MARKUP_PROPERTY_CONTENT: &str =
    "<para>The <bold>quick</bold> brown <italic>fox</italic></para>\
<para>jumps over the <monospace>lazy</monospace> dog</para>";

/// Returns a property with the same attributes the test data.
fn properties() -> Vec<Property> {
    return vec![
        Property {
            name: "part-number".to_string(),
            title: None,
            multiple: false,
            datatype: PropertyDatatype::String,
            values: vec![PropertyValue::String("PX-67S93Q".to_string())],
        },
        Property {
            name: "languages".to_string(),
            title: None,
            multiple: true,
            datatype: PropertyDatatype::String,
            values: vec![
                PropertyValue::String("de".to_string()),
                PropertyValue::String("en".to_string()),
                PropertyValue::String("fr".to_string()),
            ],
        },
        Property {
            name: "author".to_string(),
            title: None,
            multiple: false,
            datatype: PropertyDatatype::XRef,
            values: vec![PropertyValue::XRef(
                XRef::href("/ps/authors/english/".to_string())
                    .with_content("Lewis Carroll".to_string()),
            )],
        },
        Property {
            name: "example".to_string(),
            title: None,
            multiple: false,
            datatype: PropertyDatatype::Markdown,
            values: vec![PropertyValue::Markdown(
                read_events(MARKDOWN_PROPERTY_CONTENT).unwrap(),
            )],
        },
        Property {
            name: "example".to_string(),
            title: None,
            multiple: false,
            datatype: PropertyDatatype::Markup,
            values: vec![PropertyValue::Markup(
                read_events(MARKUP_PROPERTY_CONTENT).unwrap(),
            )],
        },
    ];
}

// Tests

#[test]
fn property_from_psml() {
    let str_props: Vec<Property> =
        read_psmlobjs(&fs::read_to_string("test/property.psml").unwrap()).unwrap();
    assert_eq!(properties(), str_props);
}

#[test]
fn property_to_psml() {
    let prop_strs = write_psmlobjs(properties()).unwrap();
    assert_eq!(fs::read_to_string("test/property.psml").unwrap(), prop_strs);
}

//// PropertiesFragment

// Fixtures

/// Returns a properties fragment with the same attributes the test data.
fn properties_fragments() -> Vec<PropertiesFragment> {
    return vec![
        PropertiesFragment::new(3.to_string()).with_properties(vec![
            Property {
                name: "part-id".to_string(),
                title: None,
                multiple: false,
                datatype: PropertyDatatype::String,
                values: vec![PropertyValue::String("FY765A".to_string())],
            },
            Property {
                name: "part-category".to_string(),
                title: None,
                multiple: false,
                datatype: PropertyDatatype::String,
                values: vec![PropertyValue::String("Industrial".to_string())],
            },
        ]),
        PropertiesFragment::new(4.to_string()).with_properties(vec![
            Property {
                name: "title".to_string(),
                title: Some("Title".to_string()),
                multiple: false,
                datatype: PropertyDatatype::String,
                values: vec![PropertyValue::String(
                    "Alice's Adventures in Wonderland".to_string(),
                )],
            },
            Property {
                name: "isbn".to_string(),
                title: Some("ISBN".to_string()),
                multiple: false,
                datatype: PropertyDatatype::String,
                values: vec![PropertyValue::String("0000000000001".to_string())],
            },
            Property {
                name: "author".to_string(),
                title: Some("Author".to_string()),
                multiple: false,
                datatype: PropertyDatatype::String,
                values: vec![PropertyValue::String("Lewis Caroll".to_string())],
            },
            Property {
                name: "language".to_string(),
                title: Some("Language".to_string()),
                multiple: true,
                datatype: PropertyDatatype::String,
                values: vec![
                    PropertyValue::String("English".to_string()),
                    PropertyValue::String("Spanish".to_string()),
                ],
            },
            Property {
                name: "country".to_string(),
                title: Some("Country".to_string()),
                multiple: false,
                datatype: PropertyDatatype::String,
                values: vec![PropertyValue::String("Australia".to_string())],
            },
            Property {
                name: "available-date".to_string(),
                title: Some("Available date".to_string()),
                multiple: false,
                datatype: PropertyDatatype::Date,
                values: vec![PropertyValue::String("2012-01-01".to_string())],
            },
            Property {
                name: "related".to_string(),
                title: Some("Related".to_string()),
                multiple: true,
                datatype: PropertyDatatype::XRef,
                values: vec![
                    PropertyValue::XRef(XRef::href("book4.psml".to_string())),
                    PropertyValue::XRef(XRef::href("book7.psml".to_string())),
                ],
            },
        ]),
    ];
}

// Tests

#[test]
fn properties_fragment_from_psml() {
    let str_pfrags: Vec<PropertiesFragment> =
        read_psmlobjs(&fs::read_to_string("test/properties-fragment.psml").unwrap()).unwrap();
    assert_eq!(properties_fragments(), str_pfrags);
}

#[test]
fn properties_fragment_to_psml() {
    let prop_strs = write_psmlobjs(properties_fragments()).unwrap();
    assert_eq!(
        fs::read_to_string("test/properties-fragment.psml").unwrap(),
        prop_strs
    );
}

//// Fragment

// Fixtures

/// Returns a fragment with the same attributes as the test data.
fn fragments() -> Vec<Fragment> {
    return vec![
        Fragment::new(1.to_string()).with_content(
            read_events("<heading level=\"1\">Alice in Wonderland</heading>").unwrap(),
        ),
        Fragment {
            id: 1.to_string(),
            frag_type: Some("thumbnail".to_string()),
            labels: Vec::new(),
            content: read_events("<image href=\"/path/to/an/image.ext\" />").unwrap(),
        },
        Fragment {
            id: 1.to_string(),
            frag_type: None,
            labels: vec!["internal".to_string()],
            content: read_events("<para><italic>Office</italic> use <bold>only</bold></para>")
                .unwrap(),
        },
    ];
}

// Tests

#[test]
fn fragment_from_psml() {
    let str_fragments = read_psmlobjs(&fs::read_to_string("test/fragment.psml").unwrap()).unwrap();
    assert_eq!(fragments(), str_fragments);
}

#[test]
fn fragment_to_psml() {
    let fragment_str = write_psmlobjs(fragments()).unwrap();
    assert_eq!(
        fs::read_to_string("test/fragment.psml").unwrap(),
        fragment_str
    );
}
