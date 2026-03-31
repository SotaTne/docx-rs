use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathRun {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut run = OMathRun::new();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => match name.prefix.as_deref() {
                    Some("m") => match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::RunProperty => {
                            run.property = Some(OMathRunProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::Text => {
                            run.children
                                .push(OMathRunChild::Text(OMathText::read(r, &attributes)?));
                        }
                        OMathXMLElement::Break => {
                            let br = OMathManualBreak::read(r, &attributes)?;
                            run.property = Some(run.property.unwrap_or_default().manual_break(br));
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    },
                    Some("w") => {
                        if XMLElement::from_str(&name.local_name).unwrap() == XMLElement::Break {
                            let break_type = if let Some(v) = read(&attributes, "type") {
                                BreakType::from_str(&v)?
                            } else {
                                BreakType::TextWrapping
                            };
                            run.children
                                .push(OMathRunChild::Break(Break::new(break_type)));
                        }
                    }
                    _ => {}
                },
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Run
                    {
                        return Ok(run);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathText {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut text = String::new();
        loop {
            match r.next() {
                Ok(XmlEvent::Characters(value)) | Ok(XmlEvent::Whitespace(value)) => {
                    text.push_str(&value);
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Text
                    {
                        return Ok(OMathText::new(text));
                    }
                }
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathRunProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathRunProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::Style => {
                            property.style = Some(OMathStyleType::from_str(
                                &omath_val(&attributes).ok_or(ReaderError::XMLReadError)?,
                            )?);
                        }
                        OMathXMLElement::Script => {
                            property.script = Some(OMathScriptType::from_str(
                                &omath_val(&attributes).ok_or(ReaderError::XMLReadError)?,
                            )?);
                        }
                        OMathXMLElement::Alignment => {
                            property.align = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::Break => {
                            property.manual_break = Some(OMathManualBreak::read(r, &attributes)?);
                        }
                        OMathXMLElement::NormalText => {
                            property.normal_text = Some(omath_bool(&attributes))
                        }
                        OMathXMLElement::Literal => {
                            property.literal = Some(omath_bool(&attributes))
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::RunProperty
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

impl ElementReader for OMathManualBreak {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut value = OMathManualBreak::new();
        if let Some(aln_at) = read(attrs, "alnAt") {
            value = value.align_at(aln_at.parse().map_err(|_| ReaderError::XMLReadError)?);
        }
        loop {
            match r.next() {
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Break
                    {
                        return Ok(value);
                    }
                }
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
    fn reads_omath_run() {
        let xml = r#"<m:r>
  <m:rPr>
    <m:sty m:val="b" />
    <m:lit m:val="1" />
  </m:rPr>
  <m:t>x</m:t>
</m:r>
"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let _ = parser.next();
        let run = OMathRun::read(&mut parser, &[]).unwrap();
        assert_eq!(
            run,
            OMathRun::new()
                .style(OMathStyleType::Bold)
                .literal(true)
                .add_text("x")
        );
    }
}
