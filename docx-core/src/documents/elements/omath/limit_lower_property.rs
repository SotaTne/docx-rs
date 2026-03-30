use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathLimitLowerProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathLimitLowerProperty {
    pub fn new(value: RunProperty) -> Self {
        Self {
            control_property: Some(OMathControlProperty::new(value)),
        }
    }
}

impl BuildXML for OMathLimitLowerProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_limit_lower_property()?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_limit_lower_property() {
        let xml = test_xml(&OMathLimitLowerProperty::new(RunProperty::new().bold()));
        assert_eq!(
            xml,
            r#"<m:limLowPr><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:limLowPr>"#
        );
    }

    case_test!(m_lim_low_pr_within_m_lim_low, "m:limLowPr" within "m:limLow");
}
