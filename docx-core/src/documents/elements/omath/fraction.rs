use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathFraction {
    pub numerator: OMathNumerator,
    pub denominator: OMathDenominator,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathFractionProperty>,
}

impl OMathFraction {
    pub fn new(numerator: OMathNumerator, denominator: OMathDenominator) -> Self {
        Self {
            numerator,
            denominator,
            property: None,
        }
    }

    pub fn fraction_type(mut self, value: OMathFractionType) -> Self {
        self.property = Some(self.property.unwrap_or_default().fraction_type(value));
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl BuildXML for OMathFraction {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_fraction()?
            .add_optional_child(&self.property)?
            .add_child(&self.numerator)?
            .add_child(&self.denominator)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_fraction() {
        let xml = test_xml(
            &OMathFraction::new(
                OMathNumerator::new().add_run(OMathRun::new().add_text("1")),
                OMathDenominator::new().add_run(OMathRun::new().add_text("2")),
            )
            .fraction_type(OMathFractionType::Bar),
        );
        assert_eq!(
            xml,
            r#"<m:f><m:fPr><m:type m:val="bar" /></m:fPr><m:num><m:r><m:t>1</m:t></m:r></m:num><m:den><m:r><m:t>2</m:t></m:r></m:den></m:f>"#
        );
    }

    case_test!(
        m_f_within_common_omath_parents,
        "m:f" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
