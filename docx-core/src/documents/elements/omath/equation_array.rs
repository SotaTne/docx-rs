use super::*;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OMathEquationArray {
    pub elements: Vec<OMathBase>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<OMathEquationArrayProperty>,
}

impl OMathEquationArray {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
            property: None,
        }
    }

    pub fn add_element(mut self, element: OMathBase) -> Self {
        self.elements.push(element);
        self
    }

    pub fn base_justification(mut self, value: OMathVerticalAlignmentType) -> Self {
        self.property = Some(self.property.unwrap_or_default().base_justification(value));
        self
    }

    pub fn max_distribution(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().max_distribution(value));
        self
    }

    pub fn object_distribution(mut self, value: bool) -> Self {
        self.property = Some(self.property.unwrap_or_default().object_distribution(value));
        self
    }

    pub fn row_spacing_rule(mut self, value: usize) -> Self {
        self.property = Some(self.property.unwrap_or_default().row_spacing_rule(value));
        self
    }

    pub fn row_spacing(mut self, value: usize) -> Self {
        self.property = Some(self.property.unwrap_or_default().row_spacing(value));
        self
    }

    pub fn control_property(mut self, value: RunProperty) -> Self {
        self.property = Some(self.property.unwrap_or_default().control_property(value));
        self
    }
}

impl Default for OMathEquationArray {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildXML for OMathEquationArray {
    fn build_to<W: Write>(
        &self,
        stream: crate::xml::writer::EventWriter<W>,
    ) -> crate::xml::writer::Result<crate::xml::writer::EventWriter<W>> {
        XMLBuilder::from(stream)
            .open_omath_equation_array()?
            .add_child(&self.property.clone().unwrap_or_default())?
            .add_children(&self.elements)?
            .close()?
            .into_inner()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_equation_array() {
        let xml = test_xml(
            &OMathEquationArray::new()
                .object_distribution(true)
                .add_element(OMathBase::new().add_run(OMathRun::new().add_text("x=1")))
                .add_element(OMathBase::new().add_run(OMathRun::new().add_text("y=2"))),
        );
        assert_eq!(
            xml,
            r#"<m:eqArr><m:eqArrPr><m:objDist m:val="1" /></m:eqArrPr><m:e><m:r><m:t>x=1</m:t></m:r></m:e><m:e><m:r><m:t>y=2</m:t></m:r></m:e></m:eqArr>"#
        );
    }

    case_test!(
        m_eq_arr_within_common_omath_parents,
        "m:eqArr" within "m:oMath" | "m:e" | "m:num" | "m:den" | "m:deg" | "m:lim" | "m:sub" | "m:sup"
    );
}
