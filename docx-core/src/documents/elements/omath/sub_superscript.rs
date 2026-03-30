use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathSubSuperscript {
    pub base: OMathBase,
    pub sub_argument: OMathSubArgument,
    pub super_argument: OMathSuperArgument,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathSubSuperscriptProperty>,
}

impl OMathSubSuperscript {
    pub fn new(
        base: OMathBase,
        sub_argument: OMathSubArgument,
        super_argument: OMathSuperArgument,
    ) -> Self {
        Self {
            base,
            sub_argument,
            super_argument,
            property: None,
        }
    }

    pub fn align_scripts(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().align_scripts(value));
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl BuildXML for OMathSubSuperscript {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_sub_superscript()?
            .add_optional_child(&self.property)?
            .add_child(&self.base)?
            .add_child(&self.sub_argument)?
            .add_child(&self.super_argument)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_sub_superscript() {
        let xml = test_xml(
            &OMathSubSuperscript::new(
                OMathBase::new().add_run(OMathRun::new().add_text("T")),
                OMathSubArgument::new().add_run(OMathRun::new().add_text("m")),
                OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
            )
            .align_scripts(true)
            .control_property(RunProperty::new().bold()),
        );
        assert_eq!(
            xml,
            r#"<m:sSubSup><m:sSubSupPr><m:alnScr m:val="1" /><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:sSubSupPr><m:e><m:r><m:t>T</m:t></m:r></m:e><m:sub><m:r><m:t>m</m:t></m:r></m:sub><m:sup><m:r><m:t>n</m:t></m:r></m:sup></m:sSubSup>"#
        );
    }

    case_test!(
        m_s_sub_sup_within_common_omath_parents,
        "m:sSubSup" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
