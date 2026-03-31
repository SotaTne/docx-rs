use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathPara {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut para = OMathPara::new();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::OMathParaPr => {
                            para.property = Some(OMathParaProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::OMath => {
                            para = para.add_math(OMath::read(r, &attributes)?);
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::OMathPara
                    {
                        return Ok(para);
                    }
                }
                Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
    }
}

impl ElementReader for OMathParaProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathParaProperty::new();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Justification
                    {
                        property =
                            property.justification(OMathJustification::read(r, &attributes)?);
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::OMathParaPr
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

impl ElementReader for OMathJustification {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let val = omath_val(attrs).ok_or(ReaderError::XMLReadError)?;
        let justification = OMathJustification::new(
            OMathJustificationType::from_str(&val).map_err(|_| ReaderError::XMLReadError)?,
        );

        loop {
            match r.next() {
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Justification
                    {
                        return Ok(justification);
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
    fn reads_omath_para() {
        let xml = r#"<m:oMathPara>
  <m:oMathParaPr>
    <m:jc m:val="center" />
  </m:oMathParaPr>
  <m:oMath>
    <m:r>
      <m:t>x</m:t>
    </m:r>
  </m:oMath>
</m:oMathPara>
"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let _ = parser.next();
        let para = OMathPara::read(&mut parser, &[]).unwrap();
        assert_eq!(
            para,
            OMathPara::new()
                .justification(OMathJustificationType::Center)
                .add_math(OMath::new().add_run(OMathRun::new().add_text("x")))
        );
    }

    #[test]
    fn reads_justification() {
        let xml = r#"<m:jc m:val="centerGroup" />"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let justification = loop {
            match parser.next().unwrap() {
                XmlEvent::StartElement {
                    name, attributes, ..
                } if name.local_name == "jc" => {
                    break OMathJustification::read(&mut parser, &attributes).unwrap();
                }
                _ => {}
            }
        };

        assert_eq!(
            justification,
            OMathJustification::new(OMathJustificationType::CenterGroup)
        );
    }
}
