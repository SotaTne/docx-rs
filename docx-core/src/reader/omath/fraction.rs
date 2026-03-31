use std::io::Read;
use std::str::FromStr;

use super::*;

impl ElementReader for OMathFraction {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = None;
        let mut numerator = None;
        let mut denominator = None;
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::FractionProperty => {
                            property = Some(OMathFractionProperty::read(r, &attributes)?);
                        }
                        OMathXMLElement::Numerator => {
                            numerator = Some(OMathNumerator::read(r, &attributes)?);
                        }
                        OMathXMLElement::Denominator => {
                            denominator = Some(OMathDenominator::read(r, &attributes)?);
                        }
                        _ => return Err(ReaderError::XMLReadError),
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    if name.prefix.as_deref() == Some("m")
                        && OMathXMLElement::from_str(&name.local_name).unwrap()
                            == OMathXMLElement::Fraction
                    {
                        return Ok(OMathFraction {
                            numerator: numerator.ok_or(ReaderError::XMLReadError)?,
                            denominator: denominator.ok_or(ReaderError::XMLReadError)?,
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

impl ElementReader for OMathFractionProperty {
    fn read<R: Read>(
        r: &mut EventReader<R>,
        _attrs: &[OwnedAttribute],
    ) -> Result<Self, ReaderError> {
        let mut property = OMathFractionProperty::default();
        loop {
            match r.next() {
                Ok(XmlEvent::StartElement {
                    name, attributes, ..
                }) => {
                    if name.prefix.as_deref() != Some("m") {
                        continue;
                    }
                    match OMathXMLElement::from_str(&name.local_name).unwrap() {
                        OMathXMLElement::Type => {
                            property.fraction_type = Some(OMathFractionType::from_str(
                                &omath_val(&attributes).ok_or(ReaderError::XMLReadError)?,
                            )?);
                        }
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
                            == OMathXMLElement::FractionProperty
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
    fn reads_fraction() {
        let xml = r#"<m:f>
  <m:fPr>
    <m:type m:val="bar" />
  </m:fPr>
  <m:num>
    <m:r>
      <m:t>1</m:t>
    </m:r>
  </m:num>
  <m:den>
    <m:r>
      <m:t>2</m:t>
    </m:r>
  </m:den>
</m:f>
"#;
        let mut parser = EventReader::new(xml.as_bytes());
        let _ = parser.next();
        let fraction = OMathFraction::read(&mut parser, &[]).unwrap();
        assert_eq!(
            fraction,
            OMathFraction::new(
                OMathNumerator::new().add_run(OMathRun::new().add_text("1")),
                OMathDenominator::new().add_run(OMathRun::new().add_text("2"))
            )
            .fraction_type(OMathFractionType::Bar)
        );
    }
}
