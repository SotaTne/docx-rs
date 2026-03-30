use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathFunction {
    pub name: OMathFunctionName,
    pub base: OMathBase,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathFunctionProperty>,
}

impl OMathFunction {
    pub fn new(name: OMathFunctionName, base: OMathBase) -> Self {
        Self {
            name,
            base,
            property: None,
        }
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(OMathFunctionProperty::new(value));
        self
    }
}

impl BuildXML for OMathFunction {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_function()?
            .add_child(&self.property.clone().unwrap_or_default())?
            .add_child(&self.name)?
            .add_child(&self.base)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_function() {
        let xml = test_xml(&OMathFunction::new(
            OMathFunctionName::new()
                .argument_size(1)
                .add_run(OMathRun::new().add_text("sin")),
            OMathBase::new().add_run(OMathRun::new().add_text("2")),
        ));
        assert_eq!(
            xml,
            r#"<m:func><m:funcPr /><m:fName><m:argPr><m:argSz m:val="1" /></m:argPr><m:r><m:t>sin</m:t></m:r></m:fName><m:e><m:r><m:t>2</m:t></m:r></m:e></m:func>"#
        );
    }

    case_test!(
        m_func_within_common_omath_parents,
        "m:func" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
