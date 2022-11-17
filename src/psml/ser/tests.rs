use std::{fs, io::Cursor};

use quick_xml::{
    events::{BytesText, Event},
    Reader, Writer,
};

use pretty_assertions::assert_eq;

use crate::{
    error::PSResult,
    psml::model::{
        Fragment, PropertiesFragment, Property, PropertyDatatype, PropertyValue, XRef,
        XRefDisplayKind,
    },
};

use super::{read_event, write_text, PSMLObject};

/// Reads psmlobjs from a string.
fn read_psmlobjs<T: PSMLObject>(string: &str) -> PSResult<Vec<T>> {
    let mut reader = Reader::from_str(string);
    reader.expand_empty_elements(true);

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

/// Returns a property with the same attributes as PROPERTY.
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

// Fragment

//     const FRAGMENT: &'static str = "<fragment id=\"frag_id\" title=\"Frag Title!\">\
// <p>This is normal text, this is <bold>bold</bold> text, this is <italic>italic</italic> text,\
// and this is <monospace>m o n o s p a c e</monospace> text!</p></fragment>";

//     fn fragment_content() -> Vec<Event<'static>> {
//         // TODO write better test fixture
//         let mut reader = Reader::from_str(FRAGMENT);
//         loop {
//             match reader.read_event() {
//                 Err(err) => panic!("Read error: {:?}", err),
//                 Ok(Event::Start(frag_start)) => match frag_start.name().as_ref() {
//                     b"fragment" => {
//                         return read_fragment_content(&mut reader, "testing fragment").unwrap()
//                     }
//                     _ => panic!(
//                         "Unexpected element in test fragment string: {:?}",
//                         frag_start.name()
//                     ),
//                 },
//                 Ok(event) => panic!("Unexpected event in test fragment string: {:?}", event),
//             }
//         }
//     }

//     fn fragment() -> Fragment {
//         return Fragment {
//             id: "frag_id".to_string(),
//             title: Some("Frag Title!".to_string()),
//             content: fragment_content(),
//         };
//     }

//     #[test]
//     fn fragment_from_psml() {
//         let str_fragment = read_psmlobj(FRAGMENT).unwrap();
//         assert_eq!(fragment(), str_fragment);
//     }

//     #[test]
//     fn fragment_to_psml() {
//         let fragment_str = write_psmlobj(fragment()).unwrap();
//         assert_eq!(FRAGMENT, fragment_str);
//     }
