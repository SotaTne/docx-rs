use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathNary {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut sub_argument = None;
        let mut super_argument = None;
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
                        OMathXMLElement::NaryProperty => {
                            property = Some(OMathNaryProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::SubArgument => {
                            sub_argument = Some(OMathSubArgument::read(r, &attributes)?);
                        }
                        OMathXMLElement::SuperArgument => {
                            super_argument = Some(OMathSuperArgument::read(r, &attributes)?);
                        }
                        OMathXMLElement::Base => base = Some(OMathBase::read(r, &attributes)?),
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Nary
                    {
                        return Ok(OMathNary {
                            sub_argument: sub_argument.ok_or(ReaderError::XMLReadError)?,
                            super_argument: super_argument.ok_or(ReaderError::XMLReadError)?,
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

impl ElementReader for OMathNaryProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathNaryProperty::default();
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
                            property.operator_char = omath_val(&attributes)
                        }
                        OMathXMLElement::LimitLocation => {
                            property.limit_location = Some(OMathLimitLocationType::from_str(
                                &omath_val(&attributes).ok_or(ReaderError::XMLReadError)?,
                            )?);
                        }
                        OMathXMLElement::Grow => property.grow = Some(omath_bool(&attributes)),
                        OMathXMLElement::HideSubArgument => {
                            property.hide_sub_argument = Some(omath_bool(&attributes));
                        }
                        OMathXMLElement::HideSuperArgument => {
                            property.hide_super_argument = Some(omath_bool(&attributes));
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
                            == OMathXMLElement::NaryProperty
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
