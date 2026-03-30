use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathPreSubSuper {
    pub sub_argument: OMathSubArgument,
    pub super_argument: OMathSuperArgument,
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathPreSubSuperProperty>,
}

impl OMathPreSubSuper {
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

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(OMathPreSubSuperProperty::new(value));
        self
    }
}

impl BuildXML for OMathPreSubSuper {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_pre_sub_super()?
            .add_child(&self.property.clone().unwrap_or_default())?
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
    fn builds_pre_sub_super() {
        let xml = test_xml(
            &OMathPreSubSuper::new(
                OMathSubArgument::new().add_run(OMathRun::new().add_text("m")),
                OMathSuperArgument::new().add_run(OMathRun::new().add_text("n")),
                OMathBase::new().add_run(OMathRun::new().add_text("T")),
            )
            .control_property(RunProperty::new().bold()),
        );
        assert_eq!(
            xml,
            r#"<m:sPre><m:sPrePr><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:sPrePr><m:sub><m:r><m:t>m</m:t></m:r></m:sub><m:sup><m:r><m:t>n</m:t></m:r></m:sup><m:e><m:r><m:t>T</m:t></m:r></m:e></m:sPre>"#
        );
    }

    case_test!(
        m_s_pre_within_common_omath_parents,
        "m:sPre" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
