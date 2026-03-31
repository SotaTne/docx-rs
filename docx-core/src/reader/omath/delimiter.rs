use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathDelimiter {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut delimiter = OMathDelimiter::new();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::DelimiterProperty => {
                            delimiter.property =
                                Some(OMathDelimiterProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::Base => {
                            delimiter.elements.push(OMathBase::read(r, &attributes)?);
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Delimiter
                    {
                        return Ok(delimiter);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathDelimiterProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathDelimiterProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::BeginChar => property.begin_char = omath_val(&attributes),
                        OMathXMLElement::EndChar => property.end_char = omath_val(&attributes),
                        OMathXMLElement::SeparatorChar => {
                            property.separator_char = omath_val(&attributes)
                        }
                        OMathXMLElement::Grow => property.grow = Some(omath_bool(&attributes)),
                        OMathXMLElement::Shape => {
                            property.shape = Some(OMathShapeDelimiterType::from_str(
                                &omath_val(&attributes).ok_or(ReaderError::XMLReadError)?,
                            )?);
                        }
                        OMathXMLElement::ControlProperty => {
                            property.control_property =
                                Some(OMathControlProperty::read(r, &attributes)?)
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::DelimiterProperty
                    {
                        return Ok(property);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
