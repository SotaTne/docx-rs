use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathPreSubSuperProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathPreSubSuperProperty {
    pub fn new(value: RunProperty) -> Self {
        Self {
            control_property: Some(OMathControlProperty::new(value)),
        }
    }
}

impl BuildXML for OMathPreSubSuperProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_pre_sub_super_property()?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_pre_sub_super_property() {
        let xml = test_xml(&OMathPreSubSuperProperty::new(RunProperty::new().bold()));
        assert_eq!(
            xml,
            r#"<m:sPrePr><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:sPrePr>"#
        );
    }

    case_test!(m_s_pre_pr_within_m_s_pre, "m:sPrePr" within "m:sPre");
}
