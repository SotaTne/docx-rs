use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathSubscript {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut base = None;
        let mut sub_argument = None;
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::SubscriptProperty => {
                            property = Some(OMathSubscriptProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::Base => {
                            base = Some(OMathBase::read(r, &attributes)?);
                        }
                        OMathXMLElement::SubArgument => {
                            sub_argument = Some(OMathSubArgument::read(r, &attributes)?);
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Subscript
                    {
                        return Ok(OMathSubscript {
                            base: base.ok_or(ReaderError::XMLReadError)?,
                            sub_argument: sub_argument.ok_or(ReaderError::XMLReadError)?,
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

impl ElementReader for OMathSubscriptProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathSubscriptProperty::default();
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
                            == OMathXMLElement::SubscriptProperty
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
    fn reads_subscript() {
        let xml = r#"<m:sSub>
  <m:sSubPr>
    <m:ctrlPr>
      <w:rPr>
        <w:i />
      </w:rPr>
    </m:ctrlPr>
  </m:sSubPr>
  <m:e>
    <m:r>
      <m:t>a</m:t>
    </m:r>
  </m:e>
  <m:sub>
    <m:r>
      <m:t>i</m:t>
    </m:r>
  </m:sub>
</m:sSub>
"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let _ = parser.next();
        let subscript = OMathSubscript::read(&mut parser, &[]).unwrap();
        assert_eq!(
            subscript,
            OMathSubscript::new(
                OMathBase::new().add_run(OMathRun::new().add_text("a")),
                OMathSubArgument::new().add_run(OMathRun::new().add_text("i")),
            )
            .control_property(RunProperty::new().italic())
        );
    }
}
