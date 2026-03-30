use super::*;

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathEquationArrayProperty {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_justification: Option<OMathVerticalAlignmentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_distribution: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_distribution: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_spacing_rule: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_spacing: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_property: Option<OMathControlProperty>,
}

impl OMathEquationArrayProperty {
    pub fn base_justification(mut self, value: OMathVerticalAlignmentType) -> Self {
        self.base_justification = Some(value);
        self
    }

    pub fn max_distribution(mut self, value: bool) -> Self {
        self.max_distribution = Some(value);
        self
    }

    pub fn object_distribution(mut self, value: bool) -> Self {
        self.object_distribution = Some(value);
        self
    }

    pub fn row_spacing_rule(mut self, value: usize) -> Self {
        self.row_spacing_rule = Some(value);
        self
    }

    pub fn row_spacing(mut self, value: usize) -> Self {
        self.row_spacing = Some(value);
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.control_property = Some(OMathControlProperty::new(value));
        self
    }
}

impl BuildXML for OMathEquationArrayProperty {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_equation_array_property()?
            .apply_opt(self.base_justification.as_ref(), |value, b| {
                b.omath_char_tag("m:baseJc", &value.to_string())
            })?
            .apply_opt(self.max_distribution, |value, b| {
                b.omath_on_off("m:maxDist", value)
            })?
            .apply_opt(self.object_distribution, |value, b| {
                b.omath_on_off("m:objDist", value)
            })?
            .apply_opt(self.row_spacing_rule, |value, b| {
                b.omath_integer_value("m:rSpRule", value)
            })?
            .apply_opt(self.row_spacing, |value, b| {
                b.omath_integer_value("m:rSp", value)
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
    fn builds_equation_array_property() {
        let xml = test_xml(
            &OMathEquationArrayProperty::default()
                .base_justification(OMathVerticalAlignmentType::Center)
                .max_distribution(true)
                .object_distribution(true)
                .row_spacing_rule(2)
                .row_spacing(120),
        );
        assert_eq!(
            xml,
            r#"<m:eqArrPr><m:baseJc m:val="center" /><m:maxDist m:val="1" /><m:objDist m:val="1" /><m:rSpRule m:val="2" /><m:rSp m:val="120" /></m:eqArrPr>"#
        );
    }

    case_test!(m_eq_arr_pr_within_m_eq_arr, "m:eqArrPr" within "m:eqArr");
}
