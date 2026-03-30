use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathJustification {
    pub value: OMathJustificationType,
}

impl OMathJustification {
    pub fn new(value: OMathJustificationType) -> Self {
        Self { value }
    }
}

impl BuildXML for OMathJustification {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .omath_justification(&self.value.to_string())?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_justification() {
        let xml = test_xml(&OMathJustification::new(
            OMathJustificationType::CenterGroup,
        ));
        assert_eq!(xml, r#"<m:jc m:val="centerGroup" />"#);
    }

    case_test!(m_jc_within_m_o_math_para_pr, "m:jc" within "m:oMathParaPr");
}
