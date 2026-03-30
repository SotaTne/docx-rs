use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathSubscript {
    pub base: OMathBase,
    pub sub_argument: OMathSubArgument,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathSubscriptProperty>,
}

impl OMathSubscript {
    pub fn new(base: OMathBase, sub_argument: OMathSubArgument) -> Self {
        Self {
            base,
            sub_argument,
            property: None,
        }
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(OMathSubscriptProperty::new(value));
        self
    }
}

impl BuildXML for OMathSubscript {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_subscript()?
            .add_optional_child(&self.property)?
            .add_child(&self.base)?
            .add_child(&self.sub_argument)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_subscript() {
        let xml = test_xml(
            &OMathSubscript::new(
                OMathBase::new().add_run(OMathRun::new().add_text("a")),
                OMathSubArgument::new().add_run(OMathRun::new().add_text("i")),
            )
            .control_property(RunProperty::new().italic()),
        );
        assert_eq!(
            xml,
            r#"<m:sSub><m:sSubPr><m:ctrlPr><w:rPr><w:i /><w:iCs /></w:rPr></m:ctrlPr></m:sSubPr><m:e><m:r><m:t>a</m:t></m:r></m:e><m:sub><m:r><m:t>i</m:t></m:r></m:sub></m:sSub>"#
        );
    }

    case_test!(
        m_s_sub_within_common_omath_parents,
        "m:sSub" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
