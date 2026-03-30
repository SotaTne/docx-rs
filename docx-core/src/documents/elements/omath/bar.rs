use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathBar {
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathBarProperty>,
}

impl OMathBar {
    pub fn new(base: OMathBase) -> Self {
        Self {
            base,
            property: None,
        }
    }

    pub fn position(mut self, value: OMathBarPositionType) -> Self {
        self.property = Some(self.property.unwrap_or_default().position(value));
        self
    }

    pub fn overline(base: OMathBase) -> Self {
        Self::new(base).position(OMathBarPositionType::Top)
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl BuildXML for OMathBar {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_bar()?
            .add_optional_child(&self.property)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_bar() {
        let xml = test_xml(
            &OMathBar::new(OMathBase::new().add_run(OMathRun::new().add_text("x")))
                .position(OMathBarPositionType::Bottom)
                .control_property(RunProperty::new().bold()),
        );
        assert_eq!(
            xml,
            r#"<m:bar><m:barPr><m:pos m:val="bot" /><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:barPr><m:e><m:r><m:t>x</m:t></m:r></m:e></m:bar>"#
        );
    }

    case_test!(
        m_bar_within_common_omath_parents,
        "m:bar" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
