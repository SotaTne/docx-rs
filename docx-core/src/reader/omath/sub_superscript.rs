use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathSubSuperscript {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut base = None;
        let mut sub_argument = None;
        let mut super_argument = None;
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::SubSuperscriptProperty => {
                            property = Some(OMathSubSuperscriptProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::Base => base = Some(OMathBase::read(r, &attributes)?),
                        OMathXMLElement::SubArgument => {
                            sub_argument = Some(OMathSubArgument::read(r, &attributes)?);
                        }
                        OMathXMLElement::SuperArgument => {
                            super_argument = Some(OMathSuperArgument::read(r, &attributes)?);
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::SubSuperscript
                    {
                        return Ok(OMathSubSuperscript {
                            base: base.ok_or(ReaderError::XMLReadError)?,
                            sub_argument: sub_argument.ok_or(ReaderError::XMLReadError)?,
                            super_argument: super_argument.ok_or(ReaderError::XMLReadError)?,
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

impl ElementReader for OMathSubSuperscriptProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathSubSuperscriptProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::AlignScripts => {
                            property.align_scripts = Some(omath_bool(&attributes));
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
                            == OMathXMLElement::SubSuperscriptProperty
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
