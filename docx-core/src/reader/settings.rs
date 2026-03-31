use super::*;
use crate::reader::{FromXML, ReaderError};
use crate::types::{
    CharacterSpacingValues, OMathBreakBinaryOperatorType, OMathBreakBinarySubtractionType,
    OMathJustificationType, OMathLimitLocationType,
};
use std::io::Read;
use std::str::FromStr;

impl FromXML for Settings {
    fn from_xml<R: Read>(reader: R) -> Result<Self, ReaderError> {
        let mut parser = EventReader::new(reader);
        let mut settings = Self::default();

        loop {
            let e = parser.next();
            match e {
                Ok(XmlEvent::StartElement {
                    attributes, name, ..
                }) => {
                    if name.prefix.as_deref() == Some("m") && name.local_name == "mathPr" {
                        settings = settings.math_properties(read_math_properties(&mut parser)?);
                        continue;
                    }

                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    match e {
                        XMLElement::DocId => {
                            for a in attributes {
                                if let Some(prefix) = a.name.prefix {
                                    let local_name = &a.name.local_name;
                                    // Ignore w14:val
                                    if local_name == "val" && prefix == "w15" {
                                        settings = settings
                                            .doc_id(a.value.to_owned().replace(['{', '}'], ""));
                                    }
                                }
                            }
                        }
                        XMLElement::DocVar => {
                            let name = attributes::read_name(&attributes);
                            let val = attributes::read_val(&attributes);
                            if let Some(name) = name {
                                if let Some(val) = val {
                                    settings = settings.add_doc_var(name, val);
                                }
                            }
                        }
                        XMLElement::DefaultTabStop => {
                            let val = attributes::read_val(&attributes);
                            if let Some(val) = val {
                                if let Ok(val) = f32::from_str(&val) {
                                    settings = settings.default_tab_stop(val as usize);
                                }
                            }
                        }
                        XMLElement::EvenAndOddHeaders => {
                            let val = attributes::read_bool(&attributes);
                            if val {
                                settings = settings.even_and_odd_headers();
                            }
                        }
                        XMLElement::AdjustLineHeightInTable => {
                            settings = settings.adjust_line_height_in_table();
                        }
                        XMLElement::CharacterSpacingControl => {
                            let val = read_val(&attributes);
                            if let Some(val) = val {
                                if let Ok(v) = CharacterSpacingValues::from_str(&val) {
                                    settings = settings.character_spacing_control(v);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(XmlEvent::EndElement { name, .. }) => {
                    let e = XMLElement::from_str(&name.local_name).unwrap();
                    if let XMLElement::Settings = e {
                        break;
                    }
                }
                Ok(XmlEvent::EndDocument { .. }) => break,
                Err(_) => return Err(ReaderError::XMLReadError),
                _ => {}
            }
        }
        Ok(settings)
    }
}

fn read_math_properties<R: Read>(
    parser: &mut EventReader<R>,
) -> Result<MathProperties, ReaderError> {
    let mut math_properties = MathProperties::new();

    loop {
        match parser.next() {
            Ok(XmlEvent::StartElement {
                name, attributes, ..
            }) => {
                if name.prefix.as_deref() != Some("m") {
                    continue;
                }

                match name.local_name.as_str() {
                    "mathFont" => {
                        let val = read_val(&attributes).ok_or(ReaderError::XMLReadError)?;
                        math_properties = math_properties.math_font(val);
                    }
                    "brkBin" => {
                        let val = read_val(&attributes).ok_or(ReaderError::XMLReadError)?;
                        let val = OMathBreakBinaryOperatorType::from_str(&val)
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.break_binary(val);
                    }
                    "brkBinSub" => {
                        let val = read_val(&attributes).ok_or(ReaderError::XMLReadError)?;
                        let val = OMathBreakBinarySubtractionType::from_str(&val)
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.break_binary_subtraction(val);
                    }
                    "smallFrac" => {
                        math_properties = math_properties.small_fraction(read_bool(&attributes));
                    }
                    "dispDef" => {
                        math_properties = math_properties.display_defaults(read_bool(&attributes));
                    }
                    "lMargin" => {
                        let val = read_val(&attributes)
                            .ok_or(ReaderError::XMLReadError)?
                            .parse()
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.left_margin(val);
                    }
                    "rMargin" => {
                        let val = read_val(&attributes)
                            .ok_or(ReaderError::XMLReadError)?
                            .parse()
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.right_margin(val);
                    }
                    "defJc" => {
                        let val = read_val(&attributes).ok_or(ReaderError::XMLReadError)?;
                        let val = OMathJustificationType::from_str(&val)
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.default_justification(val);
                    }
                    "preSp" => {
                        let val = read_val(&attributes)
                            .ok_or(ReaderError::XMLReadError)?
                            .parse()
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.pre_spacing(val);
                    }
                    "postSp" => {
                        let val = read_val(&attributes)
                            .ok_or(ReaderError::XMLReadError)?
                            .parse()
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.post_spacing(val);
                    }
                    "interSp" => {
                        let val = read_val(&attributes)
                            .ok_or(ReaderError::XMLReadError)?
                            .parse()
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.inter_spacing(val);
                    }
                    "intraSp" => {
                        let val = read_val(&attributes)
                            .ok_or(ReaderError::XMLReadError)?
                            .parse()
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.intra_spacing(val);
                    }
                    "wrapIndent" => {
                        let val = read_val(&attributes)
                            .ok_or(ReaderError::XMLReadError)?
                            .parse()
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.wrap_indent(val);
                    }
                    "wrapRight" => {
                        math_properties = math_properties.wrap_right(read_bool(&attributes));
                    }
                    "intLim" => {
                        let val = read_val(&attributes).ok_or(ReaderError::XMLReadError)?;
                        let val = OMathLimitLocationType::from_str(&val)
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.integral_limit_location(val);
                    }
                    "naryLim" => {
                        let val = read_val(&attributes).ok_or(ReaderError::XMLReadError)?;
                        let val = OMathLimitLocationType::from_str(&val)
                            .map_err(|_| ReaderError::XMLReadError)?;
                        math_properties = math_properties.nary_limit_location(val);
                    }
                    _ => return Err(ReaderError::XMLReadError),
                }
            }
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.prefix.as_deref() == Some("m") && name.local_name == "mathPr" {
                    return Ok(math_properties);
                }
            }
            Ok(XmlEvent::Whitespace(_)) | Ok(XmlEvent::Characters(_)) => {}
            Err(_) => return Err(ReaderError::XMLReadError),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn reads_settings_math_properties() {
        let xml = br#"
<w:settings>
  <w:defaultTabStop w:val="840" />
  <m:mathPr>
    <m:mathFont m:val="Cambria Math" />
    <m:brkBin m:val="repeat" />
    <m:brkBinSub m:val="-+" />
    <m:smallFrac m:val="1" />
    <m:dispDef />
    <m:lMargin m:val="0" />
    <m:rMargin m:val="0" />
    <m:defJc m:val="centerGroup" />
    <m:preSp m:val="120" />
    <m:postSp m:val="120" />
    <m:interSp m:val="60" />
    <m:intraSp m:val="30" />
    <m:wrapIndent m:val="1440" />
    <m:wrapRight m:val="1" />
    <m:intLim m:val="subSup" />
    <m:naryLim m:val="undOvr" />
  </m:mathPr>
</w:settings>
"#;

        let settings = Settings::from_xml(&xml[..]).unwrap();

        assert_eq!(
            settings,
            Settings::new().default_tab_stop(840).math_properties(
                MathProperties::new()
                    .math_font("Cambria Math")
                    .break_binary(OMathBreakBinaryOperatorType::Repeat)
                    .break_binary_subtraction(OMathBreakBinarySubtractionType::MinusPlus)
                    .small_fraction(true)
                    .display_defaults(true)
                    .left_margin(0)
                    .right_margin(0)
                    .default_justification(OMathJustificationType::CenterGroup)
                    .pre_spacing(120)
                    .post_spacing(120)
                    .inter_spacing(60)
                    .intra_spacing(30)
                    .wrap_indent(1440)
                    .wrap_right(true)
                    .integral_limit_location(OMathLimitLocationType::SubscriptSuperscript)
                    .nary_limit_location(OMathLimitLocationType::UnderOver)
            )
        );
    }
}
