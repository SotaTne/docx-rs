use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathFractionProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraction_type: Option<OMathFractionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathFractionProperty {
    pub fn fraction_type(mut self, value: OMathFractionType) -> Self {
        self.fraction_type = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathFractionProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_fraction_property()?
            .apply_opt(self.fraction_type.as_ref(), |value, b| {
                b.omath_fraction_type(&value.to_string())
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
    fn builds_fraction_property() {
        let xml = test_xml(
            &OMathFractionProperty::default()
                .fraction_type(OMathFractionType::Skewed)
                .control_property(RunProperty::new().bold()),
        );
        assert_eq!(
            xml,
            r#"<m:fPr><m:type m:val="skw" /><m:ctrlPr><w:rPr><w:b /><w:bCs /></w:rPr></m:ctrlPr></m:fPr>"#
        );
    }

    case_test!(m_f_pr_within_m_f, "m:fPr" within "m:f");
}
