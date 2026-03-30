use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathLimitLower {
    pub base: OMathBase,
    pub limit: OMathLimit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathLimitLowerProperty>,
}

impl OMathLimitLower {
    pub fn new(base: OMathBase, limit: OMathLimit) -> Self {
        Self {
            base,
            limit,
            property: None,
        }
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(OMathLimitLowerProperty::new(value));
        self
    }
}

impl BuildXML for OMathLimitLower {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_limit_lower()?
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
    fn builds_limit_lower() {
        let xml = test_xml(
            &OMathLimitLower::new(
                OMathBase::new().add_run(OMathRun::new().add_text("lim")),
                OMathLimit::new().add_run(OMathRun::new().add_text("x→0")),
            )
            .control_property(RunProperty::new().bold()),
        );
        assert_eq!(
            xml,
            r#"<m:limLow><m:limLowPr><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:limLowPr><m:e><m:r><m:t>lim</m:t></m:r></m:e><m:lim><m:r><m:t>x→0</m:t></m:r></m:lim></m:limLow>"#
        );
    }

    case_test!(
        m_lim_low_within_common_omath_parents,
        "m:limLow" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
