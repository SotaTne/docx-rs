use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathBarProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<OMathBarPositionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathBarProperty {
    pub fn position(mut self, value: OMathBarPositionType) -> Self {
        self.position = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathBarProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_bar_property()?
            .apply_opt(self.position.as_ref(), |value, b| {
                b.omath_char_tag("m:pos", &value.to_string())
            })?
            .add_optional_child(&self.control_property)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_bar_property() {
        let xml = test_xml(
            &OMathBarProperty::default()
                .position(OMathBarPositionType::Top)
                .control_property(RunProperty::new().italic()),
        );
        assert_eq!(
            xml,
            r#"<m:barPr><m:pos m:val="top" /><m:ctrlPr><w:rPr><w:i /><w:iCs /></w:rPr></m:ctrlPr></m:barPr>"#
        );
    }

    case_test!(m_bar_pr_within_m_bar, "m:barPr" within "m:bar");
}
