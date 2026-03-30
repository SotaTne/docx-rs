use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathSuperscript {
    pub base: OMathBase,
    pub super_argument: OMathSuperArgument,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathSuperscriptProperty>,
}

impl OMathSuperscript {
    pub fn new(base: OMathBase, super_argument: OMathSuperArgument) -> Self {
        Self {
            base,
            super_argument,
            property: None,
        }
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(OMathSuperscriptProperty::new(value));
        self
    }
}

impl BuildXML for OMathSuperscript {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_superscript()?
            .add_optional_child(&self.property)?
            .add_child(&self.base)?
            .add_child(&self.super_argument)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_superscript() {
        let xml = test_xml(
            &OMathSuperscript::new(
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
                OMathSuperArgument::new().add_run(OMathRun::new().add_text("2")),
            )
            .control_property(RunProperty::new().bold()),
        );
        assert_eq!(
            xml,
            r#"<m:sSup><m:sSupPr><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:sSupPr><m:e><m:r><m:t>x</m:t></m:r></m:e><m:sup><m:r><m:t>2</m:t></m:r></m:sup></m:sSup>"#
        );
    }

    case_test!(
        m_s_sup_within_common_omath_parents,
        "m:sSup" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
