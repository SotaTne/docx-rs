use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathSuperscript {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut base = None;
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
                        OMathXMLElement::SuperscriptProperty => {
                            property = Some(OMathSuperscriptProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::Base => {
                            base = Some(OMathBase::read(r, &attributes)?);
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
                            == OMathXMLElement::Superscript
                    {
                        return Ok(OMathSuperscript {
                            base: base.ok_or(ReaderError::XMLReadError)?,
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

impl ElementReader for OMathSuperscriptProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathSuperscriptProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::ControlProperty => {
                            property.control_property =
                                Some(OMathControlProperty::read(r, &attributes)?);
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::SuperscriptProperty
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn reads_superscript() {
        let xml = r#"<m:sSup>
  <m:sSupPr>
    <m:ctrlPr>
      <w:rPr>
        <w:b />
      </w:rPr>
    </m:ctrlPr>
  </m:sSupPr>
  <m:e>
    <m:r>
      <m:t>x</m:t>
    </m:r>
  </m:e>
  <m:sup>
    <m:r>
      <m:t>2</m:t>
    </m:r>
  </m:sup>
</m:sSup>
"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let _ = parser.next();
        let superscript = OMathSuperscript::read(&mut parser, &[]).unwrap();
        assert_eq!(
            superscript,
            OMathSuperscript::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
                OMathSuperArgument::new().add_run(OMathRun::new().add_text("2")),
            )
            .control_property(RunProperty::new().bold())
        );
    }
}
