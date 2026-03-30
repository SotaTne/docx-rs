use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathLimitUpper {
    pub base: OMathBase,
    pub limit: OMathLimit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathLimitUpperProperty>,
}

impl OMathLimitUpper {
    pub fn new(base: OMathBase, limit: OMathLimit) -> Self {
        Self {
            base,
            limit,
            property: None,
        }
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(OMathLimitUpperProperty::new(value));
        self
    }
}

impl BuildXML for OMathLimitUpper {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_limit_upper()?
            .add_optional_child(&self.property)?
            .add_child(&self.base)?
            .add_child(&self.limit)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_limit_upper() {
        let xml = test_xml(
            &OMathLimitUpper::new(
                OMathBase::new().add_run(OMathRun::new().add_text("lim")),
                OMathLimit::new().add_run(OMathRun::new().add_text("x→∞")),
            )
            .control_property(RunProperty::new().italic()),
        );
        assert_eq!(
            xml,
            r#"<m:limUpp><m:limUppPr><m:ctrlPr><w:rPr><w:i /><w:iCs /></w:rPr></m:ctrlPr></m:limUppPr><m:e><m:r><m:t>lim</m:t></m:r></m:e><m:lim><m:r><m:t>x→∞</m:t></m:r></m:lim></m:limUpp>"#
        );
    }

    case_test!(
        m_lim_upp_within_common_omath_parents,
        "m:limUpp" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
