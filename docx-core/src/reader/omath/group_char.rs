use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathGroupChar {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut base = None;
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::GroupCharProperty => {
                            property = Some(OMathGroupCharProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::Base => base = Some(OMathBase::read(r, &attributes)?),
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::GroupChar
                    {
                        return Ok(OMathGroupChar {
                            base: base.ok_or(ReaderError::XMLReadError)?,
                            property,
                        });
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathGroupCharProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathGroupCharProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::AccentChar => {
                            property.accent_char = omath_val(&attributes)
                        }
                        OMathXMLElement::Position => {
                            property.position = Some(
                                OMathBarPositionType::from_str(
                                    &omath_val(&attributes).ok_or(ReaderError::XMLReadError)?,
                                )
                                .map_err(|_| ReaderError::XMLReadError)?,
                            );
                        }
                        OMathXMLElement::VerticalJustification => {
                            property.vertical_justification = Some(
                                OMathBarPositionType::from_str(
                                    &omath_val(&attributes).ok_or(ReaderError::XMLReadError)?,
                                )
                                .map_err(|_| ReaderError::XMLReadError)?,
                            );
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
                            == OMathXMLElement::GroupCharProperty
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
