use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathFunctionProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathFunctionProperty {
    pub fn new(value: RunProperty) -> Self {
        Self {
            control_property: Some(OMathControlProperty::new(value)),
        }
    }
}

impl BuildXML for OMathFunctionProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_function_property()?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_function_property() {
        let xml = test_xml(&OMathFunctionProperty::new(RunProperty::new().bold()));
        assert_eq!(
            xml,
            r#"<m:funcPr><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:funcPr>"#
        );
    }

    case_test!(m_func_pr_within_m_func, "m:funcPr" within "m:func");
}
