use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathParaProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub justification: Option<OMathJustification>,
}

impl OMathParaProperty {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn justification(mut self, justification: OMathJustification) -> Self {
        self.justification = Some(justification);
        self
    }
}

impl BuildXML for OMathParaProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_para_property()?
            .add_optional_child(&self.justification)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_para_property() {
        let xml = test_xml(
            &OMathParaProperty::new()
                .justification(OMathJustification::new(OMathJustificationType::CenterGroup)),
        );
        assert_eq!(
            xml,
            r#"<m:oMathParaPr><m:jc m:val="centerGroup" /></m:oMathParaPr>"#
        );
    }

    case_test!(m_o_math_para_pr_within_m_o_math_para, "m:oMathParaPr" within "m:oMathPara");
}
