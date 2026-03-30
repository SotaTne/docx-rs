use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathNary {
    pub sub_argument: OMathSubArgument,
    pub super_argument: OMathSuperArgument,
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathNaryProperty>,
}

impl OMathNary {
    pub fn new(
        sub_argument: OMathSubArgument,
        super_argument: OMathSuperArgument,
        base: OMathBase,
    ) -> Self {
        Self {
            sub_argument,
            super_argument,
            base,
            property: None,
        }
    }

    pub fn operator_char(mut self, value: impl Into<String>) -> Self {
        self.property = Some(self.property.unwrap_or_default().operator_char(value));
        self
    }

    pub fn limit_location(mut self, value: OMathLimitLocationType) -> Self {
        self.property = Some(self.property.unwrap_or_default().limit_location(value));
        self
    }

    pub fn hide_sub_argument(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().hide_sub_argument(value));
        self
    }

    pub fn hide_super_argument(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().hide_super_argument(value));
        self
    }

    pub fn grow(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().grow(value));
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl BuildXML for OMathNary {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_nary()?
            .add_optional_child(&self.property)?
            .add_child(&self.sub_argument)?
            .add_child(&self.super_argument)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_nary() {
        let xml = test_xml(
            &OMathNary::new(
                OMathSubArgument::new().add_run(OMathRun::new().add_text("i=0")),
                OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                OMathBase::new().add_run(OMathRun::new().add_text("x")),
            )
            .operator_char("∑")
            .limit_location(OMathLimitLocationType::UnderOver)
            .grow(true),
        );
        assert_eq!(
            xml,
            r#"<m:nary><m:naryPr><m:chr m:val="∑" /><m:limLoc m:val="undOvr" /><m:grow m:val="1" /></m:naryPr><m:sub><m:r><m:t>i=0</m:t></m:r></m:sub><m:sup><m:r><m:t>n</m:t></m:r></m:sup><m:e><m:r><m:t>x</m:t></m:r></m:e></m:nary>"#
        );
    }

    case_test!(
        m_nary_within_common_omath_parents,
        "m:nary" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
