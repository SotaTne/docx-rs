use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathBox {
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathBoxProperty>,
}

impl OMathBox {
    pub fn new(base: OMathBase) -> Self {
        Self {
            base,
            property: None,
        }
    }

    pub fn operator_emulator(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().operator_emulator(value));
        self
    }

    pub fn no_break(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().no_break(value));
        self
    }

    pub fn differential(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().differential(value));
        self
    }

    pub fn align(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().align(value));
        self
    }

    pub fn manual_break(mut self, value: OMathManualBreak) -> Self {
        self.property = Some(self.property.unwrap_or_default().manual_break(value));
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl BuildXML for OMathBox {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        let property = self.property.clone().unwrap_or_default();
        XMLBuilder::from(stream)
            .open_omath_box()?
            .add_child(&property)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_box() {
        let xml = test_xml(
            &OMathBox::new(OMathBase::new().add_run(OMathRun::new().add_text("x+1")))
                .operator_emulator(true)
                .no_break(true)
                .differential(true)
                .manual_break(OMathManualBreak::new().align_at(2))
                .align(true),
        );
        assert_eq!(
            xml,
            r#"<m:box><m:boxPr><m:opEmu m:val="1" /><m:noBreak m:val="1" /><m:diff m:val="1" /><m:brk m:alnAt="2" /><m:aln m:val="1" /></m:boxPr><m:e><m:r><m:t>x+1</m:t></m:r></m:e></m:box>"#
        );
    }

    case_test!(
        m_box_within_common_omath_parents,
        "m:box" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
