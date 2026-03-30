use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathFunctionName {
    pub children: Vec<OMathChild>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathArgumentProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathFunctionName {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            property: None,
            control_property: None,
        }
    }

    pub fn argument_size(mut self, value: i8) -> Self {
        self.property = Some(self.property.unwrap_or_default().argument_size(value));
        self
    }

    pub fn add_run(mut self, run: OMathRun) -> Self {
        self.children.push(OMathChild::Run(Box::new(run)));
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathFunctionName {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_function_name()?
            .add_optional_child(&self.property)?
            .add_children(&self.children)?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_function_name() {
        let xml = test_xml(
            &OMathFunctionName::new()
                .argument_size(1)
                .add_run(OMathRun::new().add_text("sin"))
                .control_property(RunProperty::new().italic()),
        );
        assert_eq!(
            xml,
            r#"<m:fName><m:argPr><m:argSz m:val="1" /></m:argPr><m:r><m:t>sin</m:t></m:r><m:ctrlPr><w:rPr><w:i /><w:iCs /></w:rPr></m:ctrlPr></m:fName>"#
        );
    }

    case_test!(m_f_name_within_m_func, "m:fName" within "m:func");
}
