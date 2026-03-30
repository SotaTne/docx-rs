use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathAccentProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_char: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathAccentProperty {
    pub fn accent_char(mut self, value: impl Into<String>) -> Self {
        self.accent_char = Some(value.into());
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathAccentProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_accent_property()?
            .apply_opt(self.accent_char.as_deref(), |value, b| {
                b.omath_operator_char(value)
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
    fn builds_accent_property() {
        let xml = test_xml(
            &OMathAccentProperty::default()
                .accent_char("^")
                .control_property(RunProperty::new().italic()),
        );
        assert_eq!(
            xml,
            r#"<m:accPr><m:chr m:val="^" /><m:ctrlPr><w:rPr><w:i /><w:iCs /></w:rPr></m:ctrlPr></m:accPr>"#
        );
    }

    case_test!(m_acc_pr_within_m_acc, "m:accPr" within "m:acc");
}
