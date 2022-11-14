use std::{
    borrow::Cow,
    io::{BufRead, Write},
};

use crate::{
    error::{PSError, PSResult},
    psml::model::Property,
};
use quick_xml::{
    events::{attributes::Attribute, BytesEnd, BytesStart, BytesText, Event},
    name::QName,
    Reader, Writer,
};

use super::model::PropertiesFragment;

// Convenience functions

/// Decodes an attribute value and returns a PSResult.
fn decode_attr<'a, R: BufRead>(reader: &'a Reader<R>, attr: Attribute) -> PSResult<String> {
    match attr.decode_and_unescape_value(reader) {
        Err(err) => {
            return Err(PSError::ParseError {
                msg: format!("Failed to decode attribute {:?}: {:?}", attr.key, err),
            })
        }
        Ok(val) => Ok(val.into_owned()),
    }
}

/// Writes an element start to a writer and returns a PSResult.
fn write_elem_start<W: Write>(writer: &mut Writer<W>, elem: BytesStart) -> PSResult<()> {
    let name = elem.name().0.to_owned();
    match writer.write_event(Event::Start(elem)) {
        Ok(_) => return Ok(()),
        Err(err) => {
            return Err(PSError::ParseError {
                msg: format!(
                    "Error writing element {} start to writer: {:?}",
                    String::from_utf8(name)
                        .unwrap_or("(failed to decode name from utf-8)".to_string()),
                    err
                ),
            })
        }
    }
}

/// Writes an element end to a writer and returns a PSResult.
fn write_elem_end<W: Write>(writer: &mut Writer<W>, elem: BytesEnd) -> PSResult<()> {
    let name = elem.name().0.to_owned();
    match writer.write_event(Event::End(elem)) {
        Ok(_) => return Ok(()),
        Err(err) => {
            return Err(PSError::ParseError {
                msg: format!(
                    "Error writing element {} end to writer: {:?}",
                    String::from_utf8(name)
                        .unwrap_or("(failed to decode name from utf-8)".to_string()),
                    err
                ),
            })
        }
    }
}

/// Writes text to a writer and returns a PSResult.
fn write_text<W: Write>(writer: &mut Writer<W>, text: BytesText) -> PSResult<()> {
    match writer.write_event(Event::Text(text)) {
        Ok(_) => return Ok(()),
        Err(err) => {
            return Err(PSError::ParseError {
                msg: format!("Error writing text to writer: {:?}", err),
            })
        }
    }
}

/// Reads an event from a reader and returns a PSResult.
fn read_event<'a, R: BufRead>(reader: &'a mut Reader<R>) -> PSResult<Event<'a>> {
    let mut buf = Vec::new();
    match reader.read_event_into(&mut buf) {
        Err(err) => {
            return Err(PSError::ParseError {
                msg: format!("Failed to read event: {}", err),
            })
        }
        Ok(event) => return Ok(event.into_owned()),
    }
}

// PSMLObject

pub trait PSMLObject: Sized {
    /// Returns the name of the element this psmlobject is defined by in psml.
    fn elem_name() -> &'static str;

    /// Returns a PSError if the type element name and actual element name do not match.
    fn match_elem_name(elem: &BytesStart) -> PSResult<()> {
        if Self::elem_name().as_bytes() != elem.name().as_ref() {
            return Err(PSError::ParseError {
                msg: format!(
                    "Trying to parse {} from element {:?}",
                    Self::elem_name(),
                    elem.name()
                ),
            });
        } else {
            return Ok(());
        }
    }

    /// Returns an instance of this psmlobject from a reader which has just read the start tag for this object.
    fn from_psml<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Self>;
    /// Writes this object to a writer as psml.
    fn to_psml<W: Write>(&self, writer: &mut Writer<W>) -> PSResult<()>;
}

// Property

impl Property {
    /// Reads a property value into a string.
    fn read_value<R: BufRead>(reader: &mut Reader<R>, _val_start: BytesStart) -> PSResult<String> {
        let mut value = String::new();
        loop {
            match read_event(reader)? {
                Event::Text(text) => match text.unescape() {
                    Err(err) => {
                        return Err(PSError::ParseError {
                            msg: format!("Failed to unescape text: {}", err),
                        })
                    }
                    Ok(cow) => {
                        value.push_str(&cow);
                    }
                },
                Event::End(val_end) => match val_end.name().as_ref() {
                    b"value" => break,
                    _ => {
                        return Err(PSError::ParseError {
                            msg: format!("Unknown element closed in value: {:#?}", val_end.name()),
                        })
                    }
                },
                _ => {}
            }
        }
        return Ok(value);
    }

    /// Reads value elements nested under a property.
    fn read_values<R: BufRead>(reader: &mut Reader<R>, pname: &str) -> PSResult<Vec<String>> {
        let mut buf = Vec::new();
        let mut values = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!(
                            "Failed reading events after property {} start: {:?}",
                            pname, err
                        ),
                    })
                }
                Ok(Event::Start(val_start)) => match val_start.name().as_ref() {
                    b"value" => {
                        values.push(Property::read_value(reader, val_start)?);
                    }
                    _ => {
                        return Err(PSError::ParseError {
                            msg: format!(
                                "Incorrect element in property {}: {:#?}",
                                pname,
                                val_start.name()
                            ),
                        })
                    }
                },
                Ok(Event::End(elem_end)) => match elem_end.name().as_ref() {
                    b"property" => break,
                    _ => {
                        return Err(PSError::ParseError {
                            msg: format!(
                                "Unknown element closed in property {}: {:#?}",
                                pname,
                                elem_end.name()
                            ),
                        })
                    }
                },
                Ok(Event::Eof) => break,
                _ => {}
            }
        }
        return Ok(values);
    }
}

impl PSMLObject for Property {
    fn elem_name() -> &'static str {
        return "property";
    }

    fn from_psml<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Property> {
        Self::match_elem_name(&elem)?;
        let mut pname = None;
        let mut ptitle = None;
        let mut pvals = Vec::new();
        for attr_res in elem.attributes() {
            match attr_res {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed to get property attribute: {:?}", err),
                    })
                }
                Ok(attr) => match attr.key.as_ref() {
                    b"name" => {
                        pname = Some(decode_attr(reader, attr)?);
                    }
                    b"title" => {
                        ptitle = Some(decode_attr(reader, attr)?);
                    }
                    b"value" => {
                        pvals.push(decode_attr(reader, attr)?);
                    }
                    _ => {}
                },
            }
        }

        if pname.is_none() {
            return Err(PSError::ParseError {
                msg: "Property missing required 'name' attribute".to_string(),
            });
        }

        pvals.extend(Property::read_values(reader, pname.as_ref().unwrap())?);

        return Ok(Property {
            name: pname.unwrap(),
            title: ptitle,
            value: pvals,
        });
    }

    fn to_psml<W: Write>(&self, writer: &mut Writer<W>) -> PSResult<()> {
        let mut prop_elem = BytesStart::new("property");
        prop_elem.push_attribute(Attribute {
            key: QName(b"name"),
            value: Cow::Borrowed(self.name.as_bytes()),
        });
        prop_elem.push_attribute(Attribute {
            key: QName(b"title"),
            value: Cow::Borrowed(self.title.as_ref().unwrap_or(&"".to_string()).as_bytes()),
        });

        let single_val = self.value.len() <= 1;
        if single_val == true {
            prop_elem.push_attribute(Attribute {
                key: QName(b"value"),
                value: Cow::Borrowed(self.value.get(0).unwrap_or(&"".to_string()).as_bytes()),
            });
        }

        write_elem_start(writer, prop_elem)?;

        if single_val == false {
            for val in &self.value {
                write_elem_start(writer, BytesStart::new("value"))?;
                write_text(writer, BytesText::new(val))?;
                write_elem_end(writer, BytesEnd::new("value"))?;
            }
        }

        write_elem_end(writer, BytesEnd::new("property"))?;

        return Ok(());
    }
}

// PropertiesFragment

impl PSMLObject for PropertiesFragment {
    fn elem_name() -> &'static str {
        return "properties-fragment";
    }

    fn from_psml<R: BufRead>(
        reader: &mut Reader<R>,
        elem: BytesStart,
    ) -> PSResult<PropertiesFragment> {
        Self::match_elem_name(&elem)?;
        let mut frag_id = None;
        let mut frag_title = None;
        for attr in elem.attributes() {
            match attr {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed to get properties fragment attribute: {:?}", err),
                    })
                }
                Ok(attr) => match attr.key.as_ref() {
                    b"id" => frag_id = Some(decode_attr(reader, attr)?),
                    b"title" => frag_title = Some(decode_attr(reader, attr)?),
                    _ => {}
                },
            }
        }

        if frag_id.is_none() {
            return Err(PSError::ParseError {
                msg: "Properties fragment missing required 'id' attribute.".to_string(),
            });
        }

        let mut buf = Vec::new();
        let mut props = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!(
                            "Failed to read events after properties-fragment {} start: {:?}",
                            frag_id.unwrap(),
                            err
                        ),
                    })
                }
                Ok(Event::Start(prop_start)) => {
                    props.push(Property::from_psml(reader, prop_start)?)
                }
                Ok(Event::End(elem_end)) => match elem_end.name().as_ref() {
                    b"properties-fragment" => break,
                    _ => {
                        return Err(PSError::ParseError {
                            msg: format!(
                                "Unexpected element closed in properties-fragment: {:#?}",
                                elem_end.name()
                            ),
                        })
                    }
                },
                Ok(Event::Eof) => break,
                _ => {}
            }
        }

        return Ok(PropertiesFragment {
            id: frag_id.unwrap(),
            title: frag_title,
            properties: props,
        });
    }

    fn to_psml<W: Write>(&self, writer: &mut Writer<W>) -> PSResult<()> {
        let mut elem_start = BytesStart::new("properties-fragment");

        elem_start.push_attribute(Attribute {
            key: QName(b"id"),
            value: Cow::Borrowed(self.id.as_bytes()),
        });
        if self.title.is_some() {
            elem_start.push_attribute(Attribute {
                key: QName(b"title"),
                value: Cow::Borrowed(self.title.as_ref().unwrap().as_bytes()),
            });
        }

        write_elem_start(writer, elem_start)?;

        for property in &self.properties {
            property.to_psml(writer)?;
        }

        write_elem_end(writer, BytesEnd::new("properties-fragment"))?;

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use quick_xml::{events::Event, Reader, Writer};

    use crate::{
        error::PSResult,
        psml::model::{PropertiesFragment, Property},
    };

    use super::PSMLObject;

    /// Reads a psmlobj from a string.
    fn read_psmlobj<T: PSMLObject>(string: &str) -> PSResult<T> {
        let mut reader = Reader::from_str(string);
        reader.trim_text(true);
        reader.expand_empty_elements(true);

        let elem_name = T::elem_name().as_bytes();
        loop {
            match reader.read_event() {
                Err(err) => panic!("Read error: {}", err),
                Ok(Event::Start(elem)) => match elem.name().as_ref() {
                    _en if _en == elem_name => return T::from_psml(&mut reader, elem),
                    _ => {}
                },
                _ => {}
            }
        }
    }

    /// Writes a psmlobj to a string.
    fn write_psmlobj(psmlobj: impl PSMLObject) -> PSResult<String> {
        let mut writer = Writer::new(Cursor::new(Vec::new()));
        psmlobj.to_psml(&mut writer)?;

        return Ok(String::from_utf8(writer.into_inner().into_inner()).unwrap());
    }

    // Property

    const PROPERTY: &'static str = "<property name=\"propname\" title=\"prop title\">\
<value>value1</value>\
<value>value2</value>\
<value>value3</value>\
</property>";

    /// Returns a property with the same attributes as PROPERTY.
    fn property() -> Property {
        return Property {
            name: "propname".to_string(),
            title: Some("prop title".to_string()),
            value: vec![
                "value1".to_string(),
                "value2".to_string(),
                "value3".to_string(),
            ],
        };
    }

    #[test]
    fn property_from_psml() {
        let str_prop: Property = read_psmlobj(PROPERTY).unwrap();
        assert_eq!(property(), str_prop);
    }

    #[test]
    fn property_to_psml() {
        let prop_str = write_psmlobj(property()).unwrap();
        assert_eq!(PROPERTY, prop_str);
    }

    // PropertiesFragment

    const PROPERTIES_FRAGMENT: &'static str = "<properties-fragment id=\"pfrag_id\" title=\"PFrag Title\">\
<property name=\"prop1\" title=\"Prop1 Title\" value=\"value1\"></property>\
<property name=\"prop2\" title=\"Multival Prop(2) title\"><value>value2-1</value><value>value2-2</value>\
<value>value2-3</value></property></properties-fragment>";

    fn properties_fragment() -> PropertiesFragment {
        return PropertiesFragment {
            id: "pfrag_id".to_string(),
            title: Some("PFrag Title".to_string()),
            properties: vec![
                Property {
                    name: "prop1".to_string(),
                    title: Some("Prop1 Title".to_string()),
                    value: vec!["value1".to_string()],
                },
                Property {
                    name: "prop2".to_string(),
                    title: Some("Multival Prop(2) title".to_string()),
                    value: vec![
                        "value2-1".to_string(),
                        "value2-2".to_string(),
                        "value2-3".to_string(),
                    ],
                },
            ],
        };
    }

    #[test]
    fn properties_fragment_from_psml() {
        let str_prop: PropertiesFragment = read_psmlobj(PROPERTIES_FRAGMENT).unwrap();
        assert_eq!(properties_fragment(), str_prop);
    }

    #[test]
    fn properties_fragment_to_psml() {
        let prop_str = write_psmlobj(properties_fragment()).unwrap();
        assert_eq!(PROPERTIES_FRAGMENT, prop_str);
    }
}
