use std::borrow::Cow;

use crate::{
    error::{PSError, PSResult},
    psml::model::Property,
};
use quick_xml::{
    events::{attributes::Attribute, BytesEnd, BytesStart, BytesText, Event},
    name::QName,
    Reader, Writer,
};

// Convenience functions

/// Decodes an attribute value and returns a PSResult.
fn decode_attr<'a>(reader: &'a Reader<&[u8]>, attr: Attribute) -> PSResult<String> {
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
fn write_elem_start(writer: &mut Writer<&mut [u8]>, elem: BytesStart) -> PSResult<()> {
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
fn write_elem_end(writer: &mut Writer<&mut [u8]>, elem: BytesEnd) -> PSResult<()> {
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
fn write_text(writer: &mut Writer<&mut [u8]>, text: BytesText) -> PSResult<()> {
    match writer.write_event(Event::Text(text)) {
        Ok(_) => return Ok(()),
        Err(err) => {
            return Err(PSError::ParseError {
                msg: format!("Error writing text to writer: {:?}", err),
            })
        }
    }
}

// PSMLObject

pub trait PSMLObject: Sized {
    fn from_psml(reader: &mut Reader<&[u8]>, elem: BytesStart) -> PSResult<Self>;
    fn to_psml(&self, writer: &mut Writer<&mut [u8]>) -> PSResult<()>;
}

// Property

impl Property {
    /// Reads a property value into a string.
    fn read_value(reader: &mut Reader<&[u8]>, _val_start: BytesStart) -> PSResult<String> {
        let mut value = String::new();
        loop {
            match reader.read_event() {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed reading events after value start: {:?}", err),
                    })
                }
                Ok(Event::Text(text)) => match text.unescape() {
                    Err(err) => {
                        return Err(PSError::ParseError {
                            msg: format!("Failed to unescape text: {}", err),
                        })
                    }
                    Ok(cow) => {
                        value.push_str(&cow);
                    }
                },
                Ok(Event::End(val_end)) => match val_end.name().as_ref() {
                    b"value" => break,
                    _ => {
                        return Err(PSError::ParseError {
                            msg: format!("Unknown element closed in value: {:#?}", val_end.name()),
                        })
                    }
                },
                Ok(_) => {}
            }
        }
        return Ok(value);
    }

    /// Reads value elements nested under a property.
    fn read_values(reader: &mut Reader<&[u8]>, pname: &str) -> PSResult<Vec<String>> {
        let mut values = Vec::new();
        loop {
            match reader.read_event() {
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
                _ => {}
            }
        }
        return Ok(values);
    }
}

impl PSMLObject for Property {
    /// Reads a property from a reader and the start element.
    fn from_psml(reader: &mut Reader<&[u8]>, elem: BytesStart) -> PSResult<Property> {
        if elem.name().as_ref() != b"property" {
            return Err(PSError::ParseError {
                msg: format!(
                    "Trying to parse property from start of element: {:?}",
                    elem.name()
                ),
            });
        };

        let mut pname = None;
        let mut ptitle = None;
        let mut pvals = Vec::new();
        for attr_res in elem.attributes() {
            match attr_res {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed to decode property attribute: {:?}", err),
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
                msg: "Property missing name attribute".to_string(),
            });
        }

        pvals.extend(Property::read_values(reader, pname.as_ref().unwrap())?);

        return Ok(Property {
            name: pname.unwrap(),
            title: ptitle,
            value: pvals,
        });
    }

    fn to_psml(&self, writer: &mut Writer<&mut [u8]>) -> PSResult<()> {
        let mut prop_elem = BytesStart::new("property");
        prop_elem.push_attribute(Attribute {
            key: QName("name".as_bytes()),
            value: Cow::Borrowed(self.name.as_bytes()),
        });
        prop_elem.push_attribute(Attribute {
            key: QName("title".as_bytes()),
            value: Cow::Borrowed(self.title.as_ref().unwrap_or(&"".to_string()).as_bytes()),
        });

        let single_val = self.value.len() <= 1;
        if single_val == true {
            prop_elem.push_attribute(Attribute {
                key: QName("value".as_bytes()),
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

        return Ok(());
    }
}
