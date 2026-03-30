use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathPara {
    pub children: Vec<OMath>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathParaProperty>,
}

impl OMathPara {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_math(mut self, math: OMath) -> Self {
        self.children.push(math);
        self
    }

    pub fn justification(mut self, value: OMathJustificationType) -> Self {
        self.property = Some(
            self.property
                .unwrap_or_default()
                .justification(OMathJustification::new(value)),
        );
        self
    }
}

impl BuildXML for OMathPara {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_para()?
            .add_optional_child(&self.property)?
            .add_children(&self.children)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_para() {
        let xml = test_xml(
            &OMathPara::new()
                .justification(OMathJustificationType::Right)
                .add_math(OMath::new().add_run(OMathRun::new().add_text("1+1=2"))),
        );
        assert_eq!(
            xml,
            r#"<m:oMathPara><m:oMathParaPr><m:jc m:val="right" /></m:oMathParaPr><m:oMath><m:r><m:t>1+1=2</m:t></m:r></m:oMath></m:oMathPara>"#
        );
    }

    case_test!(m_o_math_within_m_o_math_para, "m:oMath" within "m:oMathPara");
}
