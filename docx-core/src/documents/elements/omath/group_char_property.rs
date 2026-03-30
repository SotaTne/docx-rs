use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathGroupCharProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<OMathBarPositionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_justification: Option<OMathBarPositionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathGroupCharProperty {
    pub fn accent_char(mut self, value: impl Into<String>) -> Self {
        self.accent_char = Some(value.into());
        self
    }

    pub fn position(mut self, value: OMathBarPositionType) -> Self {
        self.position = Some(value);
        self
    }

    pub fn vertical_justification(mut self, value: OMathBarPositionType) -> Self {
        self.vertical_justification = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathGroupCharProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_group_char_property()?
            .apply_opt(self.accent_char.as_deref(), |value, b| {
                b.omath_operator_char(value)
            })?
            .apply_opt(self.position.as_ref(), |value, b| {
                b.omath_char_tag("m:pos", &value.to_string())
            })?
            .apply_opt(self.vertical_justification.as_ref(), |value, b| {
                b.omath_char_tag("m:vertJc", &value.to_string())
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
    fn builds_group_char_property() {
        let xml = test_xml(
            &OMathGroupCharProperty::default()
                .accent_char("⏟")
                .position(OMathBarPositionType::Bottom)
                .vertical_justification(OMathBarPositionType::Top)
                .control_property(RunProperty::new().bold()),
        );
        assert_eq!(
            xml,
            r#"<m:groupChrPr><m:chr m:val="⏟" /><m:pos m:val="bot" /><m:vertJc m:val="top" /><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:groupChrPr>"#
        );
    }

    case_test!(m_group_chr_pr_within_m_group_chr, "m:groupChrPr" within "m:groupChr");
}
