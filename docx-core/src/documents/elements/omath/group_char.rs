use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathGroupChar {
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathGroupCharProperty>,
}

impl OMathGroupChar {
    pub fn new(base: OMathBase) -> Self {
        Self {
            base,
            property: None,
        }
    }

    pub fn accent_char(mut self, value: impl Into<String>) -> Self {
        self.property = Some(self.property.unwrap_or_default().accent_char(value));
        self
    }

    pub fn position(mut self, value: OMathBarPositionType) -> Self {
        self.property = Some(self.property.unwrap_or_default().position(value));
        self
    }

    pub fn vertical_justification(mut self, value: OMathBarPositionType) -> Self {
        self.property = Some(
            self.property
                .unwrap_or_default()
                .vertical_justification(value),
        );
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }

    pub fn overbrace(base: OMathBase) -> Self {
        Self::new(base)
            .accent_char("\u{23de}")
            .position(OMathBarPositionType::Top)
    }

    pub fn underbrace(base: OMathBase) -> Self {
        Self::new(base)
            .accent_char("\u{23df}")
            .position(OMathBarPositionType::Bottom)
    }
}

impl BuildXML for OMathGroupChar {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_group_char()?
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
    fn builds_group_char() {
        let xml = test_xml(
            &OMathGroupChar::overbrace(OMathBase::new().add_run(OMathRun::new().add_text("a+b")))
                .vertical_justification(OMathBarPositionType::Bottom),
        );
        assert_eq!(
            xml,
            r#"<m:groupChr><m:groupChrPr><m:chr m:val="⏞" /><m:pos m:val="top" /><m:vertJc m:val="bot" /></m:groupChrPr><m:e><m:r><m:t>a+b</m:t></m:r></m:e></m:groupChr>"#
        );
    }

    case_test!(
        m_group_chr_within_common_omath_parents,
        "m:groupChr" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
