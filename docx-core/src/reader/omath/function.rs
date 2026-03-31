use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathFunction {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut function_name = None;
        let mut base = None;
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name: start_name,
                    attributes,
                    ..
                }) => {
                    if start_name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&start_name.local_name).unwrap() {
                        OMathXMLElement::FunctionProperty => {
                            property = Some(OMathFunctionProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::FunctionName => {
                            function_name = Some(OMathFunctionName::read(r, &attributes)?);
                        }
                        OMathXMLElement::Base => {
                            base = Some(OMathBase::read(r, &attributes)?);
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Function
                    {
                        return Ok(OMathFunction {
                            name: function_name.ok_or(ReaderError::XMLReadError)?,
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

impl ElementReader for OMathFunctionProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathFunctionProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::ControlProperty
                    {
                        property.control_property =
                            Some(OMathControlProperty::read(r, &attributes)?);
                    } else {
                        return Err(ReaderError::XMLReadError);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::FunctionProperty
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

impl ElementReader for OMathFunctionName {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut value = OMathFunctionName::new();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::ArgumentProperty => {
                            value.property = Some(OMathArgumentProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::ControlProperty => {
                            value.control_property =
                                Some(OMathControlProperty::read(r, &attributes)?);
                        }
                        _ => {
                            if let Some(child) = omath_child_from_start(r, &name, &attributes)? {
                                value.children.push(child);
                            } else {
                                return Err(ReaderError::XMLReadError);
                            }
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::FunctionName
                    {
                        return Ok(value);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}
