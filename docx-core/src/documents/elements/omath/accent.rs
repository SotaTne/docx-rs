use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathAccent {
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathAccentProperty>,
}

impl OMathAccent {
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

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }

    pub fn vector(base: OMathBase) -> Self {
        Self::new(base).accent_char("\u{20d7}")
    }
}

impl BuildXML for OMathAccent {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_accent()?
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
    fn builds_accent() {
        let xml = test_xml(
            &OMathAccent::vector(OMathBase::new().add_run(OMathRun::new().add_text("x")))
                .control_property(RunProperty::new().bold()),
        );
        assert_eq!(
            xml,
            r#"<m:acc><m:accPr><m:chr m:val="⃗" /><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:accPr><m:e><m:r><m:t>x</m:t></m:r></m:e></m:acc>"#
        );
    }

    case_test!(
        m_acc_within_common_omath_parents,
        "m:acc" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
