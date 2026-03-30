use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathRadical {
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<OMathDegree>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathRadicalProperty>,
}

impl OMathRadical {
    pub fn new(base: OMathBase) -> Self {
        Self {
            base,
            degree: None,
            property: None,
        }
    }

    pub fn degree(mut self, degree: OMathDegree) -> Self {
        self.degree = Some(degree);
        self
    }

    pub fn hide_degree(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().hide_degree(value));
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl BuildXML for OMathRadical {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_radical()?
            .add_optional_child(&self.property)?
            .add_optional_child(&self.degree)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_radical() {
        let xml = test_xml(
            &OMathRadical::new(OMathBase::new().add_run(OMathRun::new().add_text("x")))
                .degree(OMathDegree::new().add_run(OMathRun::new().add_text("3")))
                .hide_degree(true),
        );
        assert_eq!(
            xml,
            r#"<m:rad><m:radPr><m:degHide m:val="1" /></m:radPr><m:deg><m:r><m:t>3</m:t></m:r></m:deg><m:e><m:r><m:t>x</m:t></m:r></m:e></m:rad>"#
        );
    }

    case_test!(
        m_rad_within_common_omath_parents,
        "m:rad" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
