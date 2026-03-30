use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathBoxProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_emulator: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_break: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_break: Option<OMathManualBreak>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathBoxProperty {
    pub fn operator_emulator(mut self, value: bool) -> Self {
        self.operator_emulator = Some(value);
        self
    }

    pub fn no_break(mut self, value: bool) -> Self {
        self.no_break = Some(value);
        self
    }

    pub fn differential(mut self, value: bool) -> Self {
        self.differential = Some(value);
        self
    }

    pub fn manual_break(mut self, value: OMathManualBreak) -> Self {
        self.manual_break = Some(value);
        self
    }

    pub fn align(mut self, value: bool) -> Self {
        self.align = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathBoxProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_box_property()?
            .apply_opt(self.operator_emulator, |value, b| {
                b.omath_on_off("m:opEmu", value)
            })?
            .apply_opt(self.no_break, |value, b| b.omath_on_off("m:noBreak", value))?
            .apply_opt(self.differential, |value, b| {
                b.omath_on_off("m:diff", value)
            })?
            .add_optional_child(&self.manual_break)?
            .apply_opt(self.align, |value, b| b.omath_on_off("m:aln", value))?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_box_property() {
        let xml = test_xml(
            &OMathBoxProperty::default()
                .operator_emulator(true)
                .no_break(true)
                .differential(true)
                .manual_break(OMathManualBreak::new().align_at(2))
                .align(true),
        );
        assert_eq!(
            xml,
            r#"<m:boxPr><m:opEmu m:val="1" /><m:noBreak m:val="1" /><m:diff m:val="1" /><m:brk m:alnAt="2" /><m:aln m:val="1" /></m:boxPr>"#
        );
    }

    case_test!(m_box_pr_within_m_box, "m:boxPr" within "m:box");
}
