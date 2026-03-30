use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathLimitUpperProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathLimitUpperProperty {
    pub fn new(value: RunProperty) -> Self {
        Self {
            control_property: Some(OMathControlProperty::new(value)),
        }
    }
}

impl BuildXML for OMathLimitUpperProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_limit_upper_property()?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_limit_upper_property() {
        let xml = test_xml(&OMathLimitUpperProperty::new(RunProperty::new().italic()));
        assert_eq!(
            xml,
            r#"<m:limUppPr><m:ctrlPr><w:rPr><w:i /><w:iCs /></w:rPr></m:ctrlPr></m:limUppPr>"#
        );
    }

    case_test!(m_lim_upp_pr_within_m_lim_upp, "m:limUppPr" within "m:limUpp");
}
