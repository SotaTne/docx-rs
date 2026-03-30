use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathDelimiterProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grow: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<OMathShapeDelimiterType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathDelimiterProperty {
    pub fn begin_char(mut self, value: impl Into<String>) -> Self {
        self.begin_char = Some(value.into());
        self
    }

    pub fn end_char(mut self, value: impl Into<String>) -> Self {
        self.end_char = Some(value.into());
        self
    }

    pub fn separator_char(mut self, value: impl Into<String>) -> Self {
        self.separator_char = Some(value.into());
        self
    }

    pub fn grow(mut self, value: bool) -> Self {
        self.grow = Some(value);
        self
    }

    pub fn shape(mut self, value: OMathShapeDelimiterType) -> Self {
        self.shape = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathDelimiterProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_delimiter_property()?
            .apply_opt(self.begin_char.as_deref(), |value, b| {
                b.omath_char_tag("m:begChr", value)
            })?
            .apply_opt(self.separator_char.as_deref(), |value, b| {
                b.omath_char_tag("m:sepChr", value)
            })?
            .apply_opt(self.end_char.as_deref(), |value, b| {
                b.omath_char_tag("m:endChr", value)
            })?
            .apply_opt(self.grow, |value, b| b.omath_on_off("m:grow", value))?
            .apply_opt(self.shape.as_ref(), |value, b| {
                b.omath_char_tag("m:shp", &value.to_string())
            })?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_delimiter_property() {
        let xml = test_xml(
            &OMathDelimiterProperty::default()
                .begin_char("[")
                .separator_char("|")
                .end_char("]")
                .grow(true)
                .shape(OMathShapeDelimiterType::Centered),
        );
        assert_eq!(
            xml,
            r#"<m:dPr><m:begChr m:val="[" /><m:sepChr m:val="|" /><m:endChr m:val="]" /><m:grow m:val="1" /><m:shp m:val="centered" /></m:dPr>"#
        );
    }

    case_test!(m_d_pr_within_m_d, "m:dPr" within "m:d");
}
