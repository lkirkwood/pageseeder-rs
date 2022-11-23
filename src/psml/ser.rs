#[cfg(test)]
mod tests;

use std::{
    borrow::Cow,
    io::{BufRead, Write},
};

use crate::{
    error::{PSError, PSResult},
    psml::model::Property,
};
use indexmap::IndexMap;
use quick_xml::{
    events::{attributes::Attribute, BytesEnd, BytesStart, BytesText, Event},
    name::QName,
    Reader, Writer,
};

use super::model::{
    Fragment, Fragments, PropertiesFragment, PropertyDatatype, PropertyValue, Publication, Section,
    XRef, XRefDisplayKind, XRefKind,
};

//// Macros

/// Returns a PSError::ParseError complaining of an unexpected element in the context.
/// Example: unexpected_elem(elem, "opened in property")
macro_rules! unexpected_elem {
    ($elem:expr, $context:expr) => {
        return Err(PSError::ParseError {
            msg: format!(
                "Unexpected element {}: {}",
                $context,
                String::from_utf8_lossy($elem)
            ),
        })
    };
}

/// The names of the elements that may be in a markup property.
macro_rules! markup_elem_names {
    () => {
        b"heading"
            | b"para"
            | b"list"
            | b"nlist"
            | b"preformat"
            | b"br"
            | b"bold"
            | b"italic"
            | b"inline"
            | b"monospace"
            | b"underline"
            | b"image"
    };
}

/// The names of the elements that may be in a fragment.
macro_rules! fragment_elem_names {
    () => {
        b"block"
            | b"blockxref"
            | b"table"
            | b"preformat"
            | b"heading"
            | b"para"
            | b"br"
            | b"bold"
            | b"italic"
            | b"inline"
            | b"monospace"
            | b"underline"
            | b"image"
    };
}

// Conveniece functions

/// Reads an event from a reader and returns a PSResult.
fn read_event<'a, R: BufRead>(reader: &'a mut Reader<R>) -> PSResult<Event<'static>> {
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

/// Reads attributes from an element and returns them.
fn read_attrs<'a>(elem: &'a BytesStart) -> PSResult<Vec<Attribute<'a>>> {
    let mut attrs = Vec::new();
    for attr_res in elem.attributes() {
        match attr_res {
            Err(err) => {
                return Err(PSError::ParseError {
                    msg: format!(
                        "Failed to read attributes on {}: {:?}",
                        String::from_utf8_lossy(elem.name().0),
                        err
                    ),
                })
            }
            Ok(attr) => attrs.push(attr),
        }
    }
    return Ok(attrs);
}

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

/// Decodes and attribute value when it is a comma-separated list of strings,
/// and returns a PSResult.
fn decode_list_attr<'a, R: BufRead>(
    reader: &'a Reader<R>,
    attr: Attribute,
) -> PSResult<Vec<String>> {
    let string = decode_attr(reader, attr)?;
    return Ok(string
        .split([','])
        .map(|s| s.to_string())
        .collect::<Vec<String>>());
}

fn decode_bool_attr<'a, R: BufRead>(reader: &'a Reader<R>, attr: Attribute) -> PSResult<bool> {
    let attr_name = attr.key.to_owned();
    match decode_attr(reader, attr)?.as_ref() {
        "true" => return Ok(true),
        "false" => return Ok(false),
        other => {
            return Err(PSError::ParseError {
                msg: format!(
                    "Unexpected value for boolean attribute {:?}: {}",
                    attr_name, other
                ),
            })
        }
    }
}

/// Writes a attribute to an element.
fn write_attr(elem: &mut BytesStart, key: &str, value: &[u8]) {
    elem.push_attribute(Attribute {
        key: QName(key.as_bytes()),
        value: Cow::Borrowed(value),
    });
}

/// Writes a attribute to an element if it is Some.
fn write_attr_if_some(elem: &mut BytesStart, key: &str, value: Option<&String>) {
    if value.is_some() {
        write_attr(elem, key, value.unwrap().as_bytes())
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
                    String::from_utf8_lossy(&name),
                    err
                ),
            })
        }
    }
}

/// Writes an element start to a writer and returns a PSResult.
fn write_elem_empty<W: Write>(writer: &mut Writer<W>, elem: BytesStart) -> PSResult<()> {
    let name = elem.name().0.to_owned();
    match writer.write_event(Event::Empty(elem)) {
        Ok(_) => return Ok(()),
        Err(err) => {
            return Err(PSError::ParseError {
                msg: format!(
                    "Error writing element {} start to writer: {:?}",
                    String::from_utf8_lossy(&name),
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
                    String::from_utf8_lossy(&name),
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

//// PSMLObject

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

    /// Returns an instance from the start element for this obj and its reader.
    /// Consumes events until the end element for this obj.
    /// Set empty to true if the starting element is empty.
    fn from_psml<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Self>;

    /// Returns an instance from the empty element for this obj and its reader.
    /// For objs which are not allowed to be denoted by empty elements, this returns an error.
    #[allow(unused_variables)]
    fn from_psml_empty<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Self> {
        return Err(PSError::ParseError {
            msg: format!("Element {} cannot be empty!", Self::elem_name()),
        });
    }

    /// Writes this object to a writer as psml.
    fn to_psml<W: Write>(&self, writer: &mut Writer<W>) -> PSResult<()>;
}

//// Fragment content

// XRef

/// Reads the text content of an xref from a reader.
fn read_xref_content<'a, R: BufRead>(reader: &'a mut Reader<R>) -> PSResult<String> {
    let mut outstr = String::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(err) => {
                return Err(PSError::ParseError {
                    msg: format!("Failed reading contents of xref: {:?}", err),
                })
            }
            Ok(Event::Text(text)) => match String::from_utf8(text.to_vec()) {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed decoding xref content as utf-8: {}", err),
                    })
                }
                Ok(string) => outstr.push_str(&string),
            },
            Ok(Event::End(elem_end)) => match elem_end.name().as_ref() {
                b"xref" => break,
                other => unexpected_elem!(other, "closed in xref"),
            },
            _ => {}
        }
    }

    return Ok(outstr);
}

impl PSMLObject for XRef {
    fn elem_name() -> &'static str {
        return "xref";
    }

    fn from_psml<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<XRef> {
        Self::match_elem_name(&elem)?;

        let mut uriid = None;
        let mut docid = None;
        let mut href = None;
        let mut content = String::new();
        let mut config = None;
        let mut display = XRefDisplayKind::Document;
        let mut frag_id = None;
        let mut labels = Vec::new();
        let mut level = None;
        let mut reverselink = true;
        let mut reversetitle = None;
        let mut title = None;
        let mut xref_type = None;

        for attr_res in elem.attributes() {
            match attr_res {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed to read attribute on xref: {}", err),
                    })
                }
                Ok(attr) => match attr.key.as_ref() {
                    b"uriid" => uriid = Some(decode_attr(reader, attr)?),
                    b"docid" => docid = Some(decode_attr(reader, attr)?),
                    b"href" => href = Some(decode_attr(reader, attr)?),
                    b"config" => config = Some(decode_attr(reader, attr)?),
                    b"display" => display = XRefDisplayKind::from_str(&decode_attr(reader, attr)?)?,
                    b"frag" => frag_id = Some(decode_attr(reader, attr)?),
                    b"labels" => labels.extend(
                        decode_attr(reader, attr)?
                            .split(',')
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>(),
                    ),
                    b"level" => level = Some(decode_attr(reader, attr)?),
                    b"reverselink" => match decode_attr(reader, attr)?.as_ref() {
                        "true" => reverselink = true,
                        "false" => reverselink = false,
                        other => {
                            return Err(PSError::ParseError {
                                msg: format!("Unknown value for reverselink: {}", other),
                            })
                        }
                    },
                    b"reversetitle" => reversetitle = Some(decode_attr(reader, attr)?),
                    b"title" => title = Some(decode_attr(reader, attr)?),
                    b"type" => xref_type = Some(XRefKind::from_str(&decode_attr(reader, attr)?)?),
                    _ => {}
                },
            }
        }

        content.push_str(&read_xref_content(reader)?);

        return Ok(XRef {
            uriid,
            docid,
            href,
            content,
            config,
            display,
            frag_id: frag_id.unwrap_or("default".to_string()),
            labels,
            level,
            reverselink,
            reversetitle,
            title,
            xref_type,
        });
    }

    // TODO find a better solution than copy paste
    fn from_psml_empty<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<XRef> {
        Self::match_elem_name(&elem)?;

        let mut uriid = None;
        let mut docid = None;
        let mut href = None;
        let mut config = None;
        let mut display = XRefDisplayKind::Document;
        let mut frag_id = None;
        let mut labels = Vec::new();
        let mut level = None;
        let mut reverselink = true;
        let mut reversetitle = None;
        let mut title = None;
        let mut xref_type = None;

        for attr_res in elem.attributes() {
            match attr_res {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed to read attribute on xref: {}", err),
                    })
                }
                Ok(attr) => match attr.key.as_ref() {
                    b"uriid" => uriid = Some(decode_attr(reader, attr)?),
                    b"docid" => docid = Some(decode_attr(reader, attr)?),
                    b"href" => href = Some(decode_attr(reader, attr)?),
                    b"config" => config = Some(decode_attr(reader, attr)?),
                    b"display" => display = XRefDisplayKind::from_str(&decode_attr(reader, attr)?)?,
                    b"frag" => frag_id = Some(decode_attr(reader, attr)?),
                    b"labels" => labels.extend(
                        decode_attr(reader, attr)?
                            .split(',')
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>(),
                    ),
                    b"level" => level = Some(decode_attr(reader, attr)?),
                    b"reverselink" => match decode_attr(reader, attr)?.as_ref() {
                        "true" => reverselink = true,
                        "false" => reverselink = false,
                        other => {
                            return Err(PSError::ParseError {
                                msg: format!("Unknown value for reverselink: {}", other),
                            })
                        }
                    },
                    b"reversetitle" => reversetitle = Some(decode_attr(reader, attr)?),
                    b"title" => title = Some(decode_attr(reader, attr)?),
                    b"type" => xref_type = Some(XRefKind::from_str(&decode_attr(reader, attr)?)?),
                    _ => {}
                },
            }
        }

        return Ok(XRef {
            uriid,
            docid,
            href,
            content: String::new(),
            config,
            display,
            frag_id: frag_id.unwrap_or("default".to_string()),
            labels,
            level,
            reverselink,
            reversetitle,
            title,
            xref_type,
        });
    }

    fn to_psml<W: Write>(&self, writer: &mut Writer<W>) -> PSResult<()> {
        let mut elem_start = BytesStart::new("xref");
        write_attr_if_some(&mut elem_start, "uriid", self.uriid.as_ref());
        write_attr_if_some(&mut elem_start, "docid", self.docid.as_ref());
        write_attr_if_some(&mut elem_start, "href", self.href.as_ref());
        write_attr_if_some(&mut elem_start, "config", self.config.as_ref());

        if self.display != XRefDisplayKind::Document {
            write_attr(&mut elem_start, "display", self.display.to_str().as_bytes());
        }

        write_attr(&mut elem_start, "frag", self.frag_id.as_bytes());

        if self.labels.len() > 0 {
            write_attr(&mut elem_start, "labels", self.labels.join(",").as_bytes())
        }

        write_attr_if_some(&mut elem_start, "level", self.level.as_ref());

        if self.reverselink == false {
            write_attr(&mut elem_start, "reverselink", "false".as_bytes());
        }

        write_attr_if_some(&mut elem_start, "title", self.title.as_ref());

        if self.xref_type.is_some() {
            write_attr(
                &mut elem_start,
                "type",
                self.xref_type.as_ref().unwrap().to_str().as_bytes(),
            )
        }

        write_elem_start(writer, elem_start)?;
        write_text(writer, BytesText::new(&self.content))?;
        write_elem_end(writer, BytesEnd::new("xref"))?;

        return Ok(());
    }
}

// Property

/// Reads values under a property with string or date datatypes.
fn read_string_property_values<'a, R: BufRead>(
    reader: &'a mut Reader<R>,
) -> PSResult<Vec<PropertyValue>> {
    let mut values: Vec<PropertyValue> = Vec::new();
    let mut current_val = String::new();
    loop {
        match read_event(reader)? {
            Event::Start(elem) => match elem.name().as_ref() {
                b"value" => {}
                other => unexpected_elem!(other, "opened in string/date property"),
            },
            Event::Empty(elem) => match elem.name().as_ref() {
                b"value" => values.push(PropertyValue::String(String::new())),
                other => unexpected_elem!(other, "closed in string/date property"),
            },
            Event::End(elem) => match elem.name().as_ref() {
                b"value" => {
                    values.push(PropertyValue::String(current_val));
                    current_val = String::new();
                }
                b"property" => break,
                other => unexpected_elem!(other, "closed in property value"),
            },
            Event::Text(text) => current_val.push_str(&String::from_utf8_lossy(&text)),
            _ => {}
        }
    }
    return Ok(values);
}

/// Reads values under a property with xref datatype.
fn read_xref_property_values<'a, R: BufRead>(
    reader: &'a mut Reader<R>,
) -> PSResult<Vec<PropertyValue>> {
    let mut values = Vec::new();
    loop {
        match read_event(reader)? {
            Event::Start(elem) => values.push(PropertyValue::XRef(XRef::from_psml(reader, elem)?)),
            Event::Empty(elem) => {
                values.push(PropertyValue::XRef(XRef::from_psml_empty(reader, elem)?))
            }
            Event::End(elem) => match elem.name().as_ref() {
                b"property" => break,
                other => unexpected_elem!(other, "closed in property"),
            },
            _ => {}
        }
    }
    return Ok(values);
}

/// Reads values under a property with link datatype.
fn read_link_property_values<'a, R: BufRead>(
    reader: &'a mut Reader<R>,
) -> PSResult<Vec<PropertyValue>> {
    let mut event_buf = Vec::new();
    let mut values = Vec::new();
    loop {
        match read_event(reader)? {
            Event::Start(elem) => match elem.name().as_ref() {
                b"link" => event_buf.push(Event::Start(elem)),
                other => unexpected_elem!(other, "opened in link property"),
            },
            Event::Empty(elem) => match elem.name().as_ref() {
                b"link" => event_buf.push(Event::Empty(elem)),
                other => unexpected_elem!(other, "empty in link property"),
            },
            Event::End(elem) => match elem.name().as_ref() {
                b"link" => {
                    event_buf.push(Event::End(elem));
                    values.push(PropertyValue::Link(event_buf));
                    event_buf = Vec::new();
                }
                b"property" => break,
                other => unexpected_elem!(other, "closed in link property"),
            },
            other => event_buf.push(other),
        }
    }
    return Ok(values);
}

/// Reads values under a property with datatype markdown.
fn read_markdown_property_values<'a, R: BufRead>(
    reader: &'a mut Reader<R>,
) -> PSResult<Vec<PropertyValue>> {
    let mut values = Vec::new();
    let mut event_buf = Vec::new();
    loop {
        match read_event(reader)? {
            Event::Start(elem) => match elem.name().as_ref() {
                b"markdown" => event_buf.push(Event::Start(elem)),
                other => unexpected_elem!(other, "opened in markdown property"),
            },
            Event::Empty(elem) => match elem.name().as_ref() {
                b"markdown" => event_buf.push(Event::Empty(elem)),
                other => unexpected_elem!(other, "empty in markdown property"),
            },
            Event::End(elem) => match elem.name().as_ref() {
                b"markdown" => {
                    event_buf.push(Event::End(elem));
                    values.push(PropertyValue::Markdown(event_buf));
                    event_buf = Vec::new();
                }
                b"property" => break,
                other => unexpected_elem!(other, "closed in markdown property"),
            },
            Event::Eof => {
                return Err(PSError::ParseError {
                    msg: format!("Unexpected EOF in property."),
                })
            }
            other => event_buf.push(other),
        }
    }
    return Ok(values);
}

fn read_markup_property_values<'a, R: BufRead>(
    reader: &'a mut Reader<R>,
) -> PSResult<Vec<PropertyValue>> {
    let mut values = Vec::new();
    let mut event_buf = Vec::new();
    loop {
        match read_event(reader)? {
            Event::Start(elem) => match elem.name().as_ref() {
                markup_elem_names!() => event_buf.push(Event::Start(elem)),
                other => unexpected_elem!(other, "opened in markup property"),
            },
            Event::Empty(elem) => match elem.name().as_ref() {
                markup_elem_names!() => event_buf.push(Event::Empty(elem)),
                other => unexpected_elem!(other, "empty in markup property"),
            },
            Event::End(elem) => match elem.name().as_ref() {
                markup_elem_names!() => event_buf.push(Event::End(elem)),
                b"property" => {
                    values.push(PropertyValue::Markup(event_buf));
                    break;
                }
                other => unexpected_elem!(other, "closed in markup property"),
            },
            Event::Eof => {
                return Err(PSError::ParseError {
                    msg: "Unexpected EOF in property.".to_string(),
                })
            }
            other => event_buf.push(other),
        }
    }
    return Ok(values);
}

fn read_property_values<'a, R: BufRead>(
    reader: &'a mut Reader<R>,
    datatype: &PropertyDatatype,
) -> PSResult<Vec<PropertyValue>> {
    match datatype {
        PropertyDatatype::String | PropertyDatatype::Date => {
            return read_string_property_values(reader)
        }
        PropertyDatatype::XRef => return read_xref_property_values(reader),
        PropertyDatatype::Link => return read_link_property_values(reader),
        PropertyDatatype::Markdown => return read_markdown_property_values(reader),
        PropertyDatatype::Markup => return read_markup_property_values(reader),
    }
}

/// Writes values for a property with multiple.
fn write_property_values<W: Write>(
    writer: &mut Writer<W>,
    values: &Vec<PropertyValue>,
) -> PSResult<()> {
    for val in values {
        match val {
            PropertyValue::String(string) => {
                write_elem_start(writer, BytesStart::new("value"))?;
                write_text(writer, BytesText::new(string))?;
                write_elem_end(writer, BytesEnd::new("value"))?;
            }
            PropertyValue::XRef(xref) => {
                xref.to_psml(writer)?;
            }
            PropertyValue::Link(events) => {
                for event in events {
                    match writer.write_event(event) {
                        Err(err) => {
                            return Err(PSError::ParseError {
                                msg: format!("Failed to write property link content: {:?}", err),
                            })
                        }
                        Ok(_) => {}
                    }
                }
            }
            PropertyValue::Markdown(events) => {
                for event in events {
                    match writer.write_event(event) {
                        Err(err) => {
                            return Err(PSError::ParseError {
                                msg: format!(
                                    "Failed to write property markdown content: {:?}",
                                    err
                                ),
                            })
                        }
                        Ok(_) => {}
                    }
                }
            }
            PropertyValue::Markup(events) => {
                for event in events {
                    match writer.write_event(event) {
                        Err(err) => {
                            return Err(PSError::ParseError {
                                msg: format!("Failed to write property markup content: {:?}", err),
                            })
                        }
                        Ok(_) => {}
                    }
                }
            }
        }
    }
    return Ok(());
}

impl PSMLObject for Property {
    fn elem_name() -> &'static str {
        return "property";
    }

    fn from_psml<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Property> {
        Self::match_elem_name(&elem)?;
        let mut pname = None;
        let mut title = None;
        let mut multiple = false;
        let mut datatype = PropertyDatatype::String;
        let mut values = Vec::new();
        for attr_res in elem.attributes() {
            match attr_res {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed to get property attribute: {:?}", err),
                    })
                }
                Ok(attr) => {
                    match attr.key.as_ref() {
                        b"name" => {
                            pname = Some(decode_attr(reader, attr)?);
                        }
                        b"title" => {
                            title = Some(decode_attr(reader, attr)?);
                        }
                        b"value" => {
                            if multiple == true {
                                return Err(PSError::ParseError { msg: format!("Cannot use value attribute on property when multiple = true.") });
                            } else {
                                values.push(PropertyValue::String(decode_attr(reader, attr)?));
                            }
                        }
                        b"multiple" => match decode_attr(reader, attr)?.as_ref() {
                            "true" => multiple = true,
                            "false" => multiple = false,
                            other => {
                                return Err(PSError::ParseError {
                                    msg: format!("Unrecognized value for multiple attr: {}", other),
                                })
                            }
                        },
                        b"datatype" => {
                            datatype =
                                PropertyDatatype::from_str(decode_attr(reader, attr)?.as_ref())
                        }
                        _ => {}
                    }
                }
            }
        }

        if (multiple == true)
            | (values.len() == 0)
            | ((datatype != PropertyDatatype::String) & (datatype != PropertyDatatype::Date))
        {
            values.extend(read_property_values(reader, &datatype)?);
        } else {
            // using value from attribute
            loop {
                match read_event(reader)? {
                    Event::End(elem_end) => match elem_end.name().as_ref() {
                        b"property" => break,
                        other => unexpected_elem!(other, "opened in property"),
                    },
                    _ => {}
                }
            }
        }

        if (multiple == false) & (values.len() > 1) {
            multiple = true;
        }

        if pname.is_none() {
            return Err(PSError::ParseError {
                msg: "Property missing required 'name' attribute".to_string(),
            });
        }

        return Ok(Property {
            name: pname.unwrap(),
            title,
            multiple,
            datatype,
            values,
        });
    }

    fn from_psml_empty<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Property> {
        Self::match_elem_name(&elem)?;
        let mut pname = None;
        let mut title = None;
        let mut multiple = false;
        let mut datatype = PropertyDatatype::String;
        let mut values = Vec::new();
        for attr_res in elem.attributes() {
            match attr_res {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed to get property attribute: {:?}", err),
                    })
                }
                Ok(attr) => {
                    match attr.key.as_ref() {
                        b"name" => {
                            pname = Some(decode_attr(reader, attr)?);
                        }
                        b"title" => {
                            title = Some(decode_attr(reader, attr)?);
                        }
                        b"value" => {
                            if multiple == true {
                                return Err(PSError::ParseError { msg: format!("Cannot use value attribute on property when multiple = true.") });
                            } else {
                                values.push(PropertyValue::String(decode_attr(reader, attr)?));
                            }
                        }
                        b"multiple" => match decode_attr(reader, attr)?.as_ref() {
                            "true" => multiple = true,
                            "false" => multiple = false,
                            other => {
                                return Err(PSError::ParseError {
                                    msg: format!("Unrecognized value for multiple attr: {}", other),
                                })
                            }
                        },
                        b"datatype" => {
                            datatype =
                                PropertyDatatype::from_str(decode_attr(reader, attr)?.as_ref())
                        }
                        _ => {}
                    }
                }
            }
        }

        if pname.is_none() {
            return Err(PSError::ParseError {
                msg: "Property missing required 'name' attribute".to_string(),
            });
        }

        return Ok(Property {
            name: pname.unwrap(),
            title,
            multiple,
            datatype,
            values,
        });
    }

    fn to_psml<W: Write>(&self, writer: &mut Writer<W>) -> PSResult<()> {
        let mut elem_start = BytesStart::new("property");
        write_attr(&mut elem_start, "name", self.name.as_bytes());
        write_attr_if_some(&mut elem_start, "title", self.title.as_ref());

        if self.multiple == true {
            write_attr(&mut elem_start, "multiple", "true".as_bytes());
        }

        if self.datatype != PropertyDatatype::String {
            write_attr(
                &mut elem_start,
                "datatype",
                self.datatype.to_str().as_bytes(),
            );
        }

        let num_values = self.values.len();
        if num_values == 0 {
            write_attr(&mut elem_start, "value", &[])
        } else if num_values == 1 {
            let val = self.values.get(0);
            if val.is_some() {
                match val.unwrap() {
                    PropertyValue::String(string) => {
                        write_attr(&mut elem_start, "value", string.as_bytes());
                    }
                    _ => {}
                }
            }
        }

        write_elem_start(writer, elem_start)?;
        if (num_values > 1)
            | ((self.datatype != PropertyDatatype::String)
                & (self.datatype != PropertyDatatype::Date))
        {
            write_property_values(writer, &self.values)?;
        }
        write_elem_end(writer, BytesEnd::new("property"))?;

        return Ok(());
    }
}

//// Fragments

/// Returns the id, frag_type and labels from a fragment element.
fn read_fragment_attrs<'a, R: BufRead>(
    reader: &'a mut Reader<R>,
    elem: &BytesStart,
) -> PSResult<(String, Option<String>, Vec<String>)> {
    let mut id = None;
    let mut frag_type = None;
    let mut labels = Vec::new();
    for attr in elem.attributes() {
        match attr {
            Err(err) => {
                return Err(PSError::ParseError {
                    msg: format!("Failed to get properties fragment attribute: {:?}", err),
                })
            }
            Ok(attr) => match attr.key.as_ref() {
                b"id" => id = Some(decode_attr(reader, attr)?),
                b"type" => frag_type = Some(decode_attr(reader, attr)?),
                b"labels" => labels.extend(decode_list_attr(reader, attr)?),
                _ => {}
            },
        }
    }

    if id.is_none() {
        return Err(PSError::ParseError {
            msg: "Properties fragment missing required 'id' attribute.".to_string(),
        });
    } else {
        return Ok((id.unwrap(), frag_type, labels));
    }
}

// PropertiesFragment

/// Reads properties inside a properties-fragment from a reader.
fn read_properties<'a, R: BufRead>(reader: &'a mut Reader<R>) -> PSResult<Vec<Property>> {
    let mut props = Vec::new();
    loop {
        match read_event(reader)? {
            Event::Start(prop_start) => props.push(Property::from_psml(reader, prop_start)?),
            Event::End(elem_end) => match elem_end.name().as_ref() {
                b"properties-fragment" => break,
                other => unexpected_elem!(other, "closed in properties fragment"),
            },
            Event::Eof => {
                return Err(PSError::ParseError {
                    msg: "Unexpected EOF in properties fragment.".to_string(),
                })
            }
            _ => {}
        }
    }

    return Ok(props);
}

impl PSMLObject for PropertiesFragment {
    fn elem_name() -> &'static str {
        return "properties-fragment";
    }

    fn from_psml<R: BufRead>(
        reader: &mut Reader<R>,
        elem: BytesStart,
    ) -> PSResult<PropertiesFragment> {
        Self::match_elem_name(&elem)?;
        let (id, frag_type, labels) = read_fragment_attrs(reader, &elem)?;
        let properties = read_properties(reader)?;

        return Ok(PropertiesFragment {
            id,
            frag_type,
            labels,
            properties,
        });
    }

    fn from_psml_empty<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Self> {
        Self::match_elem_name(&elem)?;
        let (id, frag_type, labels) = read_fragment_attrs(reader, &elem)?;

        return Ok(PropertiesFragment {
            id,
            frag_type,
            labels,
            properties: vec![],
        });
    }

    fn to_psml<W: Write>(&self, writer: &mut Writer<W>) -> PSResult<()> {
        let mut elem_start = BytesStart::new("properties-fragment");

        write_attr(&mut elem_start, "id", &self.id.as_bytes());
        write_attr_if_some(&mut elem_start, "type", self.frag_type.as_ref());
        if self.labels.len() > 0 {
            write_attr(&mut elem_start, "labels", &self.labels.join(",").as_bytes())
        }
        write_elem_start(writer, elem_start)?;

        for property in &self.properties {
            property.to_psml(writer)?;
        }

        write_elem_end(writer, BytesEnd::new("properties-fragment"))?;

        return Ok(());
    }
}

// Fragment

impl PSMLObject for Fragment {
    fn elem_name() -> &'static str {
        return "fragment";
    }

    fn from_psml<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Fragment> {
        Self::match_elem_name(&elem)?;
        let (id, frag_type, labels) = read_fragment_attrs(reader, &elem)?;
        let mut events = Vec::new();
        loop {
            match read_event(reader)? {
                Event::Start(elem_start) => match elem_start.name().as_ref() {
                    fragment_elem_names!() => events.push(Event::Start(elem_start.into_owned())),
                    other => unexpected_elem!(other, "opened in fragment"),
                },
                Event::Empty(elem_start) => match elem_start.name().as_ref() {
                    fragment_elem_names!() => events.push(Event::Empty(elem_start.into_owned())),
                    other => unexpected_elem!(other, "empty in fragment"),
                },
                Event::End(elem_end) => match elem_end.name().as_ref() {
                    b"fragment" => break,
                    fragment_elem_names!() => events.push(Event::End(elem_end.into_owned())),
                    other => unexpected_elem!(other, "closed in fragment"),
                },
                Event::Eof => {
                    return Err(PSError::ParseError {
                        msg: format!("Unexpected EOF in fragment {}", id),
                    })
                }
                event => events.push(event.into_owned()),
            }
        }

        return Ok(Fragment {
            id,
            frag_type,
            labels,
            content: events,
        });
    }

    fn from_psml_empty<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Self> {
        Self::match_elem_name(&elem)?;
        let (id, frag_type, labels) = read_fragment_attrs(reader, &elem)?;

        return Ok(Fragment {
            id,
            frag_type,
            labels,
            content: vec![],
        });
    }

    fn to_psml<W: Write>(&self, writer: &mut Writer<W>) -> PSResult<()> {
        let mut elem_start = BytesStart::new("fragment");
        write_attr(&mut elem_start, "id", &self.id.as_bytes());
        write_attr_if_some(&mut elem_start, "type", self.frag_type.as_ref());
        if self.labels.len() > 0 {
            write_attr(&mut elem_start, "labels", &self.labels.join(",").as_bytes())
        }
        write_elem_start(writer, elem_start)?;

        for event in &self.content {
            match writer.write_event(event) {
                Ok(()) => {}
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed writing content of fragment {}: {:?}", self.id, err),
                    })
                }
            }
        }

        write_elem_end(writer, BytesEnd::new("fragment"))?;

        return Ok(());
    }
}

//// TODO XRefFragment

//// Section

/// Reads the content of a section and returns content title and fragments.
fn read_section_content<'a, R: BufRead>(
    reader: &'a mut Reader<R>,
) -> PSResult<(Option<String>, IndexMap<String, Fragments>)> {
    let mut title = None;
    let mut fragments = IndexMap::new();
    let mut in_title = false;
    loop {
        match read_event(reader)? {
            Event::Start(elem_start) => match elem_start.name().as_ref() {
                b"title" => in_title = true,
                b"fragment" => {
                    let frag = Fragment::from_psml(reader, elem_start)?;
                    fragments.insert(frag.id.to_owned(), Fragments::Normal(frag));
                }
                b"properties-fragment" => {
                    let frag = PropertiesFragment::from_psml(reader, elem_start)?;
                    fragments.insert(frag.id.to_owned(), Fragments::Properties(frag));
                }
                b"media-fragment" => todo!("Implement media fragment."),
                b"xref-fragment" => todo!("Implement xref fragment."),
                other => unexpected_elem!(other, "opened in section"),
            },
            Event::Empty(elem) => match elem.name().as_ref() {
                b"title" => in_title = true,
                b"fragment" => {
                    let frag = Fragment::from_psml_empty(reader, elem)?;
                    fragments.insert(frag.id.to_owned(), Fragments::Normal(frag));
                }
                b"properties-fragment" => {
                    let frag = PropertiesFragment::from_psml_empty(reader, elem)?;
                    fragments.insert(frag.id.to_owned(), Fragments::Properties(frag));
                }
                b"media-fragment" => todo!("Implement media fragment."),
                b"xref-fragment" => todo!("Implement xref fragment."),
                other => unexpected_elem!(other, "empty in section"),
            },
            Event::End(elem_end) => match elem_end.name().as_ref() {
                b"title" => in_title = false,
                b"section" => break,
                other => unexpected_elem!(other, "closed in section"),
            },
            Event::Text(bytes) => match in_title {
                true => match String::from_utf8(bytes.to_vec()) {
                    Err(err) => {
                        return Err(PSError::ParseError {
                            msg: format!(
                                "Failed to decode content title from utf-8 in section: {:?}",
                                err
                            ),
                        })
                    }
                    Ok(text) => title = Some(text),
                },
                false => {}
            },
            Event::Eof => {
                return Err(PSError::ParseError {
                    msg: "Unexpected EOF in Section.".to_string(),
                })
            }
            _ => {}
        }
    }

    return Ok((title, fragments));
}

/// Writes the section content title and fragments.
fn write_section_content<W: Write>(writer: &mut Writer<W>, section: &Section) -> PSResult<()> {
    if section.content_title.is_some() {
        write_elem_start(writer, BytesStart::new("title"))?;
        write_text(
            writer,
            BytesText::new(section.content_title.as_ref().unwrap()),
        )?;
        write_elem_end(writer, BytesEnd::new("title"))?;
    }
    for fragment in section.fragments.values() {
        match fragment {
            Fragments::Normal(frag) => frag.to_psml(writer)?,
            Fragments::Properties(frag) => frag.to_psml(writer)?,
            Fragments::XRef(_frag) => todo!("Implement PSMLObj for XRefFragment"),
            Fragments::Media(()) => todo!("Add media frag"),
        }
    }
    return Ok(());
}

impl PSMLObject for Section {
    fn elem_name() -> &'static str {
        return "section";
    }

    fn from_psml<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Self> {
        Self::match_elem_name(&elem)?;
        let mut id = None;
        let mut title = None;
        let mut edit = true;
        let mut lock = false;
        let mut overwrite = true;
        let mut fragment_types = Vec::new();
        for attr_res in elem.attributes() {
            match attr_res {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed reading attribute on section: {:?}", err),
                    })
                }
                Ok(attr) => match attr.key.as_ref() {
                    b"id" => id = Some(decode_attr(reader, attr)?),
                    b"title" => title = Some(decode_attr(reader, attr)?),
                    b"edit" => edit = decode_bool_attr(reader, attr)?,
                    b"lock" => lock = decode_bool_attr(reader, attr)?,
                    b"overwrite" => overwrite = decode_bool_attr(reader, attr)?,
                    b"fragmenttype" => fragment_types.extend(decode_list_attr(reader, attr)?),
                    other => {
                        return Err(PSError::ParseError {
                            msg: format!("Unexpected attribute on section: {:?}", other),
                        })
                    }
                },
            }
        }

        let (content_title, fragments) = read_section_content(reader)?;

        if id.is_none() {
            return Err(PSError::ParseError {
                msg: format!("Section missing required attribute id."),
            });
        } else {
            return Ok(Section {
                id: id.unwrap(),
                title,
                content_title,
                edit,
                lock,
                overwrite,
                fragment_types,
                fragments: IndexMap::from(fragments),
            });
        }
    }

    fn from_psml_empty<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Self> {
        Self::match_elem_name(&elem)?;
        let mut id = None;
        let mut title = None;
        let mut edit = true;
        let mut lock = false;
        let mut overwrite = true;
        let mut fragment_types = Vec::new();
        for attr_res in elem.attributes() {
            match attr_res {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed reading attribute on section: {:?}", err),
                    })
                }
                Ok(attr) => match attr.key.as_ref() {
                    b"id" => id = Some(decode_attr(reader, attr)?),
                    b"title" => title = Some(decode_attr(reader, attr)?),
                    b"edit" => edit = decode_bool_attr(reader, attr)?,
                    b"lock" => lock = decode_bool_attr(reader, attr)?,
                    b"overwrite" => overwrite = decode_bool_attr(reader, attr)?,
                    b"fragmenttype" => fragment_types.extend(decode_list_attr(reader, attr)?),
                    other => {
                        return Err(PSError::ParseError {
                            msg: format!("Unexpected attribute on section: {:?}", other),
                        })
                    }
                },
            }
        }

        if id.is_none() {
            return Err(PSError::ParseError {
                msg: format!("Section missing required attribute id."),
            });
        } else {
            return Ok(Section {
                id: id.unwrap(),
                title,
                content_title: None,
                edit,
                lock,
                overwrite,
                fragment_types,
                fragments: IndexMap::new(),
            });
        }
    }

    fn to_psml<W: Write>(&self, writer: &mut Writer<W>) -> PSResult<()> {
        let mut elem_start = BytesStart::new("section");
        write_attr(&mut elem_start, "id", &self.id.as_bytes());
        write_attr_if_some(&mut elem_start, "title", self.title.as_ref());

        if self.edit == false {
            write_attr(&mut elem_start, "edit", "false".as_bytes());
        }
        if self.lock == true {
            write_attr(&mut elem_start, "lock", "true".as_bytes());
        }
        if self.overwrite == false {
            write_attr(&mut elem_start, "overwrite", "false".as_bytes());
        }

        write_elem_start(writer, elem_start)?;
        write_section_content(writer, self)?;
        write_elem_end(writer, BytesEnd::new("section"))?;

        return Ok(());
    }
}

//// Document

// Publication

impl PSMLObject for Publication {
    fn elem_name() -> &'static str {
        return "publication";
    }

    fn from_psml<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Self> {
        Self::match_elem_name(&elem)?;
        let mut id = None;
        let mut pub_type = None;
        for attr in read_attrs(&elem)? {
            match attr.key.as_ref() {
                b"id" => id = Some(decode_attr(&reader, attr)?),
                b"type" => pub_type = Some(decode_attr(&reader, attr)?),
                _ => {}
            }
        }
        if id.is_none() {
            return Err(PSError::ParseError {
                msg: format!("Publication missing required attribute id."),
            });
        }
        loop {
            match read_event(reader)? {
                Event::End(elem) => match elem.name().as_ref() {
                    b"publication" => break,
                    other => unexpected_elem!(other, "closed in publication"),
                },
                Event::Start(elem) | Event::Empty(elem) => {
                    unexpected_elem!(&elem, "in publication")
                }
                Event::Eof => {
                    return Err(PSError::ParseError {
                        msg: "Unexpected EOF in publication.".to_string(),
                    })
                }
                _ => {}
            }
        }
        return Ok(Publication {
            id: id.unwrap(),
            pub_type,
        });
    }

    fn from_psml_empty<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Self> {
        Self::match_elem_name(&elem)?;
        let mut id = None;
        let mut pub_type = None;
        for attr in read_attrs(&elem)? {
            match attr.key.as_ref() {
                b"id" => id = Some(decode_attr(&reader, attr)?),
                b"type" => pub_type = Some(decode_attr(&reader, attr)?),
                _ => {}
            }
        }
        if id.is_none() {
            return Err(PSError::ParseError {
                msg: format!("Publication missing required attribute id."),
            });
        }
        return Ok(Publication {
            id: id.unwrap(),
            pub_type,
        });
    }

    fn to_psml<W: Write>(&self, writer: &mut Writer<W>) -> PSResult<()> {
        let mut elem = BytesStart::new("publication");
        write_attr(&mut elem, "id", &self.id.as_bytes());
        write_attr_if_some(&mut elem, "type", self.pub_type.as_ref());
        write_elem_empty(writer, elem)?;

        return Ok(());
    }
}
