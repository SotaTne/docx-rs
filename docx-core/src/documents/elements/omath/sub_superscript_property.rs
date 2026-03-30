use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathSubSuperscriptProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align_scripts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathSubSuperscriptProperty {
    pub fn align_scripts(mut self, value: bool) -> Self {
        self.align_scripts = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathSubSuperscriptProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .write(XmlEvent::start_element("m:sSubSupPr"))?
            .apply_opt(self.align_scripts, |value, b| b.omath_align_scripts(value))?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_sub_superscript_property() {
        let xml = test_xml(
            &OMathSubSuperscriptProperty::default()
                .align_scripts(true)
                .control_property(RunProperty::new().italic()),
        );
        assert_eq!(
            xml,
            r#"<m:sSubSupPr><m:alnScr m:val="1" /><m:ctrlPr><w:rPr><w:i /><w:iCs /></w:rPr></m:ctrlPr></m:sSubSupPr>"#
        );
    }

    case_test!(m_s_sub_sup_pr_within_m_s_sub_sup, "m:sSubSupPr" within "m:sSubSup");
}
