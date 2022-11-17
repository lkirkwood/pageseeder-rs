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
use quick_xml::{
    events::{attributes::Attribute, BytesEnd, BytesStart, BytesText, Event},
    name::QName,
    Reader, Writer,
};

use super::model::{
    Fragment, PropertiesFragment, PropertyDatatype, PropertyValue, XRef, XRefDisplayKind, XRefKind,
};

//// Macros

/// Returns a PSError::ParseError complaining of an unexpected element called "name".
/// Optionally you can provide whether the element was closed or opened with "op",
/// and additional detail as to where the error occured with "context".
macro_rules! unexpected_elem {
    ($name:expr) => {
        Err(PSError::ParseError {
            msg: format!("Unexpected element: {}", $name),
        })
    };
    ($name:expr, $op:expr) => {
        Err(PSError::ParseError {
            msg: format!("Unexpected element {}: {}", $op, !$name),
        })
    };
    ($name:expr, $op:expr, $context:expr) => {
        Err(PSError::ParseError {
            msg: format!("Unexpected element {} in {}: {}", $op, $context, $name),
        })
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

    /// Returns an instance of this psmlobject from a reader which has just read the start tag for this object.
    fn from_psml<R: BufRead>(reader: &mut Reader<R>, elem: BytesStart) -> PSResult<Self>;
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
                other => return unexpected_elem!(String::from_utf8_lossy(other), "closed", "xref"),
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
                other => {
                    return unexpected_elem!(
                        String::from_utf8_lossy(other),
                        "opened",
                        "property with datatype string or date"
                    )
                }
            },
            Event::End(elem) => match elem.name().as_ref() {
                b"value" => {
                    values.push(PropertyValue::String(current_val));
                    current_val = String::new();
                }
                b"property" => break,
                other => {
                    return unexpected_elem!(
                        String::from_utf8_lossy(other),
                        "closed",
                        "property with datatype string or date"
                    )
                }
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
            Event::End(elem) => match elem.name().as_ref() {
                b"property" => break,
                other => {
                    return unexpected_elem!(
                        String::from_utf8_lossy(other),
                        "closed",
                        "xref property"
                    )
                }
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
                other => {
                    return unexpected_elem!(
                        String::from_utf8_lossy(other),
                        "opened",
                        "link property"
                    )
                }
            },
            Event::End(elem) => match elem.name().as_ref() {
                b"link" => {
                    event_buf.push(Event::End(elem));
                    values.push(PropertyValue::Link(event_buf));
                    event_buf = Vec::new();
                }
                b"property" => break,
                other => {
                    return unexpected_elem!(
                        String::from_utf8_lossy(other),
                        "closed",
                        "link property"
                    )
                }
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
                other => {
                    return unexpected_elem!(
                        String::from_utf8_lossy(other),
                        "opened",
                        "markdown property"
                    )
                }
            },
            Event::End(elem) => match elem.name().as_ref() {
                b"markdown" => {
                    event_buf.push(Event::End(elem));
                    values.push(PropertyValue::Markdown(event_buf));
                    event_buf = Vec::new();
                }
                b"property" => break,
                other => {
                    return unexpected_elem!(
                        String::from_utf8_lossy(other),
                        "closed",
                        "markdown property"
                    )
                }
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
                b"heading" | b"para" | b"list" | b"nlist" | b"preformat" | b"br" | b"bold"
                | b"italic" | b"inline" | b"monospace" | b"underline" => {
                    event_buf.push(Event::Start(elem))
                }
                other => {
                    return unexpected_elem!(
                        String::from_utf8_lossy(other),
                        "opened",
                        "markup property"
                    )
                }
            },
            Event::End(elem) => match elem.name().as_ref() {
                b"heading" | b"para" | b"list" | b"nlist" | b"preformat" | b"br" | b"bold"
                | b"italic" | b"inline" | b"monospace" | b"underline" => {
                    event_buf.push(Event::End(elem))
                }
                b"property" => {
                    values.push(PropertyValue::Markup(event_buf));
                    break;
                }
                other => {
                    return unexpected_elem!(
                        String::from_utf8_lossy(other),
                        "closed",
                        "markup property"
                    )
                }
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
                        other => {
                            return unexpected_elem!(
                                String::from_utf8_lossy(other),
                                "closed",
                                "property"
                            )
                        }
                    },
                    _ => {}
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

// PropertiesFragment

/// Reads properties inside a properties-fragment from a reader.
fn read_properties<'a, R: BufRead>(
    reader: &'a mut Reader<R>,
    frag_id: &str,
) -> PSResult<Vec<Property>> {
    let mut buf = Vec::new();
    let mut props = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Err(err) => {
                return Err(PSError::ParseError {
                    msg: format!(
                        "Failed to read events after properties-fragment {} start: {:?}",
                        frag_id, err
                    ),
                })
            }
            Ok(Event::Start(prop_start)) => props.push(Property::from_psml(reader, prop_start)?),
            Ok(Event::End(elem_end)) => match elem_end.name().as_ref() {
                b"properties-fragment" => break,
                other => {
                    return unexpected_elem!(
                        String::from_utf8_lossy(other),
                        "closed",
                        "properties fragment"
                    )
                }
            },
            Ok(Event::Eof) => break,
            _ => {}
        }
    }

    return Ok(props);
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
        let properties = read_properties(reader, id.as_ref())?;

        return Ok(PropertiesFragment {
            id,
            frag_type,
            labels,
            properties,
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
        let mut buf = Vec::new();
        loop {
            match reader.read_event_into(&mut buf) {
                Err(err) => {
                    return Err(PSError::ParseError {
                        msg: format!("Failed reading events in fragment {}: {:?}", id, err),
                    })
                }
                Ok(Event::End(elem_end)) => match elem_end.name().as_ref() {
                    b"fragment" => break,
                    _ => events.push(Event::End(elem_end.into_owned())),
                },
                Ok(Event::Eof) => {
                    return Err(PSError::ParseError {
                        msg: format!("Unexpected EOF in fragment {}", id),
                    })
                }
                Ok(event) => events.push(event.into_owned()),
            }
        }

        return Ok(Fragment {
            id,
            frag_type,
            labels,
            content: events,
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
