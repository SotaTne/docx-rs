use std::io::Read;

use super::*;

fn read_argument_container<R: Read, T>(
    r: &mut EventReader<R>,
    end: OMathXMLElement,
    mut make: impl FnMut(
        Vec<OMathChild>,
        Option<OMathArgumentProperty>,
        Option<OMathControlProperty>,
    ) -> T,
) -> Result<T, ReaderError> {
    let mut children = Vec::new();
    let mut property = None;
    let mut control_property = None;

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
                        property = Some(OMathArgumentProperty::read(r, &attributes)?);
                    }
                    OMathXMLElement::ControlProperty => {
                        control_property = Some(OMathControlProperty::read(r, &attributes)?);
                    }
                    _ => {
                        if let Some(child) = omath_child_from_start(r, &name, &attributes)? {
                            children.push(child);
                        } else {
                            return Err(ReaderError::XMLReadError);
                        }
                    }
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.prefix.as_deref() == Some("m")
                    && OMathXMLElement::from_str(&name.local_name).unwrap() == end
                {
                    return Ok(make(children, property, control_property));
                }
            }
            Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
            Err(_) => return Err(ReaderError::XMLReadError),
            _ => {}
        }
    }
}

impl ElementReader for OMath {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        Ok(Self {
            children: read_omath_children_until(r, OMathXMLElement::OMath)?,
        })
    }
}

impl ElementReader for OMathBase {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        read_argument_container(
            r,
            OMathXMLElement::Base,
            |children, property, control_property| OMathBase {
                children,
                property,
                control_property,
            },
        )
    }
}

impl ElementReader for OMathNumerator {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        read_argument_container(
            r,
            OMathXMLElement::Numerator,
            |children, property, control_property| OMathNumerator {
                children,
                property,
                control_property,
            },
        )
    }
}

impl ElementReader for OMathDenominator {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        read_argument_container(
            r,
            OMathXMLElement::Denominator,
            |children, property, control_property| OMathDenominator {
                children,
                property,
                control_property,
            },
        )
    }
}

impl ElementReader for OMathDegree {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        read_argument_container(
            r,
            OMathXMLElement::Degree,
            |children, property, control_property| OMathDegree {
                children,
                property,
                control_property,
            },
        )
    }
}

impl ElementReader for OMathLimit {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        read_argument_container(
            r,
            OMathXMLElement::Limit,
            |children, property, control_property| OMathLimit {
                children,
                property,
                control_property,
            },
        )
    }
}

impl ElementReader for OMathSubArgument {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        read_argument_container(
            r,
            OMathXMLElement::SubArgument,
            |children, property, control_property| OMathSubArgument {
                children,
                property,
                control_property,
            },
        )
    }
}

impl ElementReader for OMathSuperArgument {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        read_argument_container(
            r,
            OMathXMLElement::SuperArgument,
            |children, property, control_property| OMathSuperArgument {
                children,
                property,
                control_property,
            },
        )
    }
}

impl ElementReader for OMathArgumentProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathArgumentProperty::new();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::ArgumentSize
                    {
                        if let Some(val) = omath_val(&attributes) {
                            property = property
                                .argument_size(val.parse().map_err(|_| ReaderError::XMLReadError)?);
                        }
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::ArgumentProperty
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

impl ElementReader for OMathControlProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut run_property = None;
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() == Some("w")
                        && XMLElement::from_str(&name.local_name).unwrap()
                            == XMLElement::RunProperty
                    {
                        run_property = Some(RunProperty::read(r, &attributes)?);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::ControlProperty
                    {
                        return run_property
                            .map(OMathControlProperty::new)
                            .ok_or(ReaderError::XMLReadError);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn reads_omath_base_with_argument_property() {
        let xml = r#"<m:e>
  <m:argPr>
    <m:argSz m:val="-1" />
  </m:argPr>
  <m:r>
    <m:t>x</m:t>
  </m:r>
</m:e>
"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let _ = parser.next();
        let base = OMathBase::read(&mut parser, &[]).unwrap();
        assert_eq!(
            base,
            OMathBase {
                children: vec![OMathChild::Run(Box::new(OMathRun::new().add_text("x")))],
                property: Some(OMathArgumentProperty::new().argument_size(-1)),
                control_property: None,
            }
        );
    }
}
